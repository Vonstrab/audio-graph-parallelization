extern crate agp_lib;

use std::sync::{Arc, RwLock};

fn main() {
    let dag = agp_lib::parser::audiograph::parser::actual_parse("Samples/AG/work_stealing_test.ag")
        .expect("Failed to parse audio graph");

    match agp_lib::scheduling::execution::run_static_sched(Arc::new(RwLock::new(dag))) {
        Ok(_) => {}
        e => {
            eprintln!("Failed to run because: {:?}", e);
        }
    }
}
