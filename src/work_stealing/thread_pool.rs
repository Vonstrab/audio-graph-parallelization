use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;
use std::thread::JoinHandle;

use crossbeam::channel::{unbounded, Sender};
use crossbeam::deque::{Injector, Steal, Stealer, Worker};
use crossbeam::sync::ShardedLock;

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

#[derive(Clone, Copy)]
enum CtrlMsg {
    Start,
    Stop,
}

pub struct ThreadPool {
    join_handles: Vec<JoinHandle<()>>,

    task_graph: Arc<RwLock<TaskGraph>>,

    ctrl_chans: Vec<Sender<CtrlMsg>>,
    main_queue: Arc<Injector<usize>>,
    wait_cycle: Arc<Condvar>,
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
        let wait_cycle = Arc::new(Condvar::new());
        let mut ctrl_chans = Vec::with_capacity(threads_count);

        for i in 0..threads_count {
            let current_id = core_ids[i];

            let worker_queue = Worker::new_lifo();
            stealers.write().unwrap().push(worker_queue.stealer());

            let (tx, rx) = unbounded();
            ctrl_chans.push(tx);

            join_handles.push(thread::spawn(
                clone!(main_queue, stealers, wait_cycle, task_graph, dsp_edges => move || {
                    core_affinity::set_for_current(current_id);

                    loop {
                        match worker_queue.pop() {
                            None => {
                                match main_queue.steal() {
                                    Steal::Empty | Steal::Retry => {
                                        if stealers.read().unwrap().iter().all(|stealer| stealer.is_empty()) {
                                            wait_cycle.notify_all();

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
                                                            &worker_queue,
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
                                            &worker_queue,
                                        );
                                    }
                                }
                            }
                            Some(node_index) => {
                                exec_task(node_index,
                                    task_graph.clone(),
                                    dsp_edges.clone(),
                                    &worker_queue
                                );
                            }
                        }
                    }
                }),
            ));
        }

        ThreadPool {
            join_handles,

            task_graph,

            ctrl_chans,
            main_queue,
            wait_cycle,
        }
    }

    pub fn start(&mut self) {
        self.wait_cycle_end();

        let entry_nodes = self.task_graph.write().unwrap().get_entry_nodes();

        for node_index in entry_nodes {
            self.main_queue.push(node_index);
        }

        for chan in self.ctrl_chans.iter() {
            chan.send(CtrlMsg::Start).unwrap();
        }
    }

    pub fn stop(&self) {
        self.wait_cycle_end(); // FIXME: Causes a panic

        for chan in self.ctrl_chans.iter() {
            chan.send(CtrlMsg::Stop).unwrap();
        }
    }

    fn wait_cycle_end(&self) {
        if !self.main_queue.is_empty() {
            let m = Mutex::new(());
            let mut guard = m.lock().unwrap();

            while !self.main_queue.is_empty() {
                guard = self.wait_cycle.wait(guard).unwrap();
            }
        }
    }
}

fn exec_task(
    node_index: usize,
    task_graph: Arc<RwLock<TaskGraph>>,
    dsp_edges: Arc<RwLock<HashMap<(usize, usize), Arc<RwLock<DspEdge>>>>>,
    worker_queue: &Worker<usize>,
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

        for succ in successors {
            task_graph.write().unwrap().dec_activation_count(succ);

            if task_graph.read().unwrap().get_state(succ) == Some(TaskState::Ready) {
                worker_queue.push(succ);
            }
        }
    }
}