use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;

use crossbeam::channel::{unbounded, Receiver, Sender};
use crossbeam::deque::{Injector, Steal, Worker};
use crossbeam::sync::ShardedLock;

use crate::dsp::DspEdge;
use crate::execution::utils::exec_task;
use crate::task_graph::graph::TaskGraph;

#[derive(Clone, Copy)]
enum CtrlMsg {
    Start, // Used for telling a worker to start the execution
    Stop, // Used for telling a worker to exit
}

#[derive(Clone, Copy)]
enum FeedbackMsg {
    Done, // Message sent by a worker when it has completed all its tasks
}

pub struct ThreadPool {
    task_graph: Arc<RwLock<TaskGraph>>,

    ctrl_chans: Vec<Sender<CtrlMsg>>,
    main_queue: Arc<Injector<usize>>,
    fb_chans: Vec<Receiver<FeedbackMsg>>,
}

impl ThreadPool {
    /// Creates a thread pool for the parallel execution of an audio graph
    /// with a dynamic work stealing scheduling.
    ///
    /// # Arguments
    ///
    /// * `threads_count` - The number of threads of the pool
    /// * `task_graph` - The audio graph to be executed by the thread pool
    /// * `dsp_edges` - The buffers of the graph
    pub fn create(
        threads_count: usize,
        task_graph: Arc<RwLock<TaskGraph>>,
        dsp_edges: Arc<RwLock<HashMap<(usize, usize), Arc<RwLock<DspEdge>>>>>,
    ) -> ThreadPool {
        let core_ids = core_affinity::get_core_ids().expect("Failed to get core IDs.");
        let mut join_handles = Vec::with_capacity(threads_count);
        // The main queue used for the initial distribution of tasks to the
        // workers
        let main_queue = Arc::new(Injector::new());
        // The `Stealers` used by the workers to steal tasks from each others
        let stealers = Arc::new(ShardedLock::new(Vec::with_capacity(threads_count)));
        let mut ctrl_chans = Vec::with_capacity(threads_count);
        let mut fb_chans = Vec::with_capacity(threads_count);

        for i in 0..threads_count {
            let current_id = core_ids[i];

            // The queue of the worker
            let worker_queue = Worker::new_lifo();
            stealers.write().unwrap().push(worker_queue.stealer());

            let (tx, rx) = unbounded();
            ctrl_chans.push(tx);

            let (f_tx, f_rx) = unbounded();
            fb_chans.push(f_rx);

            join_handles.push(thread::spawn(
                clone!(main_queue, stealers, task_graph, dsp_edges => move || {
                    let mut init = true;
                    // Set the affinity so that a thread will always be
                    // executed on the same CPU
                    core_affinity::set_for_current(current_id);

                    // The main loop of the worker
                    loop {
                        match worker_queue.pop() {
                            // If there is no task in the workers queue
                            // look for a task somewhere else
                            None => {
                                match main_queue.steal() {
                                    Steal::Empty | Steal::Retry => {
                                        // If there is no more tasks anywhere
                                        // it means the worker is done
                                        if stealers.read().unwrap().iter().all(|stealer| stealer.is_empty()) {
                                            // If it the first time we get
                                            // here, we didn't completed any
                                            // tasks. If not, we must notify
                                            // the main the worker is done
                                            // for this cycle.
                                            if !init {
                                                f_tx.send(FeedbackMsg::Done).unwrap();
                                            } else {
                                                init = false;
                                            }

                                            // Wait for the next audio cycle
                                            match rx.recv() {
                                                Err(_) => {
                                                    println!("Failed to get more control messages");
                                                    break;
                                                }
                                                Ok(ctrl_msg) => match ctrl_msg {
                                                    CtrlMsg::Stop => break,
                                                    CtrlMsg::Start => {
                                                        continue;
                                                    }
                                                }
                                            }
                                        } else {
                                            // If there are remaining tasks
                                            // in other workers queues, steal
                                            // one of them and execute it
                                            for j in 0..threads_count {
                                                if j != i {
                                                    if let Steal::Success(node_index) =
                                                        stealers.read().unwrap()[i].steal()
                                                    {
                                                        exec_task(
                                                            node_index,
                                                            task_graph.clone(),
                                                            dsp_edges.clone(),
                                                            Some(&worker_queue),
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    // If there is a task in the main queue
                                    // take and execute it
                                    Steal::Success(node_index) => {
                                        exec_task(
                                            node_index,
                                            task_graph.clone(),
                                            dsp_edges.clone(),
                                            Some(&worker_queue),
                                        );
                                    }
                                }
                            }
                            // If there is a task in the worker's queue
                            // execute it
                            Some(node_index) => {
                                exec_task(node_index,
                                    task_graph.clone(),
                                    dsp_edges.clone(),
                                    Some(&worker_queue)
                                );
                            }
                        }
                    }
                }),
            ));
        }

        ThreadPool {
            task_graph,

            ctrl_chans,
            main_queue,
            fb_chans,
        }
    }

    /// Tell the thread pool the audio cycle has started so that it will
    /// execute the audio graph. This method blocks until the end of the
    /// execution.
    pub fn start(&mut self) {
        let entry_nodes = self.task_graph.write().unwrap().get_entry_nodes();

        // Put the first tasks to be ready in the main queue
        for node_index in entry_nodes {
            self.main_queue.push(node_index);
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

    pub fn stop(&self) {
        for chan in self.ctrl_chans.iter() {
            chan.send(CtrlMsg::Stop).unwrap();
        }
    }
}
