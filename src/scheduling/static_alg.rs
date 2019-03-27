use std::collections::HashMap;

use task_graph::{graph::TaskGraph, state::TaskState};

use scheduling::processor::Processor;
use scheduling::schedule::Schedule;

//return the cpn dominant sequence
fn get_cpn_dominant_sequence(graph: &mut TaskGraph) -> Vec<usize> {
    //add the parents list to the sequence
    fn add_node(graph: &mut TaskGraph, sequence: &mut Vec<usize>, node: usize) {
        let mut not_in_seq_parents = Vec::new();

        for pre in graph.get_predecessors(node).unwrap_or_default() {
            if !sequence.contains(&pre) {
                not_in_seq_parents.push(pre);
            }
        }

        if not_in_seq_parents.is_empty() {
            sequence.push(node);
        } else {
            let mut max_parent = not_in_seq_parents[0];
            let mut max_parent_blevel = graph.get_b_level(max_parent);
            for parent in not_in_seq_parents {
                let blevel = graph.get_b_level(parent);
                if blevel < max_parent_blevel {
                    max_parent = parent;
                    max_parent_blevel = blevel
                }
            }
            add_node(graph, sequence, max_parent);
            add_node(graph, sequence, node);
        }
    }

    let mut sequence = Vec::new(); //graph.get_entry_nodes();
    let sortie_nodes = graph.get_exit_nodes();

    for cp in sortie_nodes {
        add_node(graph, &mut sequence, cp)
    }

    for node in graph.get_topological_order() {
        if !sequence.contains(&node) {
            sequence.push(node);
        }
    }

    sequence
}

// Returns the time when all the predecessors of the node
// will been complete
fn get_ready_time(node: usize, graph: &TaskGraph, sched: &Schedule) -> f64 {
    let predecessors = graph.get_predecessors(node).unwrap();

    let time = sched
        .get_last_predecessor(&predecessors)
        .unwrap_or_default()
        .get_completion_time();

    time
}

// Sets the status of all reachable nodes from the entry
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

// Returns true if all predecessors are in the state Ready
fn predecessors_scheduled(node: usize, graph: &TaskGraph) -> bool {
    graph
        .get_predecessors(node)
        .unwrap()
        .iter()
        .all(|pred| graph.get_state(*pred).unwrap() == TaskState::Scheduled)
}

//return the best Processor possible using the duplication method
fn optimal_proc(
    graph: &mut TaskGraph,
    control: &Processor,
    candidate: usize,
    communication_cost: f64,
    schedule: &Schedule,
) -> Processor {
    let mut duplicate_proc = Processor::new();
    duplicate_proc.duplication_from(control);

    let mut start_time =
        get_ready_time(candidate, graph, schedule).max(control.get_completion_time());
    let predecessors = graph.get_predecessors(candidate).unwrap_or_default();

    if predecessors.is_empty() {
        duplicate_proc.add_timeslot(
            candidate,
            start_time,
            start_time + graph.get_wcet(candidate).unwrap(),
        );
    } else {
        if !duplicate_proc.contains_all_list_node(&predecessors) {
            start_time = (get_ready_time(candidate, graph, schedule) + communication_cost)
                .max(control.get_completion_time());

            for not_in_proc_pred in duplicate_proc.nodes_not_in_proc(&predecessors) {
                duplicate_proc = optimal_proc(
                    graph,
                    &duplicate_proc,
                    not_in_proc_pred,
                    communication_cost,
                    schedule,
                );
                let pred_dup_start_time = duplicate_proc.get_completion_time();

                if pred_dup_start_time > start_time {
                    duplicate_proc.duplication_from(control);
                } else {
                    start_time = pred_dup_start_time;
                }
            }
        }
    }

    duplicate_proc.add_timeslot(
        candidate,
        start_time,
        start_time + graph.get_wcet(candidate).unwrap(),
    );

    duplicate_proc
}

//Return the minimum value from a ready list
//ties broken by number of successors (most first)
fn get_max_tie_misf(ready_list: &HashMap<usize, f64>, graph: &TaskGraph) -> usize {
    let mut out_node: Option<usize> = None;

    for (node, b_level) in ready_list {
        if out_node.is_none() {
            out_node = Some(*node);
        } else if (*b_level - ready_list[&out_node.unwrap()]).abs() < std::f64::EPSILON {
            //Not stric comparaison, but within error margin
            if graph.get_successors(*node) > graph.get_successors(out_node.unwrap()) {
                out_node = Some(*node);
            }
        } else if *b_level > *ready_list.get(&out_node.unwrap()).unwrap() {
            out_node = Some(*node);
        }
    }

    out_node.unwrap()
}

