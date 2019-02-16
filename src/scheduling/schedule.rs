//! This module implements a schedule

use scheduling::processor::Processor;

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
        self.processors.push(Processor::new())
    }

    pub fn get_nb_processor(&self) -> usize {
        self.processors.len()
    }
}
