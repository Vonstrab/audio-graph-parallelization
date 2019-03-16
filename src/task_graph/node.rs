extern crate rand;

use super::state::TaskState;
use super::task::Task;

use self::rand::Rng;

#[derive(Debug)]
pub struct Node {
    pub task: Task,
    pub wcet: Option<f64>, // Worst case execution time
    pub state: TaskState,
    pub predecessors: Vec<usize>,
    pub successors: Vec<usize>,
}

impl Node {
    pub fn new(task: Task) -> Node {
        Node {
            task,
            wcet: None,
            state: TaskState::WaitingDependencies,
            predecessors: Vec::new(),
            successors: Vec::new(),
        }
    }

    pub fn get_wcet(&mut self) -> Option<f64> {
        if self.wcet.is_some() {
            return self.wcet;
        }

        match self.task {
            Task::Constant(x) => {
                if x < 0.0 {
                    panic!("Error: negative constant WCET\n");
                }

                self.wcet = Some(x);
                self.wcet
            }
            Task::Random(start, end) => {
                if end < start {
                    panic!("Error: bad interval for random WCET\n");
                }

                if start < 0.0 {
                    panic!("Error: negative start for random WCET\n");
                }

                if end < 0.0 {
                    panic!("Error: negative end for random WCET\n");
                }

                let mut rng = rand::thread_rng();
                let x: f64 = rng.gen_range(start, end);

                self.wcet = Some(x);
                self.wcet
            }
            // TODO: Estimations for Puredata and Audiograph
            Task::Audiograph { wcet, .. } => {
                if wcet.is_some() {
                    self.wcet = wcet;
                    self.wcet
                } else {
                    self.wcet = Some(0.0);
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
        assert_eq!(node.predecessors.len(), 0);
        assert_eq!(node.successors.len(), 0);
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
