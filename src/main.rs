extern crate agp_lib;

use agp_lib::parser::audiograph::parser;

use agp_lib::scheduling::static_alg::*;

pub fn static_schedule_file(filepath: &str) {
    println!("File : {:?}", filepath);

    println!("Parsing");

    let mut graph = parser::actual_parse(&filepath).expect("Failed parsing the audio graph\n");

    println!("\nCalcul of nodes number");
    println!("Number of nodes: {}", graph.get_topological_order().len());
    if graph.get_topological_order().len() < 50 {
        println!("\nOutpout the dot representation in tmp/graph.dot");
        agp_lib::task_graph::graph::run_dot(&graph, "graph");
    }
    let nb_procs = 9;
    println!("\nWith {} processors:", nb_procs);

    println!("\nCalcul of ETF");

    let mut dur = std::time::SystemTime::now();;
    let etf_schedule = etf(&mut graph, nb_procs);

    println!(
        "EFT schedule time : {} s",
        etf_schedule.get_completion_time()
    );
    println!(
        "in :{}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );

    println!("\nCalcul of RANDOM");

    dur = std::time::SystemTime::now();
    let random_schedule = random(&mut graph, nb_procs);
    println!(
        "Random schedule time: {} s",
        random_schedule.get_completion_time()
    );
    println!(
        "in :{}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );

    println!("\nCalcul of HLFET");

    dur = std::time::SystemTime::now();
    let hlfet_schedule = hlfet(&mut graph, nb_procs);

    println!(
        "hlfet schedule time : {} s",
        hlfet_schedule.get_completion_time()
    );

    println!(
        "in :{}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );

    println!("\nCalcul of CPFD no communication cost");

    dur = std::time::SystemTime::now();

    let cpfd_schedule = cpfd(&mut graph, 0.0);

    println!(
        "cpfd no cost time: {} s",
        cpfd_schedule.get_completion_time()
    );
    println!("with : {} Processors", cpfd_schedule.processors.len());
    println!(
        "in :{}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );

    println!("\nCalcul of CPFD cost  = 1.0");

    dur = std::time::SystemTime::now();

    let cpfd_schedule = cpfd(&mut graph, 1.0);

    println!(
        "cpfd schedule time : {} s",
        cpfd_schedule.get_completion_time()
    );

    println!("with : {} Processors", cpfd_schedule.processors.len());
    println!(
        "in :{}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        panic!("Need a file");
    }
    static_schedule_file(&args[1]);
}
