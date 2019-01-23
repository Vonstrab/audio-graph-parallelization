//! Parse a fileformat describing audiographs

use std::fs::File;
use std::io::*;

use pest::Parser;

#[derive(Parser)]
#[grammar = "audiograph.pest"]
pub struct AudiographParser;

pub fn parse_audiograph(audiograph: &str) -> bool {
    let parse_result =
        AudiographParser::parse(Rule::file, audiograph).unwrap_or_else(|e| panic!("{}", e));

    let mut nodes: Vec<String> = Vec::new();
    let mut edges: Vec<(i32, i32)> = Vec::new();

    for file in parse_result {
        let defs = file.into_inner();

        for def in defs {
            match def.as_rule() {
                Rule::OBJ => {
                    let fields = def.into_inner();

                    let mut node = String::new();

                    for field in fields {
                        if field.as_rule() == Rule::ID {
                            node.push_str(field.as_str());
                        }

                        if field.as_rule() == Rule::AOBJ {
                            node.push(' ');
                            node.push_str(field.as_str());
                        }
                    }

                    nodes.push(node);
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
                _ => {}
            }
        }
    }

    // DEBUG
    for node in nodes {
        println!("{}", node)
    }

    // DEBUG
    for edge in edges {
        println!("Source: {}", edge.0);
        println!("Target: {}", edge.1);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_audiograph_test() {
        //TODO
    }

}
