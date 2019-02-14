//! This module implements a TimeSlot
//!

#[derive(Clone, Copy)]
pub struct Timeslot {
    start_time: f64,
    completion_time: f64,
    node: usize,
}

impl Timeslot {
    pub fn new(node: usize, start: f64, completion: f64) -> Timeslot {
        Timeslot {
            start_time: start,
            completion_time: completion,
            node: node,
        }
    }

    pub fn get_start(&self) -> f64 {
        self.start_time
    }

    pub fn get_completion(&self) -> f64 {
        self.completion_time
    }

    pub fn get_node(&self) -> usize {
        self.node
    }
}
