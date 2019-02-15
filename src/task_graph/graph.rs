use std::collections::HashMap;

use super::node::Node;
use super::state::TaskState;
use super::task::Task;

pub struct TaskGraph {
    nodes: Vec<Node>,
    edges: HashMap<(usize, usize), Option<f64>>,
    entry_nodes: Vec<usize>,
    exit_nodes: Vec<usize>,
}

impl TaskGraph {
    pub fn get_entry_nodes(&self) -> &Vec<usize> {
        &self.entry_nodes
    }

    pub fn get_exit_nodes(&self) -> &Vec<usize> {
        &self.exit_nodes
    }

    pub fn get_predecessors(&self, node_index: usize) -> Option<&Vec<usize>> {
        if node_index < self.nodes.len() {
            Some(&self.nodes[node_index].predecessors)
        } else {
            None
        }
    }

    pub fn get_successors(&self, node_index: usize) -> Option<&Vec<usize>> {
        if node_index < self.nodes.len() {
            Some(&self.nodes[node_index].successors)
        } else {
            None
        }
    }

    pub fn get_topological_order(&self) -> Vec<usize> {
        unimplemented!()
    }

    pub fn get_rev_topological_order(&self) -> Vec<usize> {
        unimplemented!()
    }

    pub fn find_task(&self, taks: &Task) -> Option<usize> {
        unimplemented!()
    }

    pub fn get_wcet(&self, node_index: usize) -> Option<f64> {
        if node_index < self.nodes.len() {
            self.nodes[node_index].wcet
        } else {
            None
        }
    }

    pub fn get_state(&self, node_index: usize) -> Option<TaskState> {
        if node_index < self.nodes.len() {
            Some(self.nodes[node_index].state)
        } else {
            None
        }
    }

    pub fn get_communication_cost(
        &self,
        src_node_index: usize,
        dest_node_index: usize,
    ) -> Option<f64> {
        unimplemented!()
    }

    pub fn get_t_level(&self, node_index: usize) -> Option<f64> {
        unimplemented!()
    }

    pub fn get_b_level(&self, node_index: usize) -> Option<f64> {
        unimplemented!()
    }

    pub fn get_static_level(&self, node_index: usize) -> Option<f64> {
        unimplemented!()
    }

    pub fn add_node(&mut self) -> usize {
        unimplemented!()
    }

    pub fn add_edge(&mut self, src_node_index: usize, dest_node_index: usize) -> bool {
        unimplemented!()
    }

    pub fn assign_task(&mut self, node_index: usize) -> bool {
        unimplemented!()
    }
}
