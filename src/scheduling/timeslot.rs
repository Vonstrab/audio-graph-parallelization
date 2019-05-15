//! This module implements a `TimeSlot` which is a part of a static scheduling.

use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Copy, PartialEq, Default)]
pub struct TimeSlot {
    start_time: f64,
    completion_time: f64,
    node: usize,
}

impl TimeSlot {
    /// Creates a new `TimeSlot` for a task.
    pub fn new(node: usize, start_time: f64, completion_time: f64) -> TimeSlot {
        // Check pre-condition
        debug_assert!(start_time < completion_time, "TimeSlot::new() : completions < start");

        let ts = TimeSlot {
            start_time: start_time,
            completion_time: completion_time,
            node,
        };

        // Check invariants
        debug_assert!(ts.check_invariants(), "TimeSlot::new() : Invariants Error");

        ts
    }

    /// Returns the start time of the `TimeSlot`.
    pub fn get_start_time(&self) -> f64 {
        self.start_time
    }

    /// Returns the completion time of the `TimeSlot`.
    pub fn get_completion_time(&self) -> f64 {
        self.completion_time
    }

    /// Returns the index of the task's node.
    pub fn get_node(&self) -> usize {
        self.node
    }

    fn check_invariants(&self) -> bool {
        self.start_time >= 0.0
            && self.completion_time >= 0.0
            && self.start_time <= self.completion_time
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
