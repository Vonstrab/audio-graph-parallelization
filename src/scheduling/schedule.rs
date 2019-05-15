//! This module implements the representation of a static scheduling

use std::fmt::{Display, Error, Formatter};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use scheduling::processor::Processor;
use scheduling::timeslot::TimeSlot;

#[derive(Clone, Default)]
/// A list of the `Processor`s which will execute the tasks of the `Schedule`
pub struct Schedule {
    pub processors: Vec<Processor>,
}

impl Schedule {
    /// Returns an empty `Schedule`
    pub fn new() -> Schedule {
        Schedule::default()
    }

    /// Adds an empty `Processor` and returns the previous number of `Processor`s in the `Schedule`.
    pub fn add_processor(&mut self) -> usize {
        self.processors.push(Processor::new());
        self.processors.len() - 1
    }

    //Returns the number of `Processor`s in the `Schedule`.
    pub fn get_nb_processor(&self) -> usize {
        self.processors.len()
    }

    /// Returns the `TimeSlot`, of a `TaskGraph` node, with the earliest completion time.
    ///
    /// # Arguments
    /// * `node_index` - The index of the node
    pub fn get_time_slot(&self, node_index: usize) -> Option<TimeSlot> {
        let mut output: Option<TimeSlot> = None;

        for procs in &self.processors {
            for ts in &procs.time_slots {
                if ts.get_node() == node_index {
                    if output.is_none()
                        || output.unwrap().get_completion_time() > ts.get_completion_time()
                    {
                        output = Some(*ts);
                    }
                }
            }
        }

        output
    }

    /// Returns the completion time of the static scheduling.
    pub fn get_completion_time(&self) -> f64 {
        let mut time: f64 = 0.0;

        for processor in &self.processors {
            time = time.max(processor.get_completion_time());
        }

        time
    }

    pub fn output(&self, filename: &str) -> Result<(), std::io::Error> {
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

        if !Path::new("./tmp").exists() {
            std::fs::DirBuilder::new()
                .create("./tmp")
                .expect("failed to create tmp directory");
        }

        let path = format!("tmp/{}.txt", filename);
        let mut file = File::create(&path).expect("failed to create file");

        write!(file, "{}", out_file)
    }

    /// Returns the `TimeSlot`, containing one of the `predecessors`, with the latest completion time.
    ///
    /// # Arguments
    ///
    /// * `predecessors` - The list of the predecesssors
    pub fn get_last_predecessor(&self, predecesssors: &Vec<usize>) -> Option<TimeSlot> {
        if predecesssors.is_empty() {
            return None;
        }

        let mut output: Option<TimeSlot> = None;

        for pred in predecesssors {
            let pred_ts = self.get_time_slot(*pred);

            if pred_ts.is_none() {
                continue;
            }

            if output.is_none()
                || output.unwrap().get_completion_time() < pred_ts.unwrap().get_completion_time()
            {
                output = pred_ts;
            }
        }

        output
    }

    /// Returns the list of `Processor`s on which the `predecesssors` are scheduled
    ///
    /// # Argument
    /// * `predecessors` - The list of the indices of the predecessors
    pub fn get_p_set(&self, predecesssors: &Vec<usize>) -> Vec<usize> {
        let mut p_set = Vec::new();

        for (proc_index, processor) in self.processors.iter().enumerate() {
            if processor.contains_list_node(&predecesssors) {
                p_set.push(proc_index);
            }
        }

        p_set
    }
}

impl Display for Schedule {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt)?;

        for (i, processor) in self.processors.iter().enumerate() {
            writeln!(fmt, "processor {} * {}", i, processor)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod schedule_test {
    use super::*;

    #[test]
    fn test_constructor() {
        let sche = Schedule::new();

        assert_eq!(sche.processors.len(), 0);
    }

    #[test]
    fn test_add_processor() {
        let mut sche = Schedule::new();

        sche.add_processor();
        sche.add_processor();
        sche.add_processor();
        sche.add_processor();
        assert_eq!(sche.processors.len(), 4);
    }

    #[test]
    fn test_getters() {
        let mut sche = Schedule::new();

        sche.add_processor();
        sche.add_processor();
        sche.add_processor();
        sche.add_processor();
        assert_eq!(sche.get_nb_processor(), 4);

        sche.processors[3].add_timeslot(5, 1.0, 2.0);
        sche.processors[2].add_timeslot(7, 4.0, 5.0);
        assert_eq!(sche.get_completion_time(), 5.0);

        assert!(sche.get_time_slot(1).is_none());
        assert!(sche.get_time_slot(5) == Some(TimeSlot::new(5, 1.0, 2.0)));
        assert!(sche.get_time_slot(7) == Some(TimeSlot::new(7, 4.0, 5.0)));
    }
}
