//! Implements a task graph.
use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};

use crate::scheduling::schedule::Schedule;

use super::node::Node;
use super::state::TaskState;
use super::task::DspTask;
use super::task::Task;

#[derive(Debug)]
pub struct TaskGraph {
    nodes: Vec<Node>,
    // The edges are a hash map associating two (the source and the destination of the arc) nodes
    // with a communication cost.
    edges: HashMap<(usize, usize), Option<f64>>,
    entry_nodes: Vec<usize>,
    exit_nodes: Vec<usize>,
    adj_list: Vec<(Vec<usize>, Vec<usize>)>,

    sample_rate: Option<usize>,
    buffer_size: Option<usize>,
}

impl TaskGraph {
    /// Creates a new `TaskGraph`.
    ///
    /// # Arguments
    /// * `node_count` - The number of nodes in the `TaskGraph`
    /// * `edges_count` - The number of edges in the `TaskGraph`
    pub fn new(
        nodes_count: usize,
        edges_count: usize,
    ) -> TaskGraph {
        let mut adj_list = Vec::with_capacity(nodes_count);

        for _ in 0..nodes_count {
            adj_list.push((Vec::new(), Vec::new()));
        }

        TaskGraph {
            nodes: Vec::with_capacity(nodes_count),
            edges: HashMap::with_capacity(edges_count),
            entry_nodes: Vec::new(),
            exit_nodes: Vec::new(),
            adj_list,
            sample_rate: None,
            buffer_size: None,
        }
    }

    /// Returns the number of nodes in the graph.
    pub fn get_nb_node(&self) -> usize {
        self.adj_list.len()
    }

    /// Returns the number of edges in the graph.
    pub fn get_nb_edge(&self) -> usize {
        self.edges.len()
    }

    /// Returns the list of entry nodes: nodes without predecessors.
    pub fn get_entry_nodes(&mut self) -> Vec<usize> {
        if self.entry_nodes.is_empty() {
            for i in 0..self.nodes.len() {
                if self.get_predecessors(i).unwrap().is_empty() {
                    self.entry_nodes.push(i);
                }
            }
        }

        self.entry_nodes.clone()
    }

    /// Returns the list of exit nodes: nodes without successors.
    pub fn get_exit_nodes(&mut self) -> Vec<usize> {
        if self.exit_nodes.is_empty() {
            for i in 0..self.nodes.len() {
                if self.get_successors(i).unwrap().is_empty() {
                    self.exit_nodes.push(i);
                }
            }
        }

        self.exit_nodes.clone()
    }

    /// Returns the list of the predecessors of a node.
    ///
    /// # Arguments
    /// * `node_index` - The index of the node
    pub fn get_predecessors(&self, node_index: usize) -> Option<Vec<usize>> {
        if node_index < self.nodes.len() {
            Some(self.adj_list[node_index].1.clone())
        } else {
            None
        }
    }

    /// Returns the list of the successors of a node.
    ///
    /// # Arguments
    /// * `node_index` - The index of the node
    pub fn get_successors(&self, node_index: usize) -> Option<Vec<usize>> {
        if node_index < self.nodes.len() {
            Some(self.adj_list[node_index].0.clone())
        } else {
            None
        }
    }

    /// Returns the edges of the graph.
    pub fn get_edges(&self) -> HashMap<(usize, usize), Option<f64>> {
        self.edges.clone()
    }

    /// Returns the list of the nodes in the topological order.
    pub fn get_topological_order(&self) -> Vec<usize> {
        let mut top_ord = self.get_rev_topological_order();
        top_ord.reverse();

        top_ord
    }

    /// Returns the list of the nodes in the reverse topological order.
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

    /// This method is used by the topological sort.
    fn dfs(&self, node_index: usize, stack: &mut Vec<usize>, visited: &mut Vec<bool>) {
        visited[node_index] = true;

        for succ_idx in self.get_successors(node_index).unwrap() {
            if !visited[succ_idx] {
                self.dfs(succ_idx, stack, visited);
            }
        }

        stack.push(node_index);
    }

