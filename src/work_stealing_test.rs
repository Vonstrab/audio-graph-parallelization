extern crate agp_lib;
extern crate crossbeam;

use std::sync::{Arc, RwLock};

use crossbeam::channel::unbounded;

use agp_lib::measure::Measure;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        panic!("No files supplied");
    }

    let dag = agp_lib::parser::audiograph::parser::actual_parse(&args[1])
        .expect("Failed to parse audio graph");

    let (tx, rx) = unbounded();
    let mut measure_thread = Measure::new(rx);

    std::thread::spawn(move || {
        measure_thread.receive();
    });

    match agp_lib::work_stealing::execution::run_work_stealing(Arc::new(RwLock::new(dag)), tx) {
        Ok(_) => {}
        e => {
            eprintln!("Failed to run because: {:?}", e);
        }
    }
}
