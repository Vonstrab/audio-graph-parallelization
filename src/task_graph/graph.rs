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

    pub fn get_entry_nodes(&mut self) -> Vec<usize> {
        if self.entry_nodes.is_empty() {
            for i in 0..self.nodes.len() {
                if self.nodes[i].predecessors.is_empty() {
                    self.entry_nodes.push(i);
                }
            }
        }

        self.entry_nodes.clone()
    }

    pub fn get_exit_nodes(&mut self) -> Vec<usize> {
        if self.exit_nodes.is_empty() {
            for i in 0..self.nodes.len() {
                if self.nodes[i].successors.is_empty() {
                    self.exit_nodes.push(i);
                }
            }
        }

        self.exit_nodes.clone()
    }

    pub fn get_predecessors(&self, node_index: usize) -> Option<Vec<usize>> {
        if node_index < self.nodes.len() {
            Some(self.nodes[node_index].predecessors.clone())
        } else {
            None
        }
    }

    pub fn get_successors(&self, node_index: usize) -> Option<Vec<usize>> {
        if node_index < self.nodes.len() {
            Some(self.nodes[node_index].successors.clone())
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

        for succ_idx in self.get_successors(node_index).unwrap() {
            if !visited[succ_idx] {
                self.dfs(succ_idx, stack, visited);
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
        let top_ord = self.get_topological_order();
        let mut t_levels: Vec<f64> = std::iter::repeat(0.0).take(self.nodes.len()).collect();

        for i in top_ord {
            let mut max: f64 = 0.0;

            for x in self.get_predecessors(i).unwrap() {
                if t_levels[x]
                    + self.get_wcet(x).unwrap()
                    + self.get_communication_cost(x, i).unwrap()
                    > max
                {
                    max = t_levels[x]
                        + self.get_wcet(x).unwrap()
                        + self.get_communication_cost(x, i).unwrap();
                }
            }

            t_levels[i] = max;
        }

        t_levels.get(node_index).map(|val| *val)
    }

    pub fn get_b_level(&self, node_index: usize) -> Option<f64> {
        let rev_top_ord = self.get_rev_topological_order();
        let mut b_levels: Vec<f64> = std::iter::repeat(0.0).take(self.nodes.len()).collect();

        for i in rev_top_ord {
            let mut max: f64 = 0.0;

            for y in self.get_successors(i).unwrap() {
                if self.get_communication_cost(i, y).unwrap() + b_levels[y] > max {
                    max = self.get_communication_cost(i, y).unwrap() + b_levels[y];
                }
            }

            b_levels[i] = self.get_wcet(i).unwrap() + max;
        }

        b_levels.get(node_index).map(|val| *val)
    }

    pub fn get_static_level(&self, node_index: usize) -> Option<f64> {
        let rev_top_ord = self.get_rev_topological_order();
        let mut s_levels: Vec<f64> = std::iter::repeat(0.0).take(self.nodes.len()).collect();

        for i in rev_top_ord {
            let mut max: f64 = 0.0;

            for y in self.get_successors(i).unwrap() {
                if s_levels[y] > max {
                    max = s_levels[y];
                }
            }

            s_levels[i] = self.get_wcet(i).unwrap() + max;
        }

        s_levels.get(node_index).map(|val| *val)
    }

    pub fn add_task(&mut self, task: Task) -> usize {
        self.nodes.push(Node::new(task));

        self.nodes.len() - 1
    }

    pub fn add_edge(&mut self, src_node_index: usize, dest_node_index: usize) -> bool {
        if src_node_index < self.nodes.len() && dest_node_index < self.nodes.len() {
            self.nodes[src_node_index].successors.push(dest_node_index);
            self.nodes[dest_node_index].predecessors.push(src_node_index);

            self.edges.insert((src_node_index, dest_node_index), None);

            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topological_sort() {
        let mut g = TaskGraph::new(8, 9);
        let mut nodes_idx = Vec::new();

        for _ in 0..8 {
            nodes_idx.push(g.add_task(Task::A));
        }

        g.add_edge(7, 5);
        g.add_edge(7, 6);
        g.add_edge(5, 2);
        g.add_edge(5, 4);
        g.add_edge(6, 4);
        g.add_edge(6, 3);
        g.add_edge(2, 1);
        g.add_edge(3, 1);
        g.add_edge(1, 0);

        let top_ord = g.get_topological_order();

        assert_eq!(top_ord, vec![7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_reverse_topological_sort() {
        let mut g = TaskGraph::new(8, 9);
        let mut nodes_idx = Vec::new();

        for _ in 0..8 {
            nodes_idx.push(g.add_task(Task::A));
        }

        g.add_edge(7, 5);
        g.add_edge(7, 6);
        g.add_edge(5, 2);
        g.add_edge(5, 4);
        g.add_edge(6, 4);
        g.add_edge(6, 3);
        g.add_edge(2, 1);
        g.add_edge(3, 1);
        g.add_edge(1, 0);

        let top_ord = g.get_rev_topological_order();

        assert_eq!(top_ord, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }
}
