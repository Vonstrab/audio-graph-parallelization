use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};

use rand::Rng;

use crate::dsp::{DspEdge, DspNode};

use super::state::TaskState;
use super::task::{DspTask, Task};

#[derive(Debug)]
pub struct Node {
    pub task: Task,
    pub dsp_task: Arc<Mutex<Option<DspTask>>>,
    pub wcet: Option<f64>, // Worst case execution time
    pub state: TaskState,

    pub sample_rate: Option<usize>,
    pub buffer_size: Option<usize>,
}

impl Node {
    /// Creates a new `Node` with a fake task. This is only used for testing the static scheduling
    /// algorithms and thus it can't be used to execute a task graph with DSPs.
    pub fn new(task: Task) -> Node {
        Node {
            task,
            dsp_task: Arc::new(Mutex::new(None)),
            wcet: None,
            state: TaskState::WaitingDependencies(0),
            sample_rate: None,
            buffer_size: None,
        }
    }

    /// Creates a new `Node` with a `DspTask`. Used for executing an audio graph.
    pub fn with_dsp(dsp: DspTask) -> Node {
        Node {
            task: Task::Constant(0.0),
            dsp_task: Arc::new(Mutex::new(Some(dsp))),
            wcet: None,
            state: TaskState::WaitingDependencies(0),
            sample_rate: None,
            buffer_size: None,
        }
    }

    /// Returns the estimated WCET of a node.
    pub fn get_wcet(&mut self) -> Option<f64> {
        if self.wcet.is_none() {
            self.estimate_wcet();
        }

        self.wcet
    }

    // This method is used by `get_wcet` to do the estimation of the WCET of a node's task.
    fn estimate_wcet(&mut self) {
        let dsp = self.dsp_task.lock().unwrap();

        match dsp.as_ref() {
            None => {
                self.wcet = match self.task {
                    Task::Constant(x) => {
                        if x < 0.0 {
                            None
                        } else {
                            Some(x)
                        }
                    }
                    Task::Random(start, end) => {
                        if start < 0.0 || end < 0.0 || end < start {
                            None
                        } else {
                            Some(rand::thread_rng().gen_range(start, end))
                        }
                    }
                    // CPFD is erratic when the WCET is 0,
                    // it will over-duplicate with no cost
                    Task::Audiograph { wcet, .. } => wcet.or(Some(0.1)),
                    _ => Some(1.0),
                };
            }
            Some(dsp) => {
                let dsp = &dsp.dsp;

                let mut max_duration = Duration::new(0, 0);

                for _ in 0..50 {
                    let timer = Instant::now();

                    match dsp {
                        DspNode::Oscillator(mut o) => {
                            o.process(Arc::new(RwLock::new(DspEdge::new(
                                self.buffer_size.expect("buffer size not set"),
                                self.sample_rate.expect("sample rate not set"),
                            ))));
                        }
                        DspNode::Modulator(mut m) => {
                            m.process(
                                Arc::new(RwLock::new(DspEdge::new(
                                    self.buffer_size.expect("buffer size not set"),
                                    self.sample_rate.expect("sample rate not set"),
                                ))),
                                Arc::new(RwLock::new(DspEdge::new(
                                    self.buffer_size.expect("buffer size not set"),
                                    self.sample_rate.expect("sample rate not set"),
                                ))),
                            );
                        }
                        DspNode::InputsOutputsAdaptor(mut ioa) => {
                            ioa.process(
                                vec![
                                    Arc::new(RwLock::new(DspEdge::new(
                                        self.buffer_size.expect("buffer size not set"),
                                        self.sample_rate.expect("sample rate not set"),
                                    ))),
                                    Arc::new(RwLock::new(DspEdge::new(
                                        self.buffer_size.expect("buffer size not set"),
                                        self.sample_rate.expect("sample rate not set"),
                                    ))),
                                ],
                                vec![Arc::new(RwLock::new(DspEdge::new(
                                    self.buffer_size.expect("buffer size not set"),
                                    self.sample_rate.expect("sample rate not set"),
                                )))],
                            );
                        }
                        DspNode::Sink(mut s) => {
                            let mut vec = vec![0.0; self.buffer_size.expect("buffer size not set")];
                            let mut buffer = vec.as_mut_slice();

                            s.set_buffer(buffer.as_mut_ptr(), 60);
                            s.process(Arc::new(RwLock::new(DspEdge::new(
                                self.buffer_size.expect("buffer size not set"),
                                self.sample_rate.expect("sample rate not set"),
                            ))));
                        }
                    }

                    let duration = timer.elapsed();

                    if max_duration < duration {
                        max_duration = duration;
                    }
                }

                self.wcet = Some(f64::from(max_duration.subsec_micros()) / 1_000_000.0);
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

    // TODO: Tests for the WCET
}
