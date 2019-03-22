//! This module implements a schedule

use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use scheduling::processor::Processor;
use scheduling::timeslot::TimeSlot;

use task_graph::graph::TaskGraph;

pub struct Schedule {
    pub processors: Vec<Processor>,
}

impl Schedule {
    pub fn new() -> Schedule {
        Schedule {
            processors: Vec::new(),
        }
    }

    pub fn add_processor(&mut self) -> usize {
        self.processors.push(Processor::new());
        self.processors.len() - 1
    }

    pub fn get_nb_processor(&self) -> usize {
        self.processors.len()
    }

    //return the timeslot of the node with the earliest completion time
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


    //return the timeslot if any containing the predecessor with the last completion
    pub fn get_last_predecessor(&self, predecesssors: &Vec<usize>) -> Option<TimeSlot> {
        if predecesssors.is_empty() {
            return None;
        }

        let mut output = None;

        for pred in predecesssors {
            let pred_ts = self.get_time_slot(*pred);
            if output.is_none() {
                output = pred_ts;
            } else if output.unwrap().get_completion_time() < pred_ts.unwrap().get_completion_time()
            {
                output = pred_ts;
            }
        }

        output
    }

    //return the set of processors that allocate the predecesssors 
    pub fn get_p_set(&mut self, predecesssors: &Vec<usize>, node_index: usize) -> Vec<usize> {
        let mut p_set = Vec::new();
        for (proc_index, processor) in self.processors.iter().enumerate() {
            if processor.contains_list_node(&predecesssors) {
                p_set.push(proc_index);
            }
        }
        p_set
    }
}

impl fmt::Display for Schedule {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(fmt, "")?;

        for (i, processor) in self.processors.iter().enumerate() {
            writeln!(fmt, "processor {} * {}", i, processor)?;
        }

        Ok(())
    }
}
