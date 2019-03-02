extern crate rand;

use super::state::TaskState;
use super::task::Task;

use self::rand::Rng;
// use rand::seq::SliceRandom;

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

    pub fn get_wcet(&mut self) ->Option< f64> {
        if !self.wcet.is_none() {
            return self.wcet;
        }

        match self.task {
            Task::Constant(x) => {
                if x < 0.0 {
                    panic!("Erreur Wcet Constant négatif");
                }
                self.wcet = Some(x);
                return self.wcet;
            }
            Task::Random(start,end) => {
                if end < start  {
                    panic!("Erreur Wcet Random mauvais intervalle");
                }
                if start < 0.0 {
                    panic!("Erreur Wcet Random start négatif");
                }if end < 0.0 {
                    panic!("Erreur Wcet Random end négatif");
                }

                let mut rng = rand::thread_rng();
                let x : f64 = rng.gen_range(start, end);
                self.wcet = Some(x);
                return self.wcet;
            }
            _ =>{
                //TODO Calcul for Puredata and Audiograph
                return None;
            }
        }
    }
}
