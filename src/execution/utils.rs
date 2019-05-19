use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crossbeam::deque::Worker;

use crate::dsp::{DspEdge, DspNode};
use crate::task_graph::graph::TaskGraph;
use crate::task_graph::state::TaskState;

// Make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
                move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
                move |$(clone!(@param $p),)+| $body
        }
    );
}

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

pub fn exec_task(
    node_index: usize,
    task_graph: Arc<RwLock<TaskGraph>>,
    dsp_edges: Arc<RwLock<HashMap<(usize, usize), Arc<RwLock<DspEdge>>>>>,
    worker_queue: Option<&Worker<usize>>,
) {
    let predecessors = task_graph.read().unwrap().get_predecessors(node_index);
    let successors = task_graph.read().unwrap().get_successors(node_index);

    if let (Some(predecessors), Some(successors)) = (predecessors, successors) {
        let in_edges: Vec<_> = predecessors
            .iter()
            .map(|&src| {
                dsp_edges
                    .read()
                    .unwrap()
                    .get(&(src, node_index))
                    .unwrap()
                    .clone()
            })
            .collect();
        let out_edges: Vec<_> = successors
            .iter()
            .map(|&dst| {
                dsp_edges
                    .read()
                    .unwrap()
                    .get(&(node_index, dst))
                    .unwrap()
                    .clone()
            })
            .collect();

        let task = task_graph.read().unwrap().get_dsp(node_index);
        let task = &mut *task.lock().unwrap();

        if let Some(task) = task {
            match task.dsp {
                DspNode::Oscillator(ref mut o) => o.process(out_edges[0].clone()),
                DspNode::Modulator(ref mut m) => {
                    m.process(in_edges[0].clone(), out_edges[0].clone())
                }
                DspNode::InputsOutputsAdaptor(ref mut ioa) => ioa.process(in_edges, out_edges),
                DspNode::Sink(ref mut s) => s.process(in_edges[0].clone()),
            }
        }

        task_graph
            .write()
            .unwrap()
            .set_state(node_index, TaskState::Completed);

        for succ in successors {
            task_graph.write().unwrap().dec_activation_count(succ);

            if let Some(worker_queue) = worker_queue {
                if task_graph.read().unwrap().get_state(succ) == Some(TaskState::Ready) {
                    worker_queue.push(succ);
                }
            }
        }
    }
}
