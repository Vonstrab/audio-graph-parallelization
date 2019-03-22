use scheduling::timeslot::TimeSlot;
use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Default)]
pub struct Processor {
    pub time_slots: Vec<TimeSlot>,
    completion_time: f64,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            time_slots: Vec::new(),
            completion_time: 0.0,
        }
    }

    //duplcate from a Processor
    pub fn duplication_from(&mut self, dup_proc: Processor) {
        self.time_slots = dup_proc.time_slots.clone();
        self.completion_time = dup_proc.completion_time;
    }

    pub fn add_timeslot(&mut self, node: usize, start_time: f64, completion_time: f64) -> bool {
        // This condition expects we always append a TimeSlot
        if self.completion_time <= start_time {
            self.time_slots
                .push(TimeSlot::new(node, start_time, completion_time));
            self.completion_time = completion_time;
            return true;
        }

        return false;
    }

    pub fn get_completion_time(&self) -> f64 {
        self.completion_time
    }

    //true if its allocate a certain node
    pub fn contains_node(&self, node_index: usize) -> bool {
        for timeslot in &self.time_slots {
            if timeslot.get_node() == node_index {
                return true;
            }
        }
        false
    }

    //true if its allocate all nodes
    pub fn contains_list_node(&self, list_node_index: &Vec<usize>) -> bool {
        for node in list_node_index {
            if !self.contains_node(*node) {
                return false;
            }
        }

        true
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
