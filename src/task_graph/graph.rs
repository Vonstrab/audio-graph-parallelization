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
    pub fn new(nodes_count: usize, edges_count: usize) -> TaskGraph {
        TaskGraph {
            nodes: Vec::with_capacity(nodes_count),
            edges: HashMap::with_capacity(edges_count),
            entry_nodes: Vec::new(),
            exit_nodes: Vec::new(),
        }
    }

    pub fn get_entry_nodes(&mut self) -> &Vec<usize> {
        if self.entry_nodes.is_empty() {
            for i in 0..self.nodes.len() {
                if self.nodes[i].predecessors.is_empty() {
                    self.entry_nodes.push(i);
                }
            }
        }

        &self.entry_nodes
    }

    pub fn get_exit_nodes(&mut self) -> &Vec<usize> {
        if self.exit_nodes.is_empty() {
            for i in 0..self.nodes.len() {
                if self.nodes[i].successors.is_empty() {
                    self.exit_nodes.push(i);
                }
            }
        }

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
        let mut top_ord = self.get_rev_topological_order();
        top_ord.reverse();

        top_ord
    }

    pub fn get_rev_topological_order(&self) -> Vec<usize> {
        let mut stack = Vec::new();
        let mut visited: Vec<bool> = std::iter::repeat(false).take(self.nodes.len()).collect();

        for node_idx in 0..self.nodes.len() {
            if !visited[node_idx] {
                self.dfs(node_idx, &mut stack, &mut visited);
            }
        }

        stack
    }

    /// This method is used by the topological sort
    fn dfs(&self, node_index: usize, stack: &mut Vec<usize>, visited: &mut Vec<bool>) {
        visited[node_index] = true;

        for succ_idx in self.get_successors(node_index).clone().unwrap() {
            if !visited[*succ_idx] {
                self.dfs(*succ_idx, stack, visited);
            }
        }

        stack.push(node_index);
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
        self.edges
            .get(&(src_node_index, dest_node_index))
            .map(|val| *val)
            .unwrap_or(None)
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

    pub fn add_task(&mut self, task: Task) -> usize {
        self.nodes.push(Node::new(task));

        self.nodes.len() - 1
    }

    pub fn add_edge(&mut self, src_node_index: usize, dest_node_index: usize) -> bool {
        if src_node_index < self.nodes.len() && dest_node_index < self.nodes.len() {
            self.edges.insert((src_node_index, dest_node_index), None);

            true
        } else {
            false
        }
    }
}
