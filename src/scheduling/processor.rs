//! This module implement the Schedule on one Processor

use scheduling::timeslot::TimeSlot;
use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Default)]
///Repressent the schedule on one Processor
pub struct Processor {
    ///The List of TImeSlots scheduled on the Processors
    pub time_slots: Vec<TimeSlot>,
    completion_time: f64,
}

impl Processor {
    ///Returns an empty Processor
    pub fn new() -> Processor {
        Processor::default()
    }

    ///Duplicate the Schedule on another Processor
    ///
    ///# Arguments
    /// * dup_proc - The Processor to duplicate into
    pub fn duplication_from(&mut self, dup_proc: &Processor) {
        self.time_slots = dup_proc.time_slots.clone();
        self.completion_time = dup_proc.completion_time;
        //check invariant
        debug_assert!(
            self.check_invariants(),
            "Processor::duplication_from : Invariant Error"
        );
        //check post-condition
        debug_assert!(
            (self.get_completion_time() - dup_proc.get_completion_time()).abs() < std::f64::EPSILON
                && self.time_slots == dup_proc.time_slots,
            "Processor::duplication_from : post-condition Error"
        );
    }

    ///Return true if the TimeSlot is correctly added on the Processor
    ///
    /// # Arguments
    ///
    /// * node - The TaskGraph Node scheduled
    /// * start_time - The Start Time of the TimeSLot
    /// * Completion_time - The Completion TimeSlot
    pub fn add_timeslot(&mut self, node: usize, start_time: f64, completion_time: f64) -> bool {
        //check pre-condition
        debug_assert!(
            start_time >= 0.0 && completion_time >= 0.0,
            "Processor::add_timeslot : pre-condition Error"
        );

        // This condition expects we always append a TimeSlot
        if self.completion_time <= start_time {
            self.time_slots
                .push(TimeSlot::new(node, start_time, completion_time));
            self.completion_time = completion_time;

            //check invariant
            debug_assert!(
                self.check_invariants(),
                "Processor::add_timeslot : Invariant Error"
            );

            //check post-condition
            debug_assert!(
                (self.completion_time - completion_time).abs() < std::f64::EPSILON,
                "Processor::add_timeslot : post-condition Error"
            );
            return true;
        }

        false
    }

    ///Return the Completion Time of the Schedule

    pub fn get_completion_time(&self) -> f64 {
        self.completion_time
    }

    ///Return true if the Processor allocate a certain TaskGraph Node
    ///
    /// # Arguments
    ///
    /// * node_index - The Node Index to look for  
    pub fn contains_node(&self, node_index: usize) -> bool {
        for timeslot in &self.time_slots {
            if timeslot.get_node() == node_index {
                return true;
            }
        }
        false
    }

    ///Return true if the Processor allocate ONE of the TaskGraph Nodes
    ///
    /// # Arguments
    ///
    /// * list_node_index - The list of Node Index to look for
    pub fn contains_list_node(&self, list_node_index: &Vec<usize>) -> bool {
        for node in list_node_index {
            if self.contains_node(*node) {
                return true;
            }
        }

        false
    }

    ///Return true if the Processor allocate ALL of the TaskGraph Nodes
    ///
    /// # Arguments
    ///
    /// * list_node_index - The list of Node Index to look for
    pub fn contains_all_list_node(&self, list_node_index: &Vec<usize>) -> bool {
        for node in list_node_index {
            if !self.contains_node(*node) {
                return false;
            }
        }

        true
    }

    ///Return the list of all the ThaskGraph Nodes not in the processor
    ///
    /// # Arguments
    ///
    /// * list_node_index - The list of Node Index to look for
    pub fn nodes_not_in_proc(&self, list_node_index: &Vec<usize>) -> Vec<usize> {
        let mut output = Vec::new();
        for node in list_node_index {
            if !self.contains_node(*node) {
                output.push(*node);
            }
        }

        output
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
