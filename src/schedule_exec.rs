extern crate crossbeam;

extern crate libaudiograph;

use libaudiograph::parser::audiograph::parser;
use libaudiograph::static_scheduling::algorithms::{cpfd, etf, hlfet, random};
use libaudiograph::task_graph::graph::create_dot;

fn static_schedule_file(filepath: &str, nb_procs: usize) {
    println!("File: {:?}", filepath);

    println!("Parsing");

    let (client, _) = jack::Client::new(
        "audio_graph_static_sched",
        jack::ClientOptions::NO_START_SERVER,
    )
    .expect("jack connection error");

    let mut graph = parser::parse_audio_graph(&filepath).expect("Failed parsing the audio graph\n");

    graph.set_sample_rate(client.sample_rate());
    graph.set_buffer_size(client.buffer_size() as usize);

    if graph.get_topological_order().len() < 150 {
        println!("Output of the DOT representation in tmp/graph.got");
        create_dot(&graph, "graph");
    }

    println!("\nWith {} processors:", nb_procs);

    println!("\nComputation of ETF");

    let etf_schedule = etf(&mut graph, nb_procs);

    etf_schedule.output("etf").expect("error outpur etf");

    println!("\nComputation of RANDOM");

    let random_schedule = random(&mut graph, nb_procs);

    random_schedule
        .output("random")
        .expect("error outpur random");

    println!("\nCalcul of HLFET");

    let hlfet_schedule = hlfet(&mut graph, nb_procs);
    hlfet_schedule.output("hlfet").expect("error outpur hlfet");

    println!("\nComputation of CPFD wihout communication costs");

    let cpfd_schedule = cpfd(&mut graph, 0.0);
    cpfd_schedule.output("cpfd0").expect("error outpur cpfd0");

    println!("\nComputation of CPFD cost = 1.0");

    let cpfd_schedule = cpfd(&mut graph, 1.0);

    cpfd_schedule.output("cpfd1").expect("error outpur cpfd1");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("expected args : <File to schedule> <number of processors>");
        panic!("No files supplied");
    }

    static_schedule_file(&args[1], args[2].parse::<usize>().unwrap());
}
