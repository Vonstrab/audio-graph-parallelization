extern crate agp_lib;

use std::sync::{Arc, Mutex};

extern crate crossbeam;

use crossbeam::channel::unbounded;

use agp_lib::measure::Measure;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        panic!("No files supplied");
    }
    let dag = agp_lib::parser::audiograph::parser::actual_parse(&args[1])
        .expect("Failed to parse audio graph");

    agp_lib::task_graph::graph::run_dot(&dag, "seq");

    let (tx, rx) = unbounded();

    let mut out_thread = Measure::new(rx);
    std::thread::spawn(move || {
        out_thread.receive();
    });

    match agp_lib::execution::run_seq(Arc::new(Mutex::new(dag)), tx) {
        Ok(_) => {}
        e => {
            eprintln!("Failed to run because: {:?}", e);
        }
    }
}
