extern crate agp_lib;
extern crate crossbeam;

use std::path::Path;
use std::sync::{Arc, Mutex};

use crossbeam::channel::unbounded;

use agp_lib::measure::Measure;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        panic!("No files supplied");
    }

    let dag = agp_lib::parser::audiograph::parser::actual_parse(&args[1])
        .expect("Failed to parse audio graph");

    let dag_name = Path::new(&args[1])
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .trim_end_matches(".ag");

    agp_lib::task_graph::graph::create_dot(&dag, dag_name);

    let (tx, rx) = unbounded();
    let mut measure_thread = Measure::new(rx);

    std::thread::spawn(move || {
        measure_thread.receive();
    });

    match agp_lib::execution::sequential::run_seq(Arc::new(Mutex::new(dag)), tx) {
        Ok(_) => {}
        e => {
            eprintln!("Failed to run because: {:?}", e);
        }
    }
}
