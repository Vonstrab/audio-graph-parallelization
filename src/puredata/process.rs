//! Process a graph determining node duration time

use audiograph;
use audiograph_edge;
use audiograph_node;

pub fn process_node(ref node: &audiograph_node::AGNode) -> f64 {
    match node.object_name.as_str() {
        "msg" => 0.0,
        _ => 0.0,
    }
}
