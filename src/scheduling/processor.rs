//! This module implements a `Processor` which is a part of a static scheduling.

use scheduling::timeslot::TimeSlot;
use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Default)]
/// Ordered list of the tasks which will be executed on one `Processor`.
pub struct Processor {
    pub time_slots: Vec<TimeSlot>,
    completion_time: f64,
}

impl Processor {
    /// Returns an empty `Processor`.
    pub fn new() -> Processor {
        Processor::default()
    }

    /// Duplicates the Schedule on another Processor.
    ///
    /// # Arguments
    ///
    /// * `dup_proc` - The `Processor` to duplicate to
    pub fn duplication_from(&mut self, dup_proc: &Processor) {
        self.time_slots = dup_proc.time_slots.clone();
        self.completion_time = dup_proc.completion_time;

        // Check invariants
        debug_assert!(
            self.check_invariants(),
            "Processor::duplication_from: violated invariants"
        );

        // Check post-condition
        debug_assert!(
            (self.get_completion_time() - dup_proc.get_completion_time()).abs() < std::f64::EPSILON
                && self.time_slots == dup_proc.time_slots,
            "Processor::duplication_from: violated post-condition"
        );
    }

    /// Returns `true` if the `TimeSlot` has been correctly added to the `Processor`.
    ///
    /// # Arguments
    ///
    /// * `node` - The `TaskGraph` node to schedule
    /// * `start_time` - The time the `TimeSlot` begins
    /// * `completion_time` - The time the `TimeSlot` ends
    pub fn add_timeslot(&mut self, node: usize, start_time: f64, completion_time: f64) -> bool {
        // Check pre-condition
        debug_assert!(
            start_time >= 0.0 && completion_time >= 0.0,
            "Processor::add_timeslot: violated pre-condition"
        );

        // This condition expects us to always append a `TimeSlot`
        if self.completion_time <= start_time {
            self.time_slots
                .push(TimeSlot::new(node, start_time, completion_time));
            self.completion_time = completion_time;

            // Check invariants
            debug_assert!(
                self.check_invariants(),
                "Processor::add_timeslot: violated invariants"
            );

            // Check post-condition
            debug_assert!(
                (self.completion_time - completion_time).abs() < std::f64::EPSILON,
                "Processor::add_timeslot: violated post-condition"
            );
            return true;
        }

        false
    }

    /// Returns the completion time of the `Schedule`.
    pub fn get_completion_time(&self) -> f64 {
        self.completion_time
    }

    /// Returns `true` if the `TaskGraph` node is scheduled on this `Processor`.
    ///
    /// # Arguments
    ///
    /// * `node_index` - The index of the node
    pub fn contains_node(&self, node_index: usize) -> bool {
        for timeslot in &self.time_slots {
            if timeslot.get_node() == node_index {
                return true;
            }
        }

        false
    }

    /// Returns `true` if at least one of the `TaskGraph` nodes are scheduled on this `Processor`.
    ///
    /// # Arguments
    ///
    /// * `node_indices` - The list of node indices
    pub fn contains_list_node(&self, node_indices: &Vec<usize>) -> bool {
        node_indices
            .iter()
            .any(|&node_index| self.contains_node(node_index))
    }

    /// Returns `true` if every `TaskGraph` nodes are scheduled on this `Processor`.
    ///
    /// # Arguments
    ///
    /// * `node_indices` - The list of node indices
    pub fn contains_all_list_node(&self, node_indices: &Vec<usize>) -> bool {
        node_indices
            .iter()
            .all(|&node_index| self.contains_node(node_index))
    }

    /// Returns a list of the `TaskGraph` nodes which are **not** scheduled on this `Processor`.
    ///
    /// # Arguments
    ///
    /// * `node_indices` - The list of Node Index to look for
    pub fn nodes_not_in_proc(&self, node_indices: &Vec<usize>) -> Vec<usize> {
        node_indices
            .iter()
            .filter(|&&node_index| !self.contains_node(node_index))
            .map(|node_index| *node_index)
            .collect()
    }

    fn check_invariants(&self) -> bool {
        self.completion_time >= 0.0
    }
}

impl Display for Processor {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        for slot in 0..self.time_slots.len() {
            let spaces_count = if slot == 0 {
                (self.time_slots[slot].get_start_time() / 0.1) as usize
            } else {
                ((self.time_slots[slot].get_start_time()
                    - self.time_slots[slot - 1].get_completion_time())
                    / 0.1) as usize
            };

            let spaces = "_".repeat(spaces_count);

            write!(fmt, "{}{}", spaces, self.time_slots[slot])?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod processor_test {
    use super::*;

    #[test]
    fn test_constructor() {
        let pro = Processor::new();

        assert_eq!(pro.completion_time, 0.0);
        assert_eq!(pro.time_slots.len(), 0);
    }

    #[test]
    fn test_add_timeslot() {
        let mut pro = Processor::new();

        assert!(pro.add_timeslot(5, 1.0, 2.0));
        assert!(pro.add_timeslot(6, 2.5, 3.0));
        assert_eq!(pro.completion_time, 3.0);
        assert_eq!(pro.time_slots.len(), 2);
        assert!(pro.time_slots[0] == TimeSlot::new(5, 1.0, 2.0));
        assert!(pro.time_slots[1] == TimeSlot::new(6, 2.5, 3.0));
    }

    #[test]
    fn test_getter() {
        let mut pro = Processor::new();

        assert!(pro.add_timeslot(5, 1.0, 2.0));
        assert!(pro.add_timeslot(6, 2.5, 3.0));
        assert!(pro.add_timeslot(7, 3.5, 4.0));
        assert_eq!(pro.get_completion_time(), 4.0);
    }

}