pub fn random(graph: &mut TaskGraph, nb_processors: usize) -> Schedule {
    // Build the schedule
    let mut out_schedule = Schedule::new();
    for _ in 0..nb_processors {
        out_schedule.add_processor();
    }

    // Reset the status of all reachable nodes to `WaitingDependencies`
    set_status_waiting(graph);

    // The readylist
    let mut ready_list = graph.get_entry_nodes();

    // Main Loop
    while !ready_list.is_empty() {
        // Get a random node
        let rand_indice = rand::random::<usize>() % ready_list.len();
        let rand_node = ready_list[rand_indice];

        // Get a random processor
        let rand_proc = rand::random::<usize>() % nb_processors;
        let rand_proc_start_time = out_schedule.processors[rand_proc].get_completion_time();

        // The start time of the node will be the the maximum
        // between the processor's start time and the time when all the node's
        // parents will be completed (connexion time are overlooked).
        let node_start_time =
            rand_proc_start_time.max(get_ready_time(rand_node, &graph, &out_schedule));

        // Schedule the node
        out_schedule.processors[rand_proc].add_timeslot(
            rand_node,
            node_start_time,
            node_start_time + graph.get_wcet(rand_node).unwrap(),
        );

        graph.set_state(rand_node, TaskState::Scheduled);

        // Add successors whose all parents have been scheduled
        for node in graph.get_successors(rand_node).unwrap_or_default() {
            if !ready_list.contains(&node) && predecessors_scheduled(node, &graph) {
                ready_list.push(node);
            }
        }

        // Remove the node
        ready_list.remove(rand_indice);
    }

    out_schedule
}

pub fn hlfet(graph: &mut TaskGraph, nb_processors: usize) -> Schedule {
    // Build the schedule
    let mut out_schedule = Schedule::new();
    for _ in 0..nb_processors {
        out_schedule.add_processor();
    }

    // Reset the status of all reachable nodes to `WaitingDependencies`
    set_status_waiting(graph);

    // The firsts nodes in the readylist
    let first_nodes = graph.get_entry_nodes();

    // The ready list is a HashMap
    let mut ready_list: HashMap<usize, f64> = HashMap::new();
    for node in first_nodes {
        ready_list.insert(node, graph.get_b_level(node).unwrap());
    }

    // Main Loop
    while !ready_list.is_empty() {
        // Get the first node by b_level
        let first_node = get_max_tie_misf(&ready_list, graph);

        //First consider the first processor
        let mut chosen_proc = 0;
        let mut chosen_proc_start_time = out_schedule.processors[chosen_proc].get_completion_time();

        // Choose another processor if it is better suited
        for i in 1..out_schedule.processors.len() {
            let current_proc_start_time = out_schedule.processors[i].get_completion_time();
            if current_proc_start_time < chosen_proc_start_time {
                chosen_proc = i;
                chosen_proc_start_time = current_proc_start_time;
            }
        }

        // The start time of the node will be the the maximum
        // between the processor's start time and the time when all the node's
        // parents will be completed (connexion time are overlooked).
        let node_start_time =
            chosen_proc_start_time.max(get_ready_time(first_node, &graph, &out_schedule));

        // Schedule the node
        out_schedule.processors[chosen_proc].add_timeslot(
            first_node,
            node_start_time,
            node_start_time + graph.get_wcet(first_node).unwrap(),
        );
        graph.set_state(first_node, TaskState::Scheduled);

        // Add the successors if all theirs predecessors are scheduled
        for node in graph.get_successors(first_node).unwrap_or_default() {
            if !ready_list.contains_key(&node) && predecessors_scheduled(node, &graph) {
                ready_list.insert(node, graph.get_b_level(node).unwrap());
            }
        }

        // Remove the node
        ready_list.remove(&first_node);
    }

    out_schedule
}

