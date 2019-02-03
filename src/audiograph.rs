use petgraph::graph::NodeIndex;
use petgraph::Graph;

use audiograph_node::AGNode;

/// Represents an audiograph of nodes
pub struct AudioGraph {
    graph: Graph<AGNode, ()>,

    buffer_size: u32, // Used for computing computation and communications costs
}

impl AudioGraph {
    pub fn new(graph: Graph<AGNode, ()>) -> AudioGraph {
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
