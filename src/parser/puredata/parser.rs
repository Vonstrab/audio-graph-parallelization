//! Parse a fileformat describing audiographs

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use pest::error::Error as ParseError;
use pest::Parser;

use task_graph::graph;
use task_graph::task::Task;

#[derive(Parser)]
#[grammar = "parser/puredata/puredata.pest"]
pub struct PuredataParser;

pub fn parse_puredata(puredata: &str) -> Result<graph::TaskGraph, ParseError<Rule>> {
    let parse_result =
        PuredataParser::parse(Rule::file, puredata).unwrap_or_else(|e| panic!("{}", e));

    let mut nb_nodes = 0;
    let mut tasks: Vec<Task> = Vec::new();
    let mut edges: Vec<(usize, usize)> = Vec::new();

    for file in parse_result {
        let defs = file.into_inner();

        for def in defs {
            match def.as_rule() {
                Rule::OBJ => {
                    let fields = def.into_inner();

                    let mut xpos: i64 = -1;
                    let mut ypos: i64 = -1;
                    let mut object_name = String::default();
                    let mut args: Vec<String> = Vec::new();

                    for field in fields {
                        match field.as_rule() {
                            Rule::ID => object_name = field.as_str().to_string(),
                            Rule::POSX => xpos = field.as_str().parse::<i64>().unwrap(),
                            Rule::POSY => ypos = field.as_str().parse::<i64>().unwrap(),
                            Rule::AOBJ => {
                                for aobj in field.as_str().split_whitespace() {
                                    args.push(aobj.to_string());
                                }
                            }
                            _ => {}
                        }
                    }

                    let mut task = Task::Puredata {
                        object_name,
                        xpos,
                        ypos,
                        args,
                    };
                    tasks.push(task);
                    nb_nodes += 1;
                }
                Rule::CON => {
                    let fields = def.into_inner();

                    let mut source = 0;
                    let mut target = 0;

                    for field in fields {
                        match field.as_rule() {
                            Rule::SOURCE => source = field.as_str().parse().unwrap(),
                            Rule::TARGET => target = field.as_str().parse().unwrap(),
                            _ => {}
                        }
                    }

                    edges.push((source, target));
                }
                Rule::MSG => {
                    let fields = def.into_inner();

                    let mut xpos: i64 = -1;
                    let mut ypos: i64 = -1;
                    let mut object_name = String::default();
                    let mut args: Vec<String> = Vec::new();

                    for field in fields {
                        match field.as_rule() {
                            Rule::STRING => {
                                object_name = "msg".to_string();
                                args.push(field.to_string());
                            }
                            Rule::POSX => xpos = field.as_str().parse::<i64>().unwrap(),
                            Rule::POSY => ypos = field.as_str().parse::<i64>().unwrap(),
                            _ => {}
                        }
                    }

                    let mut task = Task::Puredata {
                        object_name,
                        xpos,
                        ypos,
                        args,
                    };
                    tasks.push(task);
                    nb_nodes += 1;
                }

                _ => {}
            }
        }
    }

    let mut graph_out = graph::TaskGraph::new(nb_nodes, edges.len());

    for i in 0..tasks.len() {
        graph_out.add_task(&tasks[i]);
    }

    for (source, target) in edges {
        graph_out.add_edge(source, target);
    }

    Ok(graph_out)
}

pub fn parse(filename: &str) -> Result<graph::TaskGraph, ParseError<Rule>> {
    let path = Path::new(filename);
    let mut file = File::open(&path).expect("Impossible to open file.");
    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("Impossible to read file.");
    parse_puredata(&s)
}
