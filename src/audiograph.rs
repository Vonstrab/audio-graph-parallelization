use petgraph::graph::NodeIndex;
use petgraph::Graph;

use audiograph_edge::AGEdge;
use audiograph_node::AGNode;

use std::fmt;

/// Represents an audiograph of nodes
pub struct AudioGraph {
    graph: Graph<AGNode, AGEdge>,

    buffer_size: u32, // Used for computing computation and communications costs
}

impl AudioGraph {
    pub fn new(graph: Graph<AGNode, AGEdge>) -> AudioGraph {
        AudioGraph {
            graph,
            buffer_size: 512, // Default value used for now. Will be changed later.
        }
    }

    pub fn nb_nodes(&self) -> usize {
        self.graph.node_count()
    }

    pub fn nb_edges(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn add_node(&mut self, node: AGNode) -> NodeIndex {
        self.graph.add_node(node)
    }
}
impl fmt::Display for AudioGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dot_graph =
            petgraph::dot::Dot::with_config(&self.graph, &[petgraph::dot::Config::EdgeNoLabel]);
        write!(f, "(\n{}\nbuffer_size = {})", dot_graph, self.buffer_size)
    }
}
