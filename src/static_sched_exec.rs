extern crate crossbeam;

extern crate libaudiograph;

use std::sync::{Arc, RwLock};

use crossbeam::channel::unbounded;

use libaudiograph::execution::static_scheduling::run_static_sched;
use libaudiograph::measure::Measure;
use libaudiograph::parser::audiograph::parser::parse_audio_graph;
use libaudiograph::static_scheduling::algorithms::SchedulingAlgorithm;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        panic!("Usage: static_sched_exec <AG File> <Number of threads> <{rand, hlfet, etf}>");
    }

    let dag = parse_audio_graph(&args[1]).expect("Failed to parse audio graph");
    let nb_threads = args[2].parse().expect("Bad number of threads");

    let sched_algo = if args[3] == "rand" {
        SchedulingAlgorithm::Random
    } else if args[3] == "hlfet" {
        SchedulingAlgorithm::HLFET
    } else if args[3] == "etf" {
        SchedulingAlgorithm::ETF
    } else {
        panic!("There is no such scheduling algorithm");
    };

    let (tx, rx) = unbounded();
    let mut measure_thread = Measure::new(rx);

    std::thread::spawn(move || {
        measure_thread.receive();
    });

    match run_static_sched(Arc::new(RwLock::new(dag)), nb_threads, sched_algo, tx) {
        Ok(_) => {}
        e => {
            eprintln!("Failed to run because: {:?}", e);
        }
    }
}
