extern crate agp_lib;

use agp_lib::parser;

use agp_lib::scheduling::static_alg::*;

fn random_dag(nb_nodes: usize) -> agp_lib::task_graph::graph::TaskGraph {
    let mut edges: Vec<(usize, usize)> = Vec::new();

    for i in 0..nb_nodes {
        for j in 0..i {
            if (rand::random::<usize>() % 1000) < 300 {
                edges.push((i , j ));
            }
        }
    }

    println!("Nombre de edges {}", edges.len());

    let mut graph = agp_lib::task_graph::graph::TaskGraph::new(nb_nodes, edges.len());

    for _ in 0..nb_nodes {
        graph.add_task(agp_lib::task_graph::task::Task::Constant(1.0));
    }

    for (s, d) in edges {
        graph.add_edge(s as usize, d as usize);
    }
    graph
}

fn main() {
    let mut g50 =
        parser::parse("Samples/PD/Metronome.pd").expect("Failed parsing the audio graph\n");

    // agp_lib::task_graph::graph::run_dot(&metro, "metro");

    // let mut g50 = random_dag(50);
    agp_lib::task_graph::graph::run_dot(&g50, "g50");

    println!("edges: {}",g50.get_topological_order().len());

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

    let dur = std::time::SystemTime::now();

    let cpfd_schedule = cpfd(&mut g50, 5);

    println!("cpfd schedule time : {}", cpfd_schedule);


    println!("cpfd schedule time : {}", cpfd_schedule.get_completion_time());

    println!("with : {} Processors", cpfd_schedule.processors.len());
    println!(
        "in :{}s {} ms",
        dur.elapsed().unwrap().as_secs(),
        dur.elapsed().unwrap().subsec_millis()
    );
}
