//! This module implements a schedule

use scheduling::processor::Processor;
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
        self.processors.push(Processor::new())
    }

    pub fn get_nb_processor(&self) -> usize {
        self.processors.len()
    }
}

impl Display for Schedule {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        for i in 0..self.processors.len() {
            writeln!(fmt, "processor {} || {}", i, self.processors[i]);
        }
        write!(fmt, "")
    }
}
