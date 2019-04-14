#![crate_type = "lib"]

extern crate core_affinity;
extern crate crossbeam;
extern crate jack;
extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod dsp;
pub mod execution;
pub mod parser;
pub mod scheduling;
pub mod task_graph;
pub mod work_stealing;
