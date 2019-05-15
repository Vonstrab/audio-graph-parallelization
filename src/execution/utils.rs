use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::dsp::DspEdge;
use crate::task_graph::graph::TaskGraph;

pub fn build_dsp_edges(
    graph: &TaskGraph,
    client: &jack::Client,
) -> HashMap<(usize, usize), Arc<RwLock<DspEdge>>> {
    let g_edges = graph.get_edges();
    let mut edges = HashMap::with_capacity(g_edges.len());

    for (src, dst) in g_edges.keys() {
        let buff = Arc::new(RwLock::new(DspEdge::new(
            client.buffer_size() as usize,
            client.sample_rate(),
        )));

        edges.insert((*src, *dst), buff);
    }

    edges
}
