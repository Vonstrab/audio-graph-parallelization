//! Parse a fileformat describing audiographs

use audio_node::AudioNode;

use pest::Parser;

use petgraph::graph::NodeIndex;
use petgraph::Graph;

#[derive(Parser)]
#[grammar = "audiograph.pest"]
pub struct AudiographParser;

pub fn audiograph_from_pd(audiograph: &str) -> Graph<AudioNode, ()> {
    let parse_result =
        AudiographParser::parse(Rule::file, audiograph).unwrap_or_else(|e| panic!("{}", e));

    //let mut nodes: Vec<String> = Vec::new();
    let mut audio_nodes: Vec<AudioNode> = Vec::new();

    let mut edges: Vec<(usize, usize)> = Vec::new();

    for file in parse_result {
        let defs = file.into_inner();

        for def in defs {
            match def.as_rule() {
                Rule::OBJ => {
                    let fields = def.into_inner();

                    //let mut node = String::new();
                    let mut node = AudioNode::new();

                    let mut posx : i64 =-1;
                    let mut posy : i64 =-1;

                    for field in fields {
                        if field.as_rule() == Rule::ID {
                            //node.push_str(field.as_str());
                            node.set_object(field.as_str().to_string());
                        }
                        if field.as_rule() == Rule::POSX {
                            posx = field.as_str().parse::<i64>().unwrap();
                        }
                        if field.as_rule() == Rule::POSY {
                            posy = field.as_str().parse::<i64>().unwrap();
                        }
                        if field.as_rule() == Rule::ID {
                            //node.push_str(field.as_str());
                            node.set_object(field.as_str().to_string());
                        }

                        if field.as_rule() == Rule::AOBJ {
                            //node.push(' ');
                            //node.push_str(field.as_str());
                            for aobj in field.as_str().split_whitespace(){
                                node.add_arg(aobj.to_string());
                            }
                        }
                    }

                    //nodes.push(node);
                    node.set_pos(posx, posy);
                    audio_nodes.push(node);
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


    //let mut audio_graph = Graph::<String, ()>::with_capacity(nodes.len(), edges.len());
    let mut audio_graph = Graph::<AudioNode, ()>::with_capacity(audio_nodes.len(), edges.len());
    let mut node_refs: Vec<NodeIndex<u32>> = Vec::with_capacity(audio_nodes.len());

    for node in audio_nodes {
        node_refs.push(audio_graph.add_node(node));
    }

    let mut ag_edges: Vec<(NodeIndex<u32>, NodeIndex<u32>)> = Vec::with_capacity(edges.len());

    for (source, target) in edges {
        ag_edges.push((node_refs[source], node_refs[target]));
    }

    audio_graph.extend_with_edges(ag_edges);


    // DEBUG
    println!(
        "{:?}",
        petgraph::dot::Dot::with_config(&audio_graph, &[petgraph::dot::Config::EdgeNoLabel])
    );

    audio_graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_audiograph_test() {
        //TODO
    }

}
