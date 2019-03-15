//! This module implements a TimeSlot
//!

use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Copy,PartialEq, Default)]
pub struct TimeSlot {
    start_time: f64,
    completion_time: f64,
    node: usize,
}

impl TimeSlot {
    pub fn new(node: usize, start: f64, completion: f64) -> TimeSlot {
        if start >= completion {
            panic!("TimeSlot::new() : completions < start")
        }
        TimeSlot {
            start_time: start,
            completion_time: completion,
            node,
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

#[cfg(test)]
mod timeslot_test {
    use super::*;

    #[test]
    fn test_constructor() {
        let ts = TimeSlot::new(5, 1.0, 2.0);

        assert_eq!(ts.completion_time, 2.0);
        assert_eq!(ts.start_time, 1.0);
        assert_eq!(ts.node, 5);
    }

    #[test]
    fn test_getters() {
        let ts = TimeSlot::new(5, 1.0, 2.0);

        assert_eq!(ts.get_completion_time(), 2.0);
        assert_eq!(ts.get_start_time(), 1.0);
        assert_eq!(ts.get_node(), 5);
    }
}
