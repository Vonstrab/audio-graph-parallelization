use std::collections::HashMap;

use task_graph::{graph::TaskGraph, state::TaskState};

use scheduling::schedule::Schedule;

//Return the time when all the predecessors of the node
//will been complete
fn get_ready_time(node: usize, graph: &TaskGraph, sched: &Schedule) -> f64 {
    let predecessors = graph.get_predecessors(node).unwrap();
    let mut time: f64 = 0.0;

    for pred in predecessors {
        let pred_timeslot = sched.get_time_slot(pred).unwrap();
        if pred_timeslot.get_completion_time() > time {
            time = pred_timeslot.get_completion_time();
        }
    }

    time
}

//Set the status of all reachable nodes from the entry
// to TaskState::WaintingDependancies
fn set_status_waiting(graph: &mut TaskGraph) {
    let mut todo_nodes = graph.get_entry_nodes();

    while !todo_nodes.is_empty() {
        let node = todo_nodes[0];
        todo_nodes.remove(0);
        graph.set_state(node, TaskState::WaitingDependencies);
        for i in graph.get_successors(node).unwrap() {
            todo_nodes.push(i);
        }
    }
}

//Return True if all the pred are in state Ready
fn are_pred_ready(node: usize, graph: &TaskGraph) -> bool {
    let predecessors = graph.get_predecessors(node).unwrap();
    for pred in predecessors {
        if graph.get_state(pred).unwrap() != TaskState::Scheduled {
            return false;
        }
    }
    true
}

//Return the minimum value from a ready list
//ties broken by number of successors (most first)
fn get_max_tie_misf(ready_list: &HashMap<usize, f64>, ref graph: &TaskGraph) -> usize {
    let mut out_node: Option<usize> = None;

    for (node, b_level) in ready_list {
        if out_node == None {
            out_node = Some(*node);
        } else {
            if *b_level == *ready_list.get(&out_node.unwrap()).unwrap() {
                if graph.get_successors(*node) > graph.get_successors(out_node.unwrap()) {
                    out_node = Some(*node);
                }
            } else {
                if *b_level > *ready_list.get(&out_node.unwrap()).unwrap() {
                    out_node = Some(*node);
                }
            }
        }
    }

    out_node.unwrap()
}

pub fn random(graph: &mut TaskGraph, nb_processors: usize) -> Schedule {
    //We build the schedule
    let mut out_schedule = Schedule::new();
    for _ in 0..nb_processors {
        out_schedule.add_processor();
    }

    //we reset the status of all reachables nodes to Waitting
    set_status_waiting(graph);

    //the readylist
    let mut ready_list = graph.get_entry_nodes();

    //Main Loop
    while !ready_list.is_empty() {
        //We got a random node by b_level
        let rand_indice = rand::random::<usize>() % ready_list.len();
        let rand_node = ready_list[rand_indice];

        //we got a rand proc
        let rand_proc = rand::random::<usize>() % nb_processors;
        let rand_proc_start_time = out_schedule.processors[rand_proc].get_completion_time();

        //the start time of the node will be the the max
        //between the proc start time and the time where all the node
        //precursors will be completed(connextion time are overlooked)
        let node_start_time =
            rand_proc_start_time.max(get_ready_time(rand_node, &graph, &out_schedule));

        //we schedule the node
        out_schedule.processors[rand_proc].add_timeslot(
            rand_node,
            node_start_time,
            node_start_time + graph.get_wcet(rand_node).unwrap(),
        );
        graph.set_state(rand_node, TaskState::Scheduled);

        //we add the succesors if all theirs precursors are scheduled
        for node in graph.get_successors(rand_node).unwrap_or(Vec::default()) {
            if !ready_list.contains(&node) && are_pred_ready(node, &graph) {
                ready_list.push(node);
            }
        }

        //we remove the node
        ready_list.remove(rand_indice);
    }

    out_schedule
}