pub fn etf(graph: &mut TaskGraph, nb_processors: usize) -> Schedule {
    // Build the schedule
    let mut out_schedule = Schedule::new();
    for _ in 0..nb_processors {
        out_schedule.add_processor();
    }

    // Reset the status of all reachable nodes to `WaitingDependencies`
    set_status_waiting(graph);

    // The firsts nodes in the readylist
    let mut ready_list: Vec<usize> = graph.get_entry_nodes();

    // Main loop
    while !ready_list.is_empty() {
        // Choose the couple node-proc with the best start time
        let mut min_proc = None;
        let mut min_node: Option<usize> = None;
        let mut min_start_time = None;

        let mut node_index: usize = 0;

        for i in 0..out_schedule.processors.len() {
            let proc_start_time = out_schedule.processors[i].get_completion_time();

            for j in 0..ready_list.len() {
                let current_node = ready_list[j];
                let current_blevel = graph.get_b_level(current_node).unwrap();
                let current_start_time =
                    proc_start_time.max(get_ready_time(current_node, &graph, &out_schedule));

                if min_start_time.is_none() {
                    min_start_time = Some(current_start_time);
                    min_node = Some(current_node);
                    min_proc = Some(i);
                    node_index = j;
                }
                //Not stric comparaison, but within error margin
                if (current_start_time - min_start_time.unwrap()).abs() < std::f64::EPSILON
                    && graph.get_b_level(min_node.unwrap()).unwrap() < current_blevel
                {
                    min_start_time = Some(current_start_time);
                    min_node = Some(current_node);
                    min_proc = Some(i);
                    node_index = j;
                }
                if current_start_time < min_start_time.unwrap() {
                    min_start_time = Some(current_start_time);
                    min_node = Some(current_node);
                    min_proc = Some(i);
                    node_index = j;
                }
            }
        }

        let end_time = min_start_time.unwrap() + graph.get_wcet(ready_list[node_index]).unwrap();

        out_schedule.processors[min_proc.unwrap()].add_timeslot(
            min_node.unwrap(),
            min_start_time.unwrap(),
            end_time,
        );

        graph.set_state(min_node.unwrap(), TaskState::Scheduled);

        let successors = graph.get_successors(min_node.unwrap()).unwrap_or_default();

        for node in successors {
            if !ready_list.contains(&node) && predecessors_scheduled(node, &graph) {
                ready_list.push(node);
            }
        }

        ready_list.remove(node_index);
    }

    out_schedule
}

pub fn cpfd(graph: &mut TaskGraph, communication_cost: f64) -> Schedule {
    //initialise the schedule
    let mut out_schedule = Schedule::new();

    // Reset the status of all reachable nodes to `WaitingDependencies`
    set_status_waiting(graph);

    //the cpn_dominant sequence
    let cpn_sequence: Vec<usize> = get_cpn_dominant_sequence(graph);

    // println!("CPN Dominant sequence {:?}", cpn_sequence);
    // println!("CPN Dominant sequence size {}", cpn_sequence.len());

    for candidate in cpn_sequence {
        let pred = graph.get_predecessors(candidate).unwrap_or_default();
        //construction of the p_set
        let p_set = out_schedule.get_p_set(&pred);

        // println!("\nschedule {}", out_schedule);

        // println!("\ncandidate {}", candidate);
        // println!("\npred {:?}", pred);
        // println!("p_set {:?}", p_set);

        //test with an empty proc
        let empty_proc = optimal_proc(
            graph,
            &Processor::new(),
            candidate,
            communication_cost,
            &out_schedule,
        );

        let empty_proc_et = empty_proc.get_completion_time();

        // println!("empty_proc_et {:?}", empty_proc_et);
        // println!("empty_proc {}", empty_proc);

        let mut et = None;
        let mut proce = 0;
        let mut best_proc: Processor = Processor::default();

        for p in p_set {
            //best configuration with current proc
            let p_proc = optimal_proc(
                graph,
                &out_schedule.processors[p],
                candidate,
                communication_cost,
                &out_schedule,
            );
            if et.is_none() {
                et = Some(best_proc.get_completion_time());
                proce = p;
                best_proc = p_proc;
            } else if best_proc.get_completion_time() < et.unwrap() {
                et = Some(out_schedule.processors[p].get_completion_time());
                proce = p;
                best_proc = p_proc;
            }
        }

        // println!("et {:?}", et);
        // println!("proce {:?}", proce);

        //if only the empty_proc
        if et.is_none() {
            out_schedule.processors.push(empty_proc.clone());
        } else {
            //else we get the best
            //we test  <= because we benefit the duplications on an empty proc
            if (empty_proc_et - et.unwrap()) > 0.5 {
                out_schedule.processors.push(empty_proc);
            } else {
                out_schedule.processors[proce].duplication_from(&best_proc);
            }
        }
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

        let sche_hlfelt = hlfet(&mut g, 2);
        let sche_rand = random(&mut g, 2);
        assert_eq!(sche_hlfelt.get_completion_time(), 5.0);
        assert!(sche_hlfelt.get_completion_time() <= sche_rand.get_completion_time());
    }

    #[test]
    fn test_eft() {
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

        let sche_etf = etf(&mut g, 2);
        let sche_rand = random(&mut g, 2);
        assert_eq!(sche_etf.get_completion_time(), 5.0);
        assert!(sche_etf.get_completion_time() <= sche_rand.get_completion_time());
    }

    #[test]
     fn test_cpdf() {
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

        let sche_cpfd = cpfd(&mut g, 0.0);
        assert_eq!(sche_cpfd.get_completion_time(), 5.0);
    }

    #[test]
    fn test_graph_8_node() {
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

        for _ in 0..24 {
            nodes_idx.push(g.add_task(Task::Constant(1.0)));
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

        for _ in 0..33 {
            nodes_idx.push(g.add_task(Task::Constant(1.0)));
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
