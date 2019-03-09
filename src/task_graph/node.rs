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
                return self.wcet;
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
                return self.wcet;
            }
            Task::Audiograph { wcet, .. } => {
                self.wcet = wcet;
                return self.wcet;
            }
            _ => {
                //TODO Calcul for Puredata and Audiograph
                self.wcet = Some(1.0);
                return self.wcet;
            }
        }
    }
}
