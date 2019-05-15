#![crate_type = "lib"]

extern crate core_affinity;
extern crate crossbeam;
extern crate jack;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate rand;

pub mod dsp;
pub mod execution;
pub mod measure;
pub mod parser;
pub mod static_scheduling;
pub mod task_graph;
