use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

use crossbeam::channel::Sender;

use crate::dsp::{DspEdge, DspNode};
use crate::measure::MeasureDestination;
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

pub fn run_seq(
    graph: Arc<Mutex<TaskGraph>>,
    tx: Sender<MeasureDestination>,
) -> Result<(), jack::Error> {
    tx.send(MeasureDestination::File(
        "tmp/seq_log.txt".to_string(),
        format!("Beginning of the execution"),
    ))
    .expect("logging error");

    let (client, _) = jack::Client::new(
        "audio_graph_sequential",
        jack::ClientOptions::NO_START_SERVER,
    )?;

    let nb_exit_nodes = graph.lock().unwrap().get_exit_nodes().len();

    tx.send(MeasureDestination::File(
        "tmp/seq_log.txt".to_string(),
        format!("Number of exit nodes: {}", nb_exit_nodes),
    ))
    .expect("logging error");

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

    // Get the sequential scheduling of the audio graph
    let exec_order = Arc::new(RwLock::new(graph.lock().unwrap().get_topological_order()));

    let callback = jack::ClosureProcessHandler::new(clone!(dsp_edges => move | _ , ps | {
        let start_time = std::time::SystemTime::now();
        tx.send(MeasureDestination::File(
            "tmp/seq_log.txt".to_string(),
            format!("\nBeginning of a cycle at: {:#?}", start_time),
        ))
        .expect("logging error");

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


        // The execution of the audio graph happens here
        for &node_index in exec_order.read().unwrap().iter() {
            let predecessors = graph.get_predecessors(node_index);
            let successors = graph.get_successors(node_index);

            if let (Some(predecessors), Some(successors)) = (predecessors, successors) {
                let in_edges: Vec<_> = predecessors
                    .iter()
                    .map(|&src| {
                        dsp_edges
                            .get(&(src, node_index))
                            .unwrap()
                            .clone()
                    })
                    .collect();

                let out_edges: Vec<_> = successors
                    .iter()
                    .map(|&dst| {
                        dsp_edges.get(&(node_index, dst)).unwrap().clone()
                    })
                    .collect();

                let task = graph.get_dsp(node_index);
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
            }
        }

        tx.send(MeasureDestination::File(
            "tmp/seq_log.txt".to_string(),
            format!(
                "\nEnd of cycle at: {:#?} \nIn: {}ms \n{}µs",
                start_time,
                start_time.elapsed().unwrap().subsec_millis(),
                start_time.elapsed().unwrap().subsec_nanos(),
            ),
        ))
        .expect("logging error");

        let time_left = ps.cycle_times().unwrap().next_usecs - jack::get_time();

        tx.send(MeasureDestination::File(
            "tmp/seq_log.txt".to_string(),
            format!("\nTime left before the deadline: {}µs ", time_left)
        ))
        .expect("logging error");

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
