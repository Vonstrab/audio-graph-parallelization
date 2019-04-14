use std::sync::{Arc, RwLock};

use crate::dsp::DspNode;
use crate::execution::build_dsp_edges;
use crate::task_graph::graph::TaskGraph;
use crate::task_graph::state::TaskState;

use super::thread_pool::ThreadPool;

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

pub fn run_work_stealing(graph: Arc<RwLock<TaskGraph>>) -> Result<(), jack::Error> {
    let (client, _) = jack::Client::new(
        "audio_graph_sequential",
        jack::ClientOptions::NO_START_SERVER,
    )?;

    let nb_exit_nodes = graph.write().unwrap().get_exit_nodes().len();
    let mut out_ports = Vec::with_capacity(nb_exit_nodes);

    for i in 0..nb_exit_nodes {
        let mut out_port =
            client.register_port(&format!("port_{}", i), jack::AudioOut::default())?;

        out_ports.push(out_port);
    }

    let dsp_edges = Arc::new(RwLock::new(build_dsp_edges(
        &*graph.read().unwrap(),
        &client,
    )));

    let thread_pool = Arc::new(RwLock::new(ThreadPool::create(
        4,
        graph.clone(),
        dsp_edges.clone(),
    )));

    let callback = jack::ClosureProcessHandler::new(clone!(thread_pool => move |_, ps| {
        // We must give new buffers for the sinks to write into, every time this callback
        // function is called by jack
        let exit_nodes = graph.write().unwrap().get_exit_nodes();
        for (i, &node_index) in exit_nodes.iter().enumerate() {
            let buffer = out_ports[i].as_mut_slice(ps);
            let frames = ps.n_frames();

            let sink = graph.read().unwrap().get_dsp(node_index);
            let sink = &mut *sink.lock().unwrap();

            if let Some(sink) = sink {
                if let DspNode::Sink(ref mut s) = sink.dsp {
                    s.set_buffer(buffer.as_mut_ptr(), frames);
                }
            }
        }

        // We must set the activation counters of each node
        let nb_nodes = graph.read().unwrap().get_nb_node();
        for node_index in 0..nb_nodes {
            let predecessors = graph.read().unwrap().get_predecessors(node_index);
            if let Some(predecessors) = predecessors {
                let activation_count = predecessors.len();

                graph.write().unwrap().set_state(node_index, if activation_count == 0 {
                    TaskState::Ready
                } else {
                    TaskState::WaitingDependencies(activation_count)
                });
            }
        }

        thread_pool.write().unwrap().start();

        jack::Control::Continue
    }));

    let _active_client = client.activate_async((), callback)?;

    let mut user_input = String::new();
    let _ignored = std::io::stdin().read_line(&mut user_input);

    thread_pool.read().unwrap().stop();

    Ok(())
}
