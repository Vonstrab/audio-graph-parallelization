extern crate agp_lib;

use agp_lib::parser;

use agp_lib::scheduling::static_alg::*;

fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1{
        panic!("Need a file");
    } 

    println!("File : {:?}", args[1]);

    println!("Parsing");

    let mut graph = parser::parse(&args[1])
        .expect("Failed parsing the audio graph\n");


    println!("\nOutpout the dot representation in tmp/graph.dot");

    agp_lib::task_graph::graph::run_dot(&graph, "graph");

    println!("\nCalcul of nodes number");

    println!("Number of nodes: {}", graph.get_topological_order().len());

    println!("\nCalcul of ETF");

    let dur = std::time::SystemTime::now();

    let mut nb_procs = 1;
    println!("Round {}",nb_procs );
    let mut etf_schedule = etf(&mut graph, nb_procs);
    let mut cond = true;
    let mut dur  = std::time::SystemTime::now();;

    while  cond {
        nb_procs +=1;
        println!("Round {}",nb_procs );
        let new_etf = etf(&mut graph, nb_procs);
        dur = std::time::SystemTime::now();
        if !(new_etf.get_completion_time() < etf_schedule.get_completion_time()){
            cond = false;
        }else{
            etf_schedule = new_etf;
        }
    }

    println!("\nWith {} processors:",nb_procs);
    println!("EFT schedule : {}", etf_schedule.get_completion_time());
    println!(
        "in :{}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );

    println!("\nCalcul of RANDOM");

    dur = std::time::SystemTime::now();
    let random_schedule = random(&mut graph, nb_procs); 
    println!(
        "Random schedule : {} s",
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
        "hlfet schedule : {} s",
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

    println!("cpfd no cost schedule: {} s", cpfd_schedule.get_completion_time());

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
        "cpfd schedule : {} s",
        cpfd_schedule.get_completion_time()
    );

    println!("with : {} Processors", cpfd_schedule.processors.len());
    println!(
        "in :{}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );
}
