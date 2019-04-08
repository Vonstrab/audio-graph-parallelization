extern crate rand;

use std::sync::RwLock;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::state::TaskState;
use super::task::DspTask;
use super::task::Task;
use crate::dsp::{DspEdge, DspNode};

use self::rand::Rng;

#[derive(Debug)]
pub struct Node {
    pub task: Task,
    pub dsp_task: Arc<Mutex<Option<DspTask>>>,
    pub wcet: Option<f64>, // Worst case execution time
    pub state: TaskState,
}

impl Node {
    pub fn new(task: Task) -> Node {
        Node {
            task,
            dsp_task: Arc::new(Mutex::new(None)),
            wcet: None,
            state: TaskState::WaitingDependencies(0),
        }
    }

    pub fn with_dsp(dsp: DspTask) -> Node {
        Node {
            task: Task::Constant(0.0),
            dsp_task: Arc::new(Mutex::new(Some(dsp))),
            wcet: None,
            state: TaskState::WaitingDependencies(0),
        }
    }

    pub fn estimate_wcet(&mut self) {
        if self.wcet.is_some() {
            return;
        }
        let mut max_duration = None;
        let mut timer = Instant::now();
        let mut dsp = self.dsp_task.lock().unwrap().take();
        if dsp.is_some() {
            let unw_dsp = dsp.unwrap().dsp;
            for _ in 0..50 {
                match unw_dsp {
                    DspNode::Oscillator(mut Oscillator) => {
                        Oscillator.process(Arc::new(RwLock::new(DspEdge::new(32, 2))));
                    }
                    DspNode::Modulator(mut Modulator) => {
                        Modulator.process(
                            Arc::new(RwLock::new(DspEdge::new(32, 2))),
                            Arc::new(RwLock::new(DspEdge::new(32, 2))),
                        );
                    }
                    DspNode::InputsOutputsAdaptor(mut InputsOutputsAdaptor) => {
                        InputsOutputsAdaptor.process(
                            vec![
                                Arc::new(RwLock::new(DspEdge::new(32, 2))),
                                Arc::new(RwLock::new(DspEdge::new(32, 2))),
                            ],
                            vec![Arc::new(RwLock::new(DspEdge::new(32, 2)))],
                        );
                    }
                    DspNode::Sink(mut Sink) => {
                        let mut vec = vec![0.0];
                        let mut buffer = vec.as_mut_slice();
                        Sink.set_buffer(buffer.as_mut_ptr(), 60);
                        Sink.process(Arc::new(RwLock::new(DspEdge::new(32, 2))));
                    }
                }
                let cur_dur = timer.elapsed();
                if max_duration.is_none() {
                    max_duration = Some(cur_dur);
                } else if cur_dur.subsec_micros() > max_duration.unwrap().subsec_micros() {
                    max_duration = Some(cur_dur);
                }
            }
            println!("time in {} micros ", max_duration.unwrap().subsec_micros());
        }
        self.wcet = Some(max_duration.unwrap().subsec_micros() as f64 / 1000000.0);
    }

    pub fn get_wcet(&mut self) -> Option<f64> {
        self.estimate_wcet();
        if self.wcet.is_some() {
            return self.wcet;
        }
        match self.task {
            Task::Constant(x) => {
                if x < 0.0 {
                    panic!("Node::get_wcet : negative constant WCET\n");
                }

                self.wcet = Some(x);
                self.wcet
            }
            Task::Random(start, end) => {
                if end < start {
                    panic!("Node::get_wcet : bad interval for random WCET\n");
                }

                if start < 0.0 {
                    panic!("Node::get_wcet : negative start for random WCET\n");
                }

                if end < 0.0 {
                    panic!("Node::get_wcet : negative end for random WCET\n");
                }

                let mut rng = rand::thread_rng();
                let x: f64 = rng.gen_range(start, end);

                self.wcet = Some(x);
                self.wcet
            }
            //NB CPFD is erratic if WCET is 0
            //if will over-duplicate with no cost
            Task::Audiograph { wcet, .. } => {
                if wcet.is_some() {
                    self.wcet = wcet;
                    self.wcet
                } else {
                    self.wcet = Some(0.1);
                    self.wcet
                }
            }

            _ => {
                // TODO: Estimations for Puredata and Audiograph
                self.wcet = Some(1.0);
                self.wcet
            }
        }
    }
}

#[cfg(test)]
mod node_test {
    use super::*;

    #[test]
    fn test_constructor() {
        let node = Node::new(Task::Constant(5.0));
        assert_eq!(node.task, Task::Constant(5.0));
        assert_eq!(node.wcet, None);
        // assert_eq!(node.predecessors.len(), 0);
        // assert_eq!(node.successors.len(), 0);
        assert_eq!(node.state, TaskState::WaitingDependencies(0));
    }

    #[test]
    fn test_wcet_constant() {
        let mut node = Node::new(Task::Constant(5.0));
        assert_eq!(node.get_wcet(), Some(5.0));
    }

    #[test]
    fn test_wcet_random() {
        let mut node = Node::new(Task::Random(1.0, 5.0));
        let wcet = node.get_wcet().unwrap();
        assert!(wcet <= 5.0);
        assert!(wcet >= 1.0);
    }

    //TODo test for the wcet
}
