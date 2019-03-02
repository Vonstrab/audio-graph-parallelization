//! Parse a fileformat describing audiographs

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use task_graph::graph;
use task_graph::task::Task;

use pest::Parser;


#[derive(Parser)]
#[grammar = "parser/puredata/puredata.pest"]
pub struct PuredataParser;

pub fn parse_puredata(puredata: &str) -> graph::TaskGraph {
    let parse_result =
        PuredataParser::parse(Rule::file, puredata).unwrap_or_else(|e| panic!("{}", e));

    let mut nb_nodes =0 ;
    let mut tasks: Vec<Task> = Vec::new();
    let mut edges: Vec<(usize, usize)> = Vec::new();

    for file in parse_result {
        let defs = file.into_inner();

        for def in defs {
            match def.as_rule() {
                Rule::OBJ => {
                    let fields = def.into_inner();


                    let mut posx: i64 = -1;
                    let mut posy: i64 = -1;
                    let mut id = String::default();
                    let mut args :Vec<String> =  Vec::new();


                    for field in fields {
                        if field.as_rule() == Rule::ID {
                            id= field.as_str().to_string();
                        }
                        if field.as_rule() == Rule::POSX {
                            posx = field.as_str().parse::<i64>().unwrap();
                        }
                        if field.as_rule() == Rule::POSY {
                            posy = field.as_str().parse::<i64>().unwrap();
                        }
                        if field.as_rule() == Rule::AOBJ {
                            for aobj in field.as_str().split_whitespace() {
                                args.push(aobj.to_string());
                            }
                        }
                    }

                    let mut task =Task::Puredata{
                        object_name:id,
                        xpos: posx,
                        ypos: posy,
                        args:args,
                    }; 
                    tasks.push(task);
                    nb_nodes += 1;
                }
                Rule::CON => {
                    let fields = def.into_inner();

                    let mut source = 0;
                    let mut target = 0;

                    for field in fields {
                        if field.as_rule() == Rule::SOURCE {
                            source = field.as_str().parse().unwrap();
                        }

                        if field.as_rule() == Rule::TARGET {
                            target = field.as_str().parse().unwrap();
                        }
                    }

                    edges.push((source, target));
                }
                Rule::MSG => {
                    let fields = def.into_inner();

                    let mut posx: i64 = -1;
                    let mut posy: i64 = -1;
                    let mut id = String::default();
                    let mut args :Vec<String> =  Vec::new();

                    for field in fields {
                        if field.as_rule() == Rule::STRING {
                            id = "msg".to_string();
                            args.push(field.to_string());
                        }
                        if field.as_rule() == Rule::POSX {
                            posx = field.as_str().parse::<i64>().unwrap();
                        }
                        if field.as_rule() == Rule::POSY {
                            posy = field.as_str().parse::<i64>().unwrap();
                        }
                    }


                    let mut task =Task::Puredata{
                        object_name:id,
                        xpos: posx,
                        ypos: posy,
                        args:args,
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
        graph_out.add_task(& tasks[i]);
    }

    for (source, target) in edges {
        graph_out.add_edge(source, target);
    }

    graph_out
}

pub fn parse(filename: &str) -> graph::TaskGraph {
    let path = Path::new(filename);
    let mut file = File::open(&path).expect("Impossible to open file.");
    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("Impossible to read file.");
    parse_puredata(&s)
}

#[cfg(test)]
mod tests {

    use puredata::parser::*;

    #[test]
    fn parse_test_aleatoire() {
        let graphe_test = parse_puredata_from_file("./Samples/PD/aleatoire.pd");
        assert_eq!(graphe_test.nb_nodes(), 9);
        assert_eq!(graphe_test.nb_edges(), 8);
    }

    #[test]
    fn parse_test_aleatoire2() {
        let graphe_test = parse_puredata_from_file("./Samples/PD/aleatoire2.pd");
        assert_eq!(graphe_test.nb_nodes(), 14);
        assert_eq!(graphe_test.nb_edges(), 15);
    }

    #[test]
    fn parse_test_aleatoire4() {
        let graphe_test = parse_puredata_from_file("./Samples/PD/aleatoire4.pd");
        assert_eq!(graphe_test.nb_nodes(), 24);
        assert_eq!(graphe_test.nb_edges(), 27);
    }

    #[test]
    fn parse_test_tonalite() {
        let graphe_test = parse_puredata_from_file("./Samples/PD/Tonalite.pd");
        assert_eq!(graphe_test.nb_nodes(), 12);
        assert_eq!(graphe_test.nb_edges(), 15);
    }
    #[test]
    fn parse_test_metronome() {
        let graphe_test = parse_puredata_from_file("./Samples/PD/Metronome.pd");
        assert_eq!(graphe_test.nb_nodes(), 386);
        assert_eq!(graphe_test.nb_edges(), 396);
    }
}