    /// Returns the `DspTask` associated with the node if there is one.
    ///
    /// # Arguments
    /// * `node_index` - The index of the node
    pub fn get_dsp(&self, node_index: usize) -> Arc<Mutex<Option<DspTask>>> {
        if node_index < self.nodes.len() {
            self.nodes[node_index].dsp_task.clone()
        } else {
            Arc::new(Mutex::new(None))
        }
    }

    /// Returns the estimated WCET of the node if there is one.
    ///
    /// # Arguments
    /// * `node_index` - The index of the node
    pub fn get_wcet(&mut self, node_index: usize) -> Option<f64> {
        if node_index < self.nodes.len() {
            self.nodes[node_index].get_wcet()
        } else {
            None
        }
    }

    /// Returns the state of the node if there is one.
    ///
    /// # Arguments
    /// * `node_index` - The index of the node
    pub fn get_state(&self, node_index: usize) -> Option<TaskState> {
        if node_index < self.nodes.len() {
            Some(self.nodes[node_index].state)
        } else {
            None
        }
    }

    /// Set the state of the node.
    ///
    /// # Arguments
    /// * `node_index` - The index of the node
    /// * `state` - The new state of the node
    pub fn set_state(&mut self, node_index: usize, state: TaskState) -> bool {
        if node_index < self.nodes.len() {
            self.nodes[node_index].state = state;
            true
        } else {
            false
        }
    }

    /// Decrements the activation count of a node and mark it as ready if it
    /// reaches 0.
    ///
    /// # Arguments
    /// * `node_index` - The index of the node
    pub fn dec_activation_count(&mut self, node_index: usize) -> bool {
        if node_index < self.nodes.len() {
            match self.nodes[node_index].state {
                TaskState::WaitingDependencies(count) => {
                    self.nodes[node_index].state = if count == 1 {
                        TaskState::Ready
                    } else {
                        TaskState::WaitingDependencies(count - 1)
                    };
                }
                _ => {}
            }

            true
        } else {
            false
        }
    }

    /// Returns the communication cost between two nodes.
    ///
    /// # Arguments
    /// * `src_node_index` - The index of the source node
    /// * `dst_node_index` - The index of the destination node
    pub fn get_communication_cost(
        &self,
        src_node_index: usize,
        dst_node_index: usize,
    ) -> Option<f64> {
        self.edges
            .get(&(src_node_index, dst_node_index))
            .map(|&cost| cost)
            .unwrap_or(None)
    }

    /// Returns the t-level of a node.
    ///
    /// # Arguments
    /// * `node_index` - The index of the node
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

            if i == node_index {
                break;
            }

