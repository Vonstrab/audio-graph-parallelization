use petgraph::graph::NodeIndex;
use petgraph::Graph;

use audio_node::AudioNode;

/// Represents an audiograph of nodes
pub struct AudioGraph {
    graph: Graph<AudioNode, ()>,
}

impl AudioGraph {
    pub fn new(graph: Graph<AudioNode, ()>) -> AudioGraph {
        AudioGraph { graph: graph }
    }

    pub fn nb_nodes(&self) -> usize {
        self.graph.node_count()
    }

    pub fn nb_edges(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn add_node(&mut self, node: AudioNode) -> NodeIndex {
        self.graph.add_node(node)
    }
}
