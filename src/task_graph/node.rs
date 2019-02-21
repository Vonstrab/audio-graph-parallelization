use super::state::TaskState;
use super::task::Task;

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

    pub fn set_wcet(& mut self, value: f64) -> bool {
        if value > 0.0 {
            self.wcet = Some(value);
            true
        } else {
            false
        }
    }
}
