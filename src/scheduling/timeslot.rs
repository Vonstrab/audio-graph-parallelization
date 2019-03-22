//! This module implements a TimeSlot
//!

use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Copy, Default)]
pub struct TimeSlot {
    start_time: f64,
    completion_time: f64,
    node: usize,
}

impl TimeSlot {
    pub fn new(node: usize, start: f64, completion: f64) -> TimeSlot {
        TimeSlot {
            start_time: start,
            completion_time: completion,
            node: node,
        }
    }

    pub fn get_start_time(&self) -> f64 {
        self.start_time
    }

    pub fn get_completion_time(&self) -> f64 {
        self.completion_time
    }

    pub fn get_node(&self) -> usize {
        self.node
    }
}

impl Display for TimeSlot {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(
            fmt,
            "|{:.*} No:{} {:.*}|",
            2, self.start_time, self.node, 2, self.completion_time
        )
    }
}
