use std::sync::{Arc, Mutex, RwLock};

use crossbeam::channel::Sender;

use crate::dsp::DspNode;
use crate::measure::MeasureDestination;
use crate::task_graph::graph::TaskGraph;

use super::utils::build_dsp_edges;

/// Sequentially executes an audio graph with JACK.
///
/// # Arguments
///
/// * `graph` - The audio graph to be executed
/// * `tx` - The channel used for sending statistical measurements
pub fn run_seq(
    graph: Arc<Mutex<TaskGraph>>,
    tx: Sender<MeasureDestination>,
) -> Result<(), jack::Error> {
    tx.send(MeasureDestination::File(
        "tmp/seq_log.txt".to_string(),
        "Beginning of the execution".to_string(),
    ))
    .expect("logging error");

    let (client, _) = jack::Client::new(
        "audio_graph_sequential",
        jack::ClientOptions::NO_START_SERVER,
    )?;

    graph.lock().unwrap().set_sample_rate(client.sample_rate());
    graph
        .lock()
        .unwrap()
        .set_buffer_size(client.buffer_size() as usize);

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

    // Allocate the audio buffers used by the DSPs of the audio graph
    let dsp_edges = Arc::new(Mutex::new(build_dsp_edges(
        &*graph.lock().unwrap(),
        &client,
    )));

    // Get the sequential scheduling of the audio graph
    let exec_order = Arc::new(RwLock::new(graph.lock().unwrap().get_topological_order()));

    // The audio callback funtion
    let callback = jack::ClosureProcessHandler::new(clone!(dsp_edges => move | _ , ps | {
        // Save the time at which the function started its execution
        let start_time = std::time::SystemTime::now();
        tx.send(MeasureDestination::File(
            "tmp/seq_log.txt".to_string(),
            format!("\nBeginning of a cycle at: {:#?}", start_time),
        ))
        .expect("logging error");

        let graph = &mut *graph.lock().unwrap();
        let dsp_edges = &mut *dsp_edges.lock().unwrap();

        // We must give new buffers for the sinks to write into,
        // every time this callback function is called by JACK
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

        // Get the time spent for the execution of the audio graph
        let elapsed_time = start_time.elapsed().unwrap();
        let time_left = ps.cycle_times().unwrap().next_usecs - jack::get_time();

        tx.send(MeasureDestination::File(
            "tmp/seq_log.txt".to_string(),
            format!(
                "\nEnd of cycle at: {:#?} \nIn: {}µs\nTime left before the deadline: {}µs",
                start_time,
                elapsed_time.as_micros(),
                time_left,
            ),
        ))
        .expect("logging error");

        // JACK will continue to call this function
        jack::Control::Continue
    }));

    // Tell JACK to start calling the callback function
    let _active_client = client.activate_async((), callback)?;

    // Wait for an input from the user in order to not immediately exit
    // the program
    let mut user_input = String::new();
    let _ignored = std::io::stdin().read_line(&mut user_input);

    Ok(())
}
