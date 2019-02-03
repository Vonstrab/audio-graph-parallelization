//! Parse a fileformat describing audiographs
use pest::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use itertools::Itertools;

use audiograph::AudioGraph;
use audiograph_edge::AGEdge;
use audiograph_node::AGNode;

use petgraph::graph::NodeIndex;
use petgraph::Graph;

#[derive(Debug)]
pub struct Edge {
    source_id: String,
    destination_id: String,
}

#[derive(Parser)]
#[grammar = "audiograph.pest"]
pub struct AudiographParser;

pub fn parse_audiograph(audiograph: &str) -> AudioGraph {
    let parse_result =
        AudiographParser::parse(Rule::file, audiograph).unwrap_or_else(|e| panic!("{}", e));

    let mut audio_nodes: Vec<AGNode> = Vec::new();
    let edges: Vec<Edge> = Vec::new();
    let mut audio_edges: Vec<(usize, usize)> = Vec::new();

    for file in parse_result {
        let statements = file.into_inner();

        for statement in statements {
            match statement.as_rule() {
                Rule::node => {
                    let fields = statement.into_inner();
                    let mut node = AGNode::new();

                    for field in fields {
                        if field.as_rule() == Rule::ident {
                            node.id = field.as_str().to_string();
                        }
                        if field.as_rule() == Rule::attribute {
                            let mut attribute = field.as_str().split(|c| (c == ':') || (c == ','));

                            let id = attribute.next().unwrap();
                            let mut v = attribute.next().unwrap();
                            v = v.trim();

                            match id {
                                "in" => node.nb_inlets = v.parse::<i64>().unwrap(),
                                "out" => node.nb_outlets = v.parse::<i64>().unwrap(),
                                "text" => node.text = Some(v.to_string()),
                                "kind" => node.object_name = v.to_string(),
                                "wcet" => node.wcet = Some(v.parse().unwrap()),
                                _ => {
                                    //Add more field
                                }
                            }
                        }
                    }
                    audio_nodes.push(node);
                }
                Rule::edges => {
                    println!("ICI 1");
                    println!("ICI 2 {:?}", statement.clone().into_inner().as_str());
                    let mut fields = statement.into_inner().tuples();

                    let (src_id_r, _src_forget) = fields.next().unwrap();
                    let mut src_id = src_id_r.as_str().to_string();

                    println!("ICI 3 {:?}", src_id);
                    println!("ICI 4 {:?}", _src_forget.as_str());

                    let mut edges = Vec::new();

                    for (mut dst_id_r, source) in fields {
                        let dst_id = dst_id_r.as_str().to_string();
                        edges.push(Edge {
                            source_id: src_id,
                            destination_id: dst_id.clone(),
                        });
                        src_id = dst_id;
                    }
                }
                Rule::deadline => {
                    //A ajouter
                }
                _ => {
                    //DEBUG
                    println!("KO 1 : {:?}", statement.as_rule());
                }
            }
        }
    }

    let mut audio_graph = Graph::<AGNode, AGEdge>::with_capacity(audio_nodes.len(), edges.len());
    let mut node_refs: Vec<NodeIndex<u32>> = Vec::with_capacity(audio_nodes.len());

    for node in audio_nodes.clone() {
        node_refs.push(audio_graph.add_node(node));
    }

    println!(" edges taille {}", edges.len());
    //find the coresponding indices
    for edge in &edges {
        println!("\n source {}", edge.source_id);
        println!(" target {}", edge.destination_id);

        let mut source = 0;
        let mut target = 0;
        let mut i = 0;
        for node in audio_nodes.clone() {
            println!(" present node{}", node.id);

            if edge.source_id == node.id {
                source = i;
                println!("la");
            }
            if edge.destination_id == node.id {
                println!("ici",);

                target = i;
            }
            i = i + 1;
        }
        audio_edges.push((source, target));
    }

    println!("{}", audio_edges.len());

    let mut ag_edges: Vec<(NodeIndex<u32>, NodeIndex<u32>)> = Vec::with_capacity(edges.len());

    for (source, target) in audio_edges {
        ag_edges.push((node_refs[source], node_refs[target]));
    }

    audio_graph.extend_with_edges(ag_edges);

    // DEBUG
    println!(
        "{:?}",
        petgraph::dot::Dot::with_config(&audio_graph, &[petgraph::dot::Config::EdgeNoLabel])
    );

    AudioGraph::new(audio_graph)
}

pub fn parse_audiograph_from_file(filename: &str) -> AudioGraph {
    let path = Path::new(filename);
    let mut file = File::open(&path).expect("Impossible to open file.");
    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("Impossible to read file.");
    parse_audiograph(&s)
}
