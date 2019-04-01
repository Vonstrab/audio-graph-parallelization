extern crate rand;

use std::sync::{Arc, Mutex};

use super::state::TaskState;
use super::task::DspTask;
use super::task::Task;

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
            state: TaskState::WaitingDependencies,
        }
    }

    pub fn with_dsp(dsp: DspTask) -> Node {
        Node {
            task: Task::Constant(0.0),
            dsp_task: Arc::new(Mutex::new(Some(dsp))),
            wcet: None,
            state: TaskState::WaitingDependencies,
        }
    }

    pub fn get_wcet(&mut self) -> Option<f64> {
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
        assert_eq!(node.state, TaskState::WaitingDependencies);
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
