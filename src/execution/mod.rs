use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

use crate::dsp::{DspEdge, DspNode};
use crate::task_graph::graph::TaskGraph;

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

pub fn run_seq(graph: Arc<Mutex<TaskGraph>>) -> Result<(), jack::Error> {
    let (client, _) = jack::Client::new(
        "audio_graph_sequential",
        jack::ClientOptions::NO_START_SERVER,
    )?;

    let nb_exit_nodes = graph.lock().unwrap().get_exit_nodes().len();
    let mut out_ports = Vec::with_capacity(nb_exit_nodes);

    for i in 0..nb_exit_nodes {
        let mut out_port =
            client.register_port(&format!("port_{}", i), jack::AudioOut::default())?;

        out_ports.push(out_port);
    }

    let dsp_edges = Arc::new(Mutex::new(build_dsp_edges(
        &*graph.lock().unwrap(),
        &client,
    )));

    let callback = jack::ClosureProcessHandler::new(clone!(dsp_edges => move |_, ps| {
        let graph = &mut *graph.lock().unwrap();
        let dsp_edges = &mut *dsp_edges.lock().unwrap();

        // We must give new buffers for the sinks to write into, every time this callback
        // function is called by jack
        for (i, &node_index) in graph.get_exit_nodes().iter().enumerate() {
            let buffer = out_ports[i].as_mut_slice(ps);
            let frames = ps.n_frames();

            let sink = graph.get_dsp(node_index);
            let sink = &mut *sink.lock().unwrap();

            if let Some(sink) = sink {
                if let DspNode::Sink(ref mut s) = sink.dsp {
                    s.set_buffer(buffer.as_mut_ptr(), frames);
                }
            }
        }

        // Get the sequential scheduling of the audio graph
        let exec_order = graph.get_topological_order();

        // The execution of the audio graph happens here
        for node_index in exec_order {
            if let (Some(predecessors), Some(successors)) = (graph.get_predecessors(node_index), graph.get_successors(node_index)) {
                let in_edges: Vec<_> = predecessors.iter().map(|&src| dsp_edges.get(&(src, node_index)).unwrap().clone()).collect();

                let out_edges: Vec<_> = successors.iter().map(|&dst| dsp_edges.get(&(node_index, dst)).unwrap().clone()).collect();

                let task = graph.get_dsp(node_index);
                let task = &mut *task.lock().unwrap();

                if let Some(task) = task {
                    match task.dsp {
                        DspNode::Oscillator(ref mut o) => o.process(out_edges[0].clone()),
                        DspNode::Modulator(ref mut m) => m.process(in_edges[0].clone(), out_edges[0].clone()),
                        DspNode::InputsOutputsAdaptor(ref mut ioa) => ioa.process(in_edges, out_edges),
                        DspNode::Sink(ref mut s) => s.process(in_edges[0].clone()),
                    }
                }
            }
        }

        jack::Control::Continue
    }));

    let _active_client = client.activate_async((), callback)?;

    let mut user_input = String::new();
    let _ignored = std::io::stdin().read_line(&mut user_input);

    Ok(())
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