pub fn hlfet(graph: &mut TaskGraph, nb_processors: usize) -> Schedule {
    //We build the schedule
    let mut out_schedule = Schedule::new();
    for _ in 0..nb_processors {
        out_schedule.add_processor();
    }

    //we reset the status of all reachables nodes to Waitting
    set_status_waiting(graph);

    //the firsts nodes in the readylist
    let first_nodes = graph.get_entry_nodes();

    //the ready list is a Hasmap
    let mut ready_list: HashMap<usize, f64> = HashMap::new();
    for node in first_nodes {
        ready_list.insert(node, graph.get_b_level(node).unwrap());
    }

    //Main Loop
    while !ready_list.is_empty() {
        //We got the first node by b_level
        let first_node = get_max_tie_misf(&mut ready_list, graph);

        //we consider the first processor
        let mut chosen_proc = 0;
        let mut chosen_proc_start_time = out_schedule.processors[chosen_proc].get_completion_time();

        //if an another proc is best suited we chose it
        for i in 1..out_schedule.processors.len() {
            let current_proc_start_time = out_schedule.processors[i].get_completion_time();
            if current_proc_start_time < chosen_proc_start_time {
                chosen_proc = i;
                chosen_proc_start_time = current_proc_start_time;
            }
        }

        //the start time of the node will be the the max
        //between the proc start time and the time where all the node
        //precursors will be completed(connextion time are overlooked)
        let node_start_time =
            chosen_proc_start_time.max(get_ready_time(first_node, &graph, &out_schedule));

        //we schedule the node
        out_schedule.processors[chosen_proc].add_timeslot(
            first_node,
            node_start_time,
            node_start_time + graph.get_wcet(first_node).unwrap(),
        );
        graph.set_state(first_node, TaskState::Scheduled);

        //we add the succesors if all theirs precursors are scheduled
        for node in graph.get_successors(first_node).unwrap_or(Vec::default()) {
            if !ready_list.contains_key(&node) && are_pred_ready(node, &graph) {
                ready_list.insert(node, graph.get_b_level(node).unwrap());
            }
        }

        //we remove the node
        ready_list.remove(&first_node);
    }

    out_schedule
}

pub fn etf(graph: &mut TaskGraph, nb_processors: usize) -> Schedule {
    //We build the schedule
    let mut out_schedule = Schedule::new();
    for _ in 0..nb_processors {
        out_schedule.add_processor();
    }

    //we reset the status of all reachables nodes to Waitting
    set_status_waiting(graph);

    //the firsts nodes in the readylist
    let mut ready_list: Vec<usize> = Vec::from(graph.get_entry_nodes());

    //Main Loop
    while !ready_list.is_empty() {
        //we will chose the couple node-proc with the best start time
        let mut min_proc = None;
        let mut min_node: Option<usize> = None;
        let mut min_start_time = None;

        let mut node_indice: usize = 0;

        for i in 0..out_schedule.processors.len() {
            let proc_start_time = out_schedule.processors[i].get_completion_time();

            for j in 0..ready_list.len() {
                let current_node = *ready_list.get(j).unwrap();
                let current_blevel = graph.get_b_level(current_node).unwrap();
                let current_start_time =
                    proc_start_time.max(get_ready_time(current_node, &graph, &out_schedule));

                if min_start_time == None {
                    min_start_time = Some(current_start_time);
                    min_node = Some(current_node);
                    min_proc = Some(i);
                    node_indice = j;
                }

                if current_start_time == min_start_time.unwrap()
                    && graph.get_b_level(min_node.unwrap()).unwrap() < current_blevel
                {
                    min_start_time = Some(current_start_time);
                    min_node = Some(current_node);
                    min_proc = Some(i);
                    node_indice = j;
                }
                if current_start_time < min_start_time.unwrap() {
                    min_start_time = Some(current_start_time);
                    min_node = Some(current_node);
                    min_proc = Some(i);
                    node_indice = j;
                }
            }
        }

        let end_time = min_start_time.unwrap() + graph.get_wcet(node_indice).unwrap();

        out_schedule.processors[min_proc.unwrap()].add_timeslot(
            min_node.unwrap(),
            min_start_time.unwrap(),
            end_time,
        );

        graph.set_state(min_node.unwrap(), TaskState::Scheduled);

        let successors = graph
            .get_successors(min_node.unwrap())
            .unwrap_or(Vec::default());

        for node in successors {
            if !ready_list.contains(&node) && are_pred_ready(node, &graph) {
                ready_list.push(node);
            }
        }

        ready_list.remove(node_indice);
    }

    out_schedule
}

