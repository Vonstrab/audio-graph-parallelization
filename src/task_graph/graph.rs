use std::collections::HashMap;

use super::node::Node;
use super::state::TaskState;
use super::task::Task;

#[derive(Debug)]
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

    pub fn find_task(&self, _taks: &Task) -> Option<usize> {
        unimplemented!()
    }

    pub fn get_wcet(&mut self, node_index: usize) -> Option<f64> {
        if node_index < self.nodes.len() {
            self.nodes[node_index].get_wcet()
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

    pub fn set_state(&mut self, node_index: usize, state: TaskState) -> bool {
        if node_index < self.nodes.len() {
            self.nodes[node_index].state = state;
            true
        } else {
            false
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

    pub fn get_t_level(&mut self, node_index: usize) -> Option<f64> {
        let top_ord = self.get_topological_order();
        let mut t_levels: Vec<f64> = std::iter::repeat(0.0).take(self.nodes.len()).collect();

        for i in top_ord {
            let mut max: f64 = 0.0;

            for x in self.get_predecessors(i).unwrap_or_default() {
                if t_levels[x]
                    + self.get_wcet(x).unwrap()
                    + self.get_communication_cost(x, i).unwrap_or(0.0)
                    > max
                {
                    max = t_levels[x]
                        + self.get_wcet(x).unwrap()
                        + self.get_communication_cost(x, i).unwrap_or(0.0);
                }
            }

            t_levels[i] = max;
        }

        t_levels.get(node_index).map(|val| *val)
    }

    pub fn get_b_level(&mut self, node_index: usize) -> Option<f64> {
        let rev_top_ord = self.get_rev_topological_order();
        let mut b_levels: Vec<f64> = std::iter::repeat(0.0).take(self.nodes.len()).collect();
        for i in rev_top_ord {
            let mut max: f64 = 0.0;

            for y in self.get_successors(i).unwrap_or_default() {
                let comm_cost = self.get_communication_cost(i, y).unwrap_or(0.0);

                if comm_cost + b_levels[y] > max {
                    max = comm_cost + b_levels[y];
                }
            }

            b_levels[i] = self.get_wcet(i).unwrap() + max;
        }

        b_levels.get(node_index).map(|val| *val)
    }

    pub fn get_static_level(&mut self, node_index: usize) -> Option<f64> {
        let rev_top_ord = self.get_rev_topological_order();
        let mut s_levels: Vec<f64> = std::iter::repeat(0.0).take(self.nodes.len()).collect();

        for i in rev_top_ord {
            let mut max: f64 = 0.0;

            for y in self.get_successors(i).unwrap_or_default() {
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
            self.nodes[dest_node_index]
                .predecessors
                .push(src_node_index);

            self.edges.insert((src_node_index, dest_node_index), None);

            true
        } else {
            false
        }
    }

    pub fn output_dot(&self, filename: &str) -> Result<(), std::io::Error> {
        let mut dot_file = String::new();

        dot_file.push_str("strict digraph{\n");

        for i in 0..(self.nodes.len() - 1) {
            let ligne = format!("{};\n", i);

            dot_file.push_str(ligne.as_str());
        }

        for ((s, t), _) in &self.edges {
            let ligne = format!("{} -> {};\n", s, t);

            dot_file.push_str(ligne.as_str());
        }

        dot_file.push_str("}\n");

        let path = Path::new(filename);
        let mut file = File::create(&path).expect("Impossible to create file.");

        write!(file, "{}", dot_file)
    }
}

pub fn run_dot(graph: &TaskGraph, graph_name: &str) {
    let tmp_dot = format!("tmp/{}.got", graph_name);

    Command::new("mkdir")
        .arg("visual")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    graph
        .output_dot(tmp_dot.as_str())
        .unwrap_or_else(|e| panic!("failed to output graph: {}", e));

    let pdf_filename = format!("tmp/{}.pdf", graph_name);

    Command::new("dot")
        .arg("-Tpdf")
        .arg(tmp_dot)
        .arg("-o")
        .arg(pdf_filename)
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topological_sort() {
        let mut g = TaskGraph::new(8, 9);
        let mut nodes_idx = Vec::new();

        for _ in 0..8 {
            nodes_idx.push(g.add_task(Task::Constant(1.0)));
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
            nodes_idx.push(g.add_task(Task::Constant(1.0)));
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