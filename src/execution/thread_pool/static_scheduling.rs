use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;

use crossbeam::channel::{unbounded, Receiver, Sender};
use crossbeam::utils::Backoff;

use crate::dsp::DspEdge;
use crate::execution::utils::exec_task;
use crate::static_scheduling::schedule::Schedule;
use crate::task_graph::graph::TaskGraph;
use crate::task_graph::state::TaskState;

#[derive(Clone, Copy, PartialEq)]
enum CtrlMsg {
    Start, // Used for telling a worker to start the execution
    Reset, // Used for preventing deadlocks when a new cycle starts while some workers are not finished
}

#[derive(Clone, Copy)]
enum FeedbackMsg {
    Done, // Message sent by a worker when it has completed all its tasks
}

pub struct ThreadPool {
    ctrl_chans: Vec<Sender<CtrlMsg>>,
    fb_chans: Vec<Receiver<FeedbackMsg>>,
}

impl ThreadPool {
    /// Creates a thread pool for executing a static scheduling of an audio
    /// graph.
    ///
    /// # Arguments
    ///
    /// * `threads_count` - The number of threads of the pool
    /// * `task_graph` - The audio graph to be executed by the thread pool
    /// * `dsp_edges` - The buffers of the graph
    /// * `sched` - The static scheduling to be followed by the threads
    pub fn create(
        threads_count: usize,
        task_graph: Arc<RwLock<TaskGraph>>,
        dsp_edges: Arc<RwLock<HashMap<(usize, usize), Arc<RwLock<DspEdge>>>>>,
        sched: Schedule,
    ) -> ThreadPool {
        let core_ids = core_affinity::get_core_ids().expect("Failed to get core IDs.");
        let mut ctrl_chans = Vec::with_capacity(threads_count);
        let mut fb_chans = Vec::with_capacity(threads_count);

        for i in 0..threads_count {
            let current_id = core_ids[i];
            let sched = sched.clone();

            let (tx, rx) = unbounded();
            ctrl_chans.push(tx);

            let (f_tx, f_rx) = unbounded();
            fb_chans.push(f_rx);

            thread::spawn(clone!(task_graph, dsp_edges => move || {
                // Set the affinity so that a thread will always be executed
                // on the same CPU
                core_affinity::set_for_current(current_id);

                // The main loop of the worker
                loop {
                    // Wait for the next audio cycle
                    match rx.recv() {
                        Err(_) => {
                            println!("Failed to get more control messages");
                            break;
                        }
                        Ok(ctrl_msg) => match ctrl_msg {
                            CtrlMsg::Reset => continue, // Ignore the reset message
                            CtrlMsg::Start => {} // Start of the cycle
                        }
                    }

                    // Execute sequentially the tasks scheduled on the worker's `Processor`
                    'processing: for node_index in sched.processors[i].time_slots.iter().map(|ts| ts.get_node()) {
                        let backoff = Backoff::new();

                        debug_assert!(
                            task_graph.read().unwrap().get_state(node_index)
                            != Some(TaskState::Completed),
                            "Task already executed?!"
                        );

                        // Wait the the task to become ready
                        while task_graph.read().unwrap().get_state(node_index) != Some(TaskState::Ready) {
                            // Do not wait any longer if the next cycle already started
                            if rx.try_recv() == Ok(CtrlMsg::Reset) {
                                break 'processing;
                            }

                            backoff.snooze();
                        }

                        // Execute the task
                        exec_task(node_index, task_graph.clone(), dsp_edges.clone(), None);
                    }

                    // Notify the main thread the worker is done for this cycle
                    f_tx.send(FeedbackMsg::Done).unwrap();
                }
            }));
        }

        ThreadPool {
            ctrl_chans,
            fb_chans,
        }
    }

    /// Tell the thread pool the audio cycle has started so that it will
    /// execute the audio graph. This method blocks until the end of the
    /// execution.
    pub fn start(&mut self) {
        // Reset any worker which would still be executing tasks from
        // the previous cycle
        for chan in self.ctrl_chans.iter() {
            chan.send(CtrlMsg::Reset).unwrap();
        }

        // Notify the workers a new cycle started
        for chan in self.ctrl_chans.iter() {
            chan.send(CtrlMsg::Start).unwrap();
        }

        // Wait for every workers to be done
        for chan in self.fb_chans.iter() {
            chan.recv().expect("Could not get feedback messages");
        }
    }
}
