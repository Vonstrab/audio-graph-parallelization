extern crate agp_lib;
extern crate proc_macro;

use agp_lib::parser;

use agp_lib::scheduling::static_alg::*;

fn static_schedule_file(filepath: &std::path::PathBuf) {
    println!("File : {:?}", filepath);

    println!("Parsing");

    let mut graph =
        parser::parse(&filepath.to_str().unwrap()).expect("Failed parsing the audio graph\n");

    println!("Number of nodes: {}", graph.get_topological_order().len());

    println!("\nCalcul of ETF");

    let mut nb_procs = 1;
    println!("Round {}", nb_procs);
    let mut etf_schedule = etf(&mut graph, nb_procs);
    let mut cond = true;
    let mut dur = std::time::SystemTime::now();;

    while cond {
        nb_procs += 1;
        println!("Round {}", nb_procs);
        let new_etf = etf(&mut graph, nb_procs);
        dur = std::time::SystemTime::now();
        if new_etf.get_completion_time() >= etf_schedule.get_completion_time() {
            cond = false;
        } else {
            etf_schedule = new_etf;
        }
    }

    println!("\nWith {} processors:", nb_procs);
    println!(
        "EFT schedule time : {} s",
        etf_schedule.get_completion_time()
    );
    println!("EFT schedule : {}", etf_schedule);
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
    println!("Random schedule : {} s", random_schedule);
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

    println!("hlfet schedule : {} ", hlfet_schedule);
    println!(
        "in :{}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );

    println!("\nCalcul of CPFD no communication cost");

    dur = std::time::SystemTime::now();

    let cpfd_schedule_no_com = cpfd(&mut graph, 0.0);

    println!(
        "cpfd no cost time: {} s",
        cpfd_schedule_no_com.get_completion_time()
    );
    println!("cpfd no cost schedule: {}", cpfd_schedule_no_com);
    println!(
        "with : {} Processors",
        cpfd_schedule_no_com.processors.len()
    );
    println!(
        "in :{}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );

    println!("\nCalcul of CPFD cost  = 1.0");

    dur = std::time::SystemTime::now();

    let cpfd_schedule = cpfd(&mut graph, 1.0);

    println!("cpfd schedule : {} ", cpfd_schedule);
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

#[test]
fn test_little_graphs() {
    for file in std::fs::read_dir("Samples/AG/little_random_graphs").unwrap() {
        let file = file.unwrap();
        let path = file.path();
        static_schedule_file(&path);
    }
}
