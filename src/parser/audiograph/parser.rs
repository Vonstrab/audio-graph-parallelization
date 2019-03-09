//! Parse files containing audiographs

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use task_graph::graph;
use task_graph::task::Task;

use pest::error::Error as ParseError;
use pest::Parser;

#[derive(Debug)]
pub struct Edge {
    src_id: String,
    src_port: u32,
    dst_id: String,
    dst_port: u32,
}

#[derive(Parser)]
#[grammar = "parser/audiograph/audiograph.pest"]
pub struct AudiographParser;

pub fn parse_audiograph(audiograph: &str) -> Result<graph::TaskGraph, ParseError<Rule>> {
    let audiograph = AudiographParser::parse(Rule::file, audiograph)?
        .next()
        .unwrap();

    use pest::iterators::*;

    fn parse_node(pair: Pair<Rule>) -> Task {
        let mut inner_rules = pair.into_inner();
        let id: String;
        let mut nb_inlets: u32 = 0;
        let mut nb_outlets: u32 = 0;
        let mut class_name: String = String::default();
        let mut text: Option<String> = None;
        let mut wcet: Option<f64> = None;
        let mut more: HashMap<String, String> = HashMap::new();
        let mut volume: f32 = 0.0;

        id = inner_rules.next().unwrap().as_str().to_string();

        //Attributes
        for attribute in inner_rules {
            let mut attr = attribute.into_inner();
            let token = attr.next().unwrap().as_str().trim_matches('\"');
            let v = attr.next().unwrap().as_str().trim_matches('\"');
            match token {
                "in" => nb_inlets = v.parse().unwrap(),
                "out" => nb_outlets = v.parse().unwrap(),
                "text" => text = Some(v.to_string()),
                "kind" => class_name = v.to_string(),
                "wcet" => wcet = Some(v.parse().unwrap()),
                "volume" => volume = v.parse().unwrap(),
                _ => {
                    more.insert(token.to_string(), v.to_string());
                }
            }
        }

        Task::Audiograph {
            id,
            nb_inlets,
            nb_outlets,
            class_name,
            text,
            wcet,
            more,
            volume,
        }
    }

    use std::vec::IntoIter;

    fn parse_edge(pair: Pair<Rule>) -> IntoIter<Edge> {
        let mut inner_rules = pair.into_inner();
        let mut port_ident = inner_rules.next().unwrap().into_inner();
        let mut src_id = port_ident.next().unwrap().as_str().to_string();
        let mut src_port = port_ident.next().unwrap().as_str().parse().unwrap();

        let mut edges = Vec::new();

        for inner_rule in inner_rules {
            port_ident = inner_rule.into_inner().next().unwrap().into_inner();
            let dst_id = port_ident.next().unwrap().as_str().to_string();
            let dst_port = port_ident.next().unwrap().as_str().parse().unwrap();
            edges.push(Edge {
                src_id,
                src_port,
                dst_id: dst_id.clone(),
                dst_port,
            });
            src_id = dst_id;
            src_port = dst_port;
        }
        edges.into_iter()
    }

    //let to_print = audiograph.clone().into_inner().flat_map()

    let (nodes, edges): (Vec<_>, Vec<_>) = audiograph
        .into_inner()
        .flat_map(|r| r.into_inner())
        .filter(|ref r| r.as_rule() != Rule::deadline)
        //.inspect(|x| println!("Statement: {:?}.", x))
        .partition(|ref r| r.as_rule() == Rule::node);

    let nodes = nodes.into_iter().map(parse_node).collect::<Vec<_>>();
    let edges = edges.into_iter().flat_map(parse_edge).collect::<Vec<_>>();
    let mut node_indexes: HashMap<String, usize> = HashMap::new();

    let mut taskgraph = graph::TaskGraph::new(nodes.len(), edges.len());

    for task in nodes.into_iter() {
        let task_id;

        match &task {
            Task::Audiograph { id, .. } => task_id = id.clone(),
            _ => panic!("Not an Audiograph"),
        }

        let node_index = taskgraph.add_task(&task);
        node_indexes.insert(task_id, node_index);
    }

    for edge in edges.iter() {
        let src_node = node_indexes[&edge.src_id];
        let dst_node = node_indexes[&edge.dst_id];
        taskgraph.add_edge(src_node, dst_node);
    }

    Ok(taskgraph)
}

pub fn parse(filename: &str) -> Result<graph::TaskGraph, ParseError<Rule>> {
    let path = Path::new(filename);
    let mut file = File::open(&path).expect("Impossible to open file.");
    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("Impossible to read file.");
    parse_audiograph(&s)
}
