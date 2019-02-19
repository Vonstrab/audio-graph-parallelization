use std::collections::HashMap;

use task_graph::{graph::TaskGraph, node::Node, state::TaskState};

use scheduling::{processor::Processor, schedule::Schedule, timeslot::TimeSlot};

//return the minimum value from a ready list
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

pub fn hlfet(graph: &mut TaskGraph, nb_processors: usize) -> Schedule {

    let mut out_schedule = Schedule::new();
    for _ in 0..nb_processors {
        out_schedule.add_processor();
    }

    //Nb : graph is of type <& mut TaskGraph> because
    //get_entry_node is on <& mut self>
    let first_nodes = graph.get_entry_nodes();

    let mut ready_list: HashMap<usize, f64> = HashMap::new();
    for node in first_nodes {
        ready_list.insert(node, graph.get_b_level(node).unwrap());
    }

    while !ready_list.is_empty() {
        let first_node = get_max_tie_misf(&mut ready_list, graph);

        let mut first_proc = 0;
        let mut first_start_time = out_schedule.processors[first_proc].get_completion_time();

        for i in 1..out_schedule.processors.len() {
            let current_start_time = out_schedule.processors[i].get_completion_time();
            if current_start_time < first_start_time {
                first_proc = i;
                first_start_time = current_start_time;
            }
        }

        out_schedule.processors[first_proc].add_timeslot(
            first_node,
            first_start_time,
            first_start_time + graph.get_wcet(first_node).unwrap(),
        );

        let successors = graph.get_successors(first_node);
        if successors != None {
            for node in successors.unwrap() {
                ready_list.insert(node, graph.get_b_level(node).unwrap());
            }
        }
        ready_list.remove(&first_node);
    }

    out_schedule
}
