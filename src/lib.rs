#![crate_type = "lib"]

pub mod puredata_parser;
pub mod audio_node;
pub mod audiograph_parser;
pub mod audiograph;

extern crate petgraph;

extern crate pest;
#[macro_use]
extern crate pest_derive;

extern crate itertools;