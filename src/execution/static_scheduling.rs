use std::sync::{Arc, RwLock};

use crossbeam::channel::Sender;

use crate::dsp::DspNode;
use crate::measure::MeasureDestination;
use crate::static_scheduling::algorithms::{schedule, SchedulingAlgorithm};
use crate::task_graph::graph::TaskGraph;
use crate::task_graph::state::TaskState;

use super::thread_pool::static_scheduling::ThreadPool;
use super::utils::build_dsp_edges;

/// Makes a parallel execution, with a static scheduling, of an audio graph
/// with JACK.
///
/// # Arguments
///
/// * `graph` - The audio graph to be executed
/// * `nb_threads` - The number of threads used for the execution
/// * `sched_algo` - The algorithm to use for making the static scheduling
/// * `tx` - The channel used for sending statistical measurements
pub fn run_static_sched(
    graph: Arc<RwLock<TaskGraph>>,
    nb_threads: usize,
    sched_algo: SchedulingAlgorithm,
    tx: Sender<MeasureDestination>,
) -> Result<(), jack::Error> {
    // Select the file to write into according to the scheduling algorithm
    let output_file = String::from(match sched_algo {
        SchedulingAlgorithm::Random => "tmp/static_rand_sched_log.txt",
        SchedulingAlgorithm::HLFET => "tmp/static_hlfet_sched_log.txt",
        SchedulingAlgorithm::ETF => "tmp/static_etf_sched_log.txt",
    });

    tx.send(MeasureDestination::File(
        output_file.clone(),
        format!("Beginning of the execution"),
    ))
    .expect("logging error");

    let (client, _) = jack::Client::new(
        "audio_graph_static_sched",
        jack::ClientOptions::NO_START_SERVER,
    )?;

    graph.write().unwrap().set_sample_rate(client.sample_rate());
    graph
        .write()
        .unwrap()
        .set_buffer_size(client.buffer_size() as usize);

    let nb_exit_nodes = graph.write().unwrap().get_exit_nodes().len();

    tx.send(MeasureDestination::File(
        output_file.clone(),
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
    let dsp_edges = Arc::new(RwLock::new(build_dsp_edges(
        &*graph.read().unwrap(),
        &client,
    )));

    // Get the static scheduling of the audio graph
    let sched = schedule(&mut graph.write().unwrap(), nb_threads, sched_algo);

    // Create the thread pool with the appropriate number of threads
    let thread_pool = Arc::new(RwLock::new(ThreadPool::create(
        nb_threads,
        graph.clone(),
        dsp_edges.clone(),
        sched,
    )));

    // The audio callback funtion
    let callback = jack::ClosureProcessHandler::new(clone!(thread_pool => move |_, ps| {
        // Save the time at which the function started its execution
        let start_time = std::time::SystemTime::now();
        tx.send(MeasureDestination::File(
            output_file.clone(),
            format!("\nBeginning of a cycle at: {:#?}", start_time),
        ))
        .expect("logging error");

        // We must give new buffers for the sinks to write into,
        // every time this callback function is called by JACK
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

        // We must reset the activation counters of each node
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

        // Execute the audio graph with the thread pool
        thread_pool.write().unwrap().start();

        // Get the time spent for the execution of the audio graph
        let elapsed_time = start_time.elapsed().unwrap();
        let time_left = ps.cycle_times().unwrap().next_usecs - jack::get_time();

        tx.send(MeasureDestination::File(
            output_file.clone(),
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