#[cfg(test)]
mod tests {
    use super::*;
    use task_graph::task::Task;

    #[test]
    fn test_hlfet() {
        let mut g = TaskGraph::new(8, 9);
        let mut nodes_idx = Vec::new();

        for i in 0..8 {
            nodes_idx.push(g.add_task(Task::A));
            g.set_wcet(i, 1.0);
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

        let sche_hlfelt = hlfet(&mut g, 2);
        let sche_rand = random(&mut g, 2);
        assert_eq!(sche_hlfelt.get_completion_time(), 5.0);
        assert!(sche_hlfelt.get_completion_time() <= sche_rand.get_completion_time());
    }

    #[test]
    fn test_eft() {
        let mut g = TaskGraph::new(8, 9);
        let mut nodes_idx = Vec::new();

        for i in 0..8 {
            nodes_idx.push(g.add_task(Task::A));
            g.set_wcet(i, 1.0);
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

        let sche_etf = etf(&mut g, 2);
        let sche_rand = random(&mut g, 2);
        assert_eq!(sche_etf.get_completion_time(), 5.0);
        assert!(sche_etf.get_completion_time() <= sche_rand.get_completion_time());
    }

    #[test]
    fn test_graph_8_node() {
        let mut g = TaskGraph::new(8, 9);
        let mut nodes_idx = Vec::new();

        for i in 0..8 {
            nodes_idx.push(g.add_task(Task::A));
            g.set_wcet(i, 1.0);
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

        let mut sche_etf = etf(&mut g, 2);
        let mut sche_hlfelt = hlfet(&mut g, 2);
        let mut sche_rand = random(&mut g, 2);
        assert!(sche_hlfelt.get_completion_time() <= sche_rand.get_completion_time());
        assert!(sche_etf.get_completion_time() <= sche_hlfelt.get_completion_time());

        sche_etf = etf(&mut g, 3);
        sche_hlfelt = hlfet(&mut g, 3);
        sche_rand = random(&mut g, 3);
        assert!(sche_hlfelt.get_completion_time() <= sche_rand.get_completion_time());
        assert!(sche_etf.get_completion_time() <= sche_hlfelt.get_completion_time());

        sche_etf = etf(&mut g, 4);
        sche_hlfelt = hlfet(&mut g, 4);
        sche_rand = random(&mut g, 4);
        assert!(sche_hlfelt.get_completion_time() <= sche_rand.get_completion_time());
        assert!(sche_etf.get_completion_time() <= sche_hlfelt.get_completion_time());
    }

    #[test]
    fn test_graph_24_node() {
        let mut g = TaskGraph::new(24, 21);
        let mut nodes_idx = Vec::new();

        for i in 0..24 {
            nodes_idx.push(g.add_task(Task::A));
            g.set_wcet(i, 1.0);
        }

        g.add_edge(0, 19);
        g.add_edge(1, 6);
        g.add_edge(1, 2);
        g.add_edge(2, 7);
        g.add_edge(3, 7);

        g.add_edge(4, 9);
        g.add_edge(5, 11);
        g.add_edge(6, 22);
        g.add_edge(6, 8);
        g.add_edge(7, 8);

        g.add_edge(7, 10);
        g.add_edge(8, 22);
        g.add_edge(8, 12);
        g.add_edge(9, 10);
        g.add_edge(10, 15);

        g.add_edge(10, 14);
        g.add_edge(10, 13);
        g.add_edge(11, 15);
        g.add_edge(11, 9);
        g.add_edge(12, 17);

        g.add_edge(12, 16);
        g.add_edge(13, 12);
        g.add_edge(14, 0);
        g.add_edge(14, 18);
        g.add_edge(16, 20);

        g.add_edge(17, 20);
        g.add_edge(17, 21);
        g.add_edge(18, 21);
        g.add_edge(18, 17);
        g.add_edge(18, 19);

        let mut sche_etf = etf(&mut g, 2);
        let mut sche_hlfelt = hlfet(&mut g, 2);
        let mut sche_rand = random(&mut g, 2);
        assert!(sche_hlfelt.get_completion_time() <= sche_rand.get_completion_time());
        assert!(sche_etf.get_completion_time() <= sche_hlfelt.get_completion_time());

        sche_etf = etf(&mut g, 3);
        sche_hlfelt = hlfet(&mut g, 3);
        sche_rand = random(&mut g, 3);
        assert!(sche_hlfelt.get_completion_time() <= sche_rand.get_completion_time());
        assert!(sche_etf.get_completion_time() <= sche_hlfelt.get_completion_time());

        sche_etf = etf(&mut g, 4);
        sche_hlfelt = hlfet(&mut g, 4);
        sche_rand = random(&mut g, 4);
        assert!(sche_hlfelt.get_completion_time() <= sche_rand.get_completion_time());
        assert!(sche_etf.get_completion_time() <= sche_hlfelt.get_completion_time());
    }

    #[test]
    fn test_graph_33_node() {
        let mut g = TaskGraph::new(33, 34);
        let mut nodes_idx = Vec::new();

        for i in 0..33 {
            nodes_idx.push(g.add_task(Task::A));
            g.set_wcet(i, 1.0);
        }

        g.add_edge(0, 6);
        g.add_edge(1, 8);
        g.add_edge(2, 8);
        g.add_edge(3, 9);
        g.add_edge(4, 10);

        g.add_edge(5, 11);
        g.add_edge(6, 17);
        g.add_edge(7, 16);
        g.add_edge(8, 15);
        g.add_edge(9, 14);

        g.add_edge(10, 13);
        g.add_edge(11, 12);
        g.add_edge(17, 19);
        g.add_edge(16, 20);
        g.add_edge(15, 20);

        g.add_edge(14, 21);
        g.add_edge(13, 21);
        g.add_edge(13, 22);
        g.add_edge(12, 22);
        g.add_edge(12, 23);

        g.add_edge(19, 24);
        g.add_edge(20, 24);
        g.add_edge(20, 25);
        g.add_edge(21, 25);
        g.add_edge(21, 26);

        g.add_edge(22, 26);
        g.add_edge(23, 26);
        g.add_edge(24, 27);
        g.add_edge(25, 29);
        g.add_edge(26, 29);

        g.add_edge(27, 28);
        g.add_edge(28, 31);
        g.add_edge(29, 30);
        g.add_edge(30, 32);

        let mut sche_etf = etf(&mut g, 3);
        let mut sche_hlfelt = hlfet(&mut g, 3);
        let mut sche_rand = random(&mut g, 3);
        assert!(sche_hlfelt.get_completion_time() <= sche_rand.get_completion_time());
        assert!(sche_etf.get_completion_time() <= sche_hlfelt.get_completion_time());

        sche_etf = etf(&mut g, 4);
        sche_hlfelt = hlfet(&mut g, 4);
        sche_rand = random(&mut g, 4);
        assert!(sche_hlfelt.get_completion_time() <= sche_rand.get_completion_time());
        assert!(sche_etf.get_completion_time() <= sche_hlfelt.get_completion_time());

        sche_etf = etf(&mut g, 5);
        sche_hlfelt = hlfet(&mut g, 5);
        sche_rand = random(&mut g, 5);
        assert!(sche_hlfelt.get_completion_time() <= sche_rand.get_completion_time());
        assert!(sche_etf.get_completion_time() <= sche_hlfelt.get_completion_time());

        sche_etf = etf(&mut g, 6);
        sche_hlfelt = hlfet(&mut g, 6);
        sche_rand = random(&mut g, 56);
        assert!(sche_hlfelt.get_completion_time() <= sche_rand.get_completion_time());
        assert!(sche_etf.get_completion_time() <= sche_hlfelt.get_completion_time());
    }

}
