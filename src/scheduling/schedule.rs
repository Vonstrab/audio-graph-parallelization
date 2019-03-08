//! This module implements a schedule

use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use scheduling::processor::Processor;
use scheduling::timeslot::TimeSlot;

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

    pub fn output(&self, filename: &str) {
        let mut out_file = String::new();
        for i in 0..self.processors.len() {
            for slot in &self.processors[i].time_slots {
                let ligne = format!(
                    "{} {} {}\n",
                    i,
                    slot.get_start_time(),
                    slot.get_completion_time()
                );
                out_file.push_str(ligne.as_str());
            }
        }
        let path = Path::new(filename);
        let mut file = File::create(&path).expect("Impossible to create file.");
        let _result = write!(file, "{}", out_file);
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
