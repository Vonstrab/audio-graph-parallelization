extern crate agp_lib;
extern crate crossbeam;

use std::sync::{Arc, RwLock};

use crossbeam::channel::unbounded;

use agp_lib::measure::Measure;
use agp_lib::scheduling::static_alg::SchedulingAlgorithm;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        panic!("Usage: static_sched_test <AG File> <{rand, hlfet, etf}>");
    }

    let dag = agp_lib::parser::audiograph::parser::actual_parse(&args[1])
        .expect("Failed to parse audio graph");

    let sched_algo = if args[2] == "rand" {
        SchedulingAlgorithm::Random
    } else if args[2] == "hlfet" {
        SchedulingAlgorithm::HLFET
    } else if args[2] == "etf" {
        SchedulingAlgorithm::ETF
    } else {
        panic!("There is no such scheduling algorithm");
    };

    let (tx, rx) = unbounded();
    let mut measure_thread = Measure::new(rx);

    std::thread::spawn(move || {
        measure_thread.receive();
    });

    match agp_lib::execution::static_scheduling::run_static_sched(
        Arc::new(RwLock::new(dag)),
        sched_algo,
        tx,
    ) {
        Ok(_) => {}
        e => {
            eprintln!("Failed to run because: {:?}", e);
        }
    }
}
