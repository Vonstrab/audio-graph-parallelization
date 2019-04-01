extern crate agp_lib;

use std::sync::{Arc, Mutex};

fn main() {
    let dag = agp_lib::parser::audiograph::parser::actual_parse("Samples/AG/seq_test.ag")
        .expect("Failed to parse audio graph");

    match agp_lib::execution::run_seq(Arc::new(Mutex::new(dag))) {
        Ok(_) => {}
        e => {
            eprintln!("Failed to run because: {:?}", e);
        }
    }
}