            t_levels[i] = max;
        }

        t_levels.get(node_index).map(|val| *val)
    }

    /// Returns the b-level of a node.
    ///
    /// # Arguments
    /// * `node_index` - The index of the node
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

            if i == node_index {
                break;
            }
        }

        b_levels.get(node_index).map(|val| *val)
    }

    /// Returns the static level of a node.
    ///
    /// # Arguments
    /// * `node_index` - The index of the node
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

            if i == node_index {
                break;
            }
        }

        s_levels.get(node_index).map(|val| *val)
    }

    /// Adds a task to the graph and returns the index of its node.
    ///
    /// # Arguments
    /// * `task` - The task to add
    pub fn add_task(&mut self, task: Task) -> usize {
        self.nodes.push(Node::new(task));

        self.nodes.len() - 1
    }

    /// Adds a `DspTask` to the graph and returns the index of its node.
    ///
    /// # Arguments
    /// * `dsp` - The `DspTask` to add
    pub fn add_dsp(&mut self, dsp: DspTask) -> usize {
        self.nodes.push(Node::with_dsp(dsp));

        self.nodes.len() - 1
    }

    /// Returns `true` if the edge between the nodes has been added.
    ///
    /// # Arguments
    /// * `src_node_index` - The index of the source node
    /// * `dst_node_index` - The index of the destination node
    pub fn add_edge(&mut self, src_node_index: usize, dst_node_index: usize) -> bool {
        if src_node_index < self.nodes.len() && dst_node_index < self.nodes.len() {
            self.adj_list[src_node_index].0.push(dst_node_index);
            self.adj_list[dst_node_index].1.push(src_node_index);

            self.edges.insert((src_node_index, dst_node_index), None);

            true
        } else {
            false
        }
    }

    /// Sets the sample rate for the estimation of the WCETs.
    pub fn set_sample_rate(&mut self, sample_rate: usize) {
        self.sample_rate = Some(sample_rate);

        for ref mut node in self.nodes.iter_mut() {
            node.sample_rate = self.sample_rate;
        }
    }

    /// Sets the buffer size for the estimation of the WCETs.
    pub fn set_buffer_size(&mut self, buffer_size: usize) {
        self.buffer_size = Some(buffer_size);

        for ref mut node in self.nodes.iter_mut() {
            node.buffer_size = self.buffer_size;
        }
    }

    /// Writes the graph in the DOT format.
    ///
    /// # Arguments
    /// * `path` - The path of the file in which the DOT graph will be written
    pub fn write_dot(&self, path: &str) -> Result<(), std::io::Error> {
        let mut dot_file = String::new();

        dot_file.push_str("strict digraph{\n");

        for (i, node) in self.nodes.iter().enumerate() {
            let line = match (*node.dsp_task.lock().unwrap()).as_ref() {
                None => format!("{};\n", i),
                Some(dsp) => format!("{};\n", dsp.id),
            };

            dot_file.push_str(&line);
        }

        for ((s, t), _) in &self.edges {
            let line = if let (Some(src_dsp), Some(dst_dsp)) = (
                (*self.nodes[*s].dsp_task.lock().unwrap()).as_ref(),
                (*self.nodes[*t].dsp_task.lock().unwrap()).as_ref(),
            ) {
                format!("{} -> {};\n", src_dsp.id, dst_dsp.id)
            } else {
                format!("{} -> {};\n", s, t)
            };

            dot_file.push_str(line.as_str());
        }

        dot_file.push_str("}\n");

        let mut file = File::create(path).expect("Impossible to create file.");

        write!(file, "{}", dot_file)
    }

    /// Returns `true` if the schedule respects the dependencies of the nodes of the task graph.
    ///
    /// # Arguments
    /// * `schedule` - The schedule to check against the graph.
    pub fn is_valid_schedule(&mut self, schedule: &Schedule) -> bool {
        for node_index in self.get_topological_order() {
            match schedule.get_time_slot(node_index) {
                None => return false,
                Some(time_slot) => {
                    if let Some(wcet) = self.get_wcet(node_index) {
                        let time_slot_duration =
                            time_slot.get_completion_time() - time_slot.get_start_time();

                        if (time_slot_duration - wcet) >= 0.00004 {
                            return false;
                        }
                    }

                    for predecessor in self.get_predecessors(node_index).unwrap_or_default() {
                        if schedule.get_time_slot(predecessor).is_none() {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}

pub fn create_dot(graph: &TaskGraph, graph_name: &str) {
    let tmp_dot = format!("tmp/{}.dot", graph_name);

    println!("Creating tmp directory");
    if !Path::new("./tmp").exists() {
        std::fs::DirBuilder::new()
            .create("./tmp")
            .expect("failed to create tmp firectory");
    }

    println!("Writing the DOT file");
    graph
        .write_dot(tmp_dot.as_str())
        .unwrap_or_else(|e| panic!("failed to output graph: {}", e));

    let pdf_filename = format!("tmp/{}.pdf", graph_name);

    println!("Running dot");

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
