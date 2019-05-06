//! This module implements a Schedule

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use scheduling::processor::Processor;
use scheduling::timeslot::TimeSlot;

use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Default)]
///Repressent the Schedule
pub struct Schedule {
    ///List of all the Processors Scheduled
    pub processors: Vec<Processor>,
}

impl Schedule {
    //Return an empty Schedule
    pub fn new() -> Schedule {
        Schedule::default()
    }

    ///Add an empty Processor and return the number of Processors Scheduled
    pub fn add_processor(&mut self) -> usize {
        self.processors.push(Processor::new());
        self.processors.len() - 1
    }

    //Return The number of Processors Scheduled
    pub fn get_nb_processor(&self) -> usize {
        self.processors.len()
    }

    ///Return the Timeslot , if any of the node with the earliest completion time
    /// 
    /// # Arguments
    /// * node_index - the Node Index to look for
    pub fn get_time_slot(&self, node_index: usize) -> Option<TimeSlot> {
        let mut output = None;
        for procs in &self.processors {
            for ts in &procs.time_slots {
                if ts.get_node() == node_index {
                    if output.is_none() {
                        output = Some(*ts);
                    } else {
                        if output.unwrap().get_completion_time() > ts.get_completion_time() {
                            output = Some(*ts);
                        }
                    }
                }
            }
        }

        output
    }

    ///Return The Completion time of all the Processors 
    pub fn get_completion_time(&self) -> f64 {
        let mut time: f64 = 0.0;

        for processor in &self.processors {
            time = time.max(processor.get_completion_time());
        }

        time
    }

    
    pub fn output(&self, ganttname: &str) -> Result<(), std::io::Error> {
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

        let tmp_dot = format!("tmp/{}.txt", ganttname);
        let path = Path::new(tmp_dot.as_str());

        if !Path::new("./tmp").exists() {
            std::fs::DirBuilder::new()
                .create("./tmp")
                .expect("failed to create tmp firectory");
        }

        let mut file = File::create(&path).expect("Impossible to create file.");

        write!(file, "{}", out_file)
    }

    ///Return the Timeslot, if any containing the predecessor with the last completion
    /// 
    /// # Arguments 
    /// 
    /// * predecessors - the list of Node Index to look for
    pub fn get_last_predecessor(&self, predecesssors: &Vec<usize>) -> Option<TimeSlot> {
        if predecesssors.is_empty() {
            return None;
        }

        let mut output = None;

        for pred in predecesssors {
            let pred_ts = self.get_time_slot(*pred);
            if pred_ts.is_none() {
                continue;
            }
            if output.is_none() {
                output = pred_ts;
            } else if output.unwrap().get_completion_time() < pred_ts.unwrap().get_completion_time()
            {
                output = pred_ts;
            }
        }

        output
    }

    ///Return the List of Processors that allocate the predecesssors 
    /// 
    ///# Argument
    /// * predecessors - List of Node Index to look for
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
