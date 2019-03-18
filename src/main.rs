extern crate agp_lib;

use agp_lib::parser;

use agp_lib::scheduling::static_alg::*;

fn random_dag(nb_nodes: usize) -> agp_lib::task_graph::graph::TaskGraph {
    let mut edges: Vec<(usize, usize)> = Vec::new();

    for i in 0..nb_nodes {
        for j in 0..i {
            if (rand::random::<usize>() % 100) < 5 {
                edges.push((i, j));
            }
        }
    }

    println!("Nombre de edges {}", edges.len());

    let mut graph = agp_lib::task_graph::graph::TaskGraph::new(nb_nodes, edges.len());

    for _ in 0..nb_nodes {
        graph.add_task(agp_lib::task_graph::task::Task::Constant(1.0));
    }

    for (s, d) in edges {
        graph.add_edge(s, d);
    }
    graph
}

fn main() {
    // let mut metro =
    //     parser::parse("Samples/PD/Metronome.pd").expect("Failed parsing the audio graph\n");

    // agp_lib::task_graph::graph::run_dot(&metro, "metro");

    let mut g50 = random_dag(200);
    agp_lib::task_graph::graph::run_dot(&g50, "g50");

    println!("200 noeuds ******************************");

    println!("\nWith 2 processors:");
    let dur = std::time::SystemTime::now();
    println!("EFT schedule :{}", etf(&mut g50, 2).get_completion_time());
    println!(
        "in :{}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );
    let dur = std::time::SystemTime::now();

    println!(
        "Random schedule :{}",
        random(&mut g50, 2).get_completion_time()
    );
    println!(
        "in :{}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );

    let dur = std::time::SystemTime::now();

    println!(
        "HLFET schedule :{}",
        hlfet(&mut g50, 2).get_completion_time()
    );
    println!(
        "in :{}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );
    let dur = std::time::SystemTime::now();

    println!("\nWith 3 processors:");
    println!(
        "Random schedule : {}",
        random(&mut g50, 3).get_completion_time()
    );
    println!(
        "in :{}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );
    let dur = std::time::SystemTime::now();
    println!("EFT schedule : {}", etf(&mut g50, 3).get_completion_time());
    println!(
        "in :{}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );
    let dur = std::time::SystemTime::now();
    println!(
        "hlfet schedule : {}",
        hlfet(&mut g50, 3).get_completion_time()
    );
    println!(
        "in :{}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );

    let dur = std::time::SystemTime::now();
    println!("\nWith 4 processors:");
    println!(
        "Random schedule : {}",
        random(&mut g50, 4).get_completion_time()
    );
    println!(
        "in :{}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );
    let dur = std::time::SystemTime::now();
    println!("EFT schedule : {}", etf(&mut g50, 4).get_completion_time());
    println!(
        "in :{}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );
    let dur = std::time::SystemTime::now();
    println!(
        "hlfet schedule : {}",
        hlfet(&mut g50, 4).get_completion_time()
    );
    println!(
        "in :{}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );

    let dur = std::time::SystemTime::now();
    println!("\nWith 5 processors:");
    println!(
        "Random schedule : {}",
        random(&mut g50, 5).get_completion_time()
    );
    println!(
        "in :{}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );
    let dur = std::time::SystemTime::now();
    println!("EFT schedule : {}", etf(&mut g50, 5).get_completion_time());
    println!(
        "in :{}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );
    let dur = std::time::SystemTime::now();
    println!(
        "hlfet schedule : {}",
        hlfet(&mut g50, 5).get_completion_time()
    );
    println!(
        "in :{}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );
}
