extern crate crossbeam;

extern crate agp_lib;

use crossbeam::channel::{unbounded, Sender};

use agp_lib::measure::{Measure, MeasureDestination};
use agp_lib::parser::audiograph::parser;
use agp_lib::static_scheduling::algorithms::{cpfd, etf, hlfet, random};

fn static_schedule_file(filepath: &str, tx: Sender<MeasureDestination>) {
    tx.send(MeasureDestination::Stdout(format!("File: {:?}", filepath)))
        .unwrap();

    tx.send(MeasureDestination::Stdout(String::from("Parsing")))
        .unwrap();

    let mut graph = parser::actual_parse(&filepath).expect("Failed parsing the audio graph\n");

    tx.send(MeasureDestination::Stdout(String::from(
        "\nComputing number of nodes",
    )))
    .unwrap();

    tx.send(MeasureDestination::Stdout(format!(
        "Number of nodes: {}",
        graph.get_topological_order().len()
    )))
    .unwrap();

    if graph.get_topological_order().len() < 50 {
        tx.send(MeasureDestination::Stdout(String::from(
            "\nOutput of the DOT representation in tmp/graph.got",
        )))
        .unwrap();
        agp_lib::task_graph::graph::create_dot(&graph, "graph");
    }

    let nb_procs = 9;
    println!("\nWith {} processors:", nb_procs);

    println!("\nComputation of ETF");

    let mut dur = std::time::SystemTime::now();;
    let etf_schedule = etf(&mut graph, nb_procs);

    println!(
        "EFT schedule time : {} s",
        etf_schedule.get_completion_time()
    );
    println!(
        "in: {}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );

    println!("\nComputation of RANDOM");

    dur = std::time::SystemTime::now();
    let random_schedule = random(&mut graph, nb_procs);
    println!(
        "Random schedule time: {} s",
        random_schedule.get_completion_time()
    );
    println!(
        "in: {}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );

    println!("\nCalcul of HLFET");

    dur = std::time::SystemTime::now();
    let hlfet_schedule = hlfet(&mut graph, nb_procs);

    println!(
        "HLFET schedule time: {} s",
        hlfet_schedule.get_completion_time()
    );

    println!(
        "in: {}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );

    println!("\nComputation of CPFD wihout communication costs");

    dur = std::time::SystemTime::now();

    let cpfd_schedule = cpfd(&mut graph, 0.0);

    println!(
        "CPFD without communication costs schedule time: {} s",
        cpfd_schedule.get_completion_time()
    );
    println!("with: {} processors", cpfd_schedule.processors.len());
    println!(
        "in: {}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );

    println!("\nComputation of CPFD cost = 1.0");

    dur = std::time::SystemTime::now();

    let cpfd_schedule = cpfd(&mut graph, 1.0);

    println!(
        "CPFD schedule time: {} s",
        cpfd_schedule.get_completion_time()
    );

    println!("with: {} processors", cpfd_schedule.processors.len());
    println!(
        "in: {}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        panic!("No files supplied");
    }

    let (tx, rx) = unbounded();

    let mut out_thread = Measure::new(rx);
    std::thread::spawn(move || {
        out_thread.receive();
    });

    static_schedule_file(&args[1], tx);
}
