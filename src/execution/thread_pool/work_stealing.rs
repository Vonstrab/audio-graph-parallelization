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
    Start,
    Stop,
}

#[derive(Clone, Copy)]
enum FeedbackMsg {
    Done,
}

pub struct ThreadPool {
    task_graph: Arc<RwLock<TaskGraph>>,

    ctrl_chans: Vec<Sender<CtrlMsg>>,
    main_queue: Arc<Injector<usize>>,
    fb_chans: Vec<Receiver<FeedbackMsg>>,
}

impl ThreadPool {
    pub fn create(
        threads_count: usize,
        task_graph: Arc<RwLock<TaskGraph>>,
        dsp_edges: Arc<RwLock<HashMap<(usize, usize), Arc<RwLock<DspEdge>>>>>,
    ) -> ThreadPool {
        let core_ids = core_affinity::get_core_ids().expect("Failed to get core IDs.");
        let mut join_handles = Vec::with_capacity(threads_count);
        let main_queue = Arc::new(Injector::new());
        let stealers = Arc::new(ShardedLock::new(Vec::with_capacity(threads_count)));
        let mut ctrl_chans = Vec::with_capacity(threads_count);
        let mut fb_chans = Vec::with_capacity(threads_count);

        for i in 0..threads_count {
            let current_id = core_ids[i];

            let worker_queue = Worker::new_lifo();
            stealers.write().unwrap().push(worker_queue.stealer());

            let (tx, rx) = unbounded();
            ctrl_chans.push(tx);

            let (f_tx, f_rx) = unbounded();
            fb_chans.push(f_rx);

            join_handles.push(thread::spawn(
                clone!(main_queue, stealers, task_graph, dsp_edges => move || {
                    let mut init = true;
                    core_affinity::set_for_current(current_id);

                    loop {
                        match worker_queue.pop() {
                            None => {
                                match main_queue.steal() {
                                    Steal::Empty | Steal::Retry => {
                                        if stealers.read().unwrap().iter().all(|stealer| stealer.is_empty()) {
                                            if !init {
                                                f_tx.send(FeedbackMsg::Done).unwrap();
                                            } else {
                                                init = false;
                                            }

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

    pub fn start(&mut self) {
        let entry_nodes = self.task_graph.write().unwrap().get_entry_nodes();

        for node_index in entry_nodes {
            self.main_queue.push(node_index);
        }

        for chan in self.ctrl_chans.iter() {
            chan.send(CtrlMsg::Start).unwrap();
        }

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
