//! This module implements a schedule

use scheduling::processor::Processor;
use scheduling::timeslot::TimeSlot;

use std::fmt::{Display, Error, Formatter};

pub struct Schedule {
    pub processors: Vec<Processor>,
}

impl Schedule {
    pub fn new() -> Schedule {
        Schedule {
            processors: Vec::new(),
        }
    }
    pub fn add_processor(&mut self) {
        self.processors.push(Processor::new());
    }

    pub fn get_nb_processor(&self) -> usize {
        self.processors.len()
    }

    pub fn get_time_slot(&self, node_index: usize) -> Option<TimeSlot> {
        for procs in &self.processors {
            for ts in &procs.time_slots {
                if ts.get_node() == node_index {
                    return Some(*ts);
                }
            }
        }

        None
    }

    pub fn get_completion_time(&self) -> f64 {
        let mut time: f64 = 0.0;

        for processor in &self.processors {
            time = time.max(processor.get_completion_time());
        }

        time
    }
}

impl Display for Schedule {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "")?;

        for (i, processor) in self.processors.iter().enumerate() {
            writeln!(fmt, "processor {} * {}", i, processor)?;
        }

        Ok(())
    }
}
