extern crate agp_lib;

use agp_lib::task_graph::graph::TaskGraph;
use agp_lib::task_graph::task::Task;

use agp_lib::scheduling::static_alg::*;

fn main() {
    //println!("Hello , No main here Yet");
    let mut g = TaskGraph::new(8, 9);
    let mut nodes_idx = Vec::new();

    for i in 0..8 {
        nodes_idx.push(g.add_task(Task::A));
        g.set_wcet(i, 1.0);
    }

    g.add_edge(7, 5);
    g.add_edge(7, 6);
    g.add_edge(5, 2);
    g.add_edge(5, 4);
    g.add_edge(6, 4);
    g.add_edge(6, 3);
    g.add_edge(2, 1);
    g.add_edge(3, 1);
    g.add_edge(1, 0);
    println!("Graphe 1 ******************************");

    println!("Avec 2 proc :");
    println!("EFT schedule{}", etf(&mut g, 2));
    println!("hlfet schedule{}", hlfet(&mut g, 2));

    println!("Avec 3 proc :");
    println!("EFT schedule{}", etf(&mut g, 3));
    println!("hlfet schedule{}", hlfet(&mut g, 3));

    println!("Avec 4 proc :");
    println!("EFT schedule{}", etf(&mut g, 4));
    println!("hlfet schedule{}", hlfet(&mut g, 4));

    println!("\n\nGraphe 2 ******************************");

    g = TaskGraph::new(24, 21);
    nodes_idx = Vec::new();

    for i in 0..24 {
        nodes_idx.push(g.add_task(Task::A));
        g.set_wcet(i, 1.0);
    }

    g.add_edge(0, 19);
    g.add_edge(1, 6);
    g.add_edge(1, 2);
    g.add_edge(2, 7);
    g.add_edge(3, 7);

    g.add_edge(4, 9);
    g.add_edge(5, 11);
    g.add_edge(6, 22);
    g.add_edge(6, 8);
    g.add_edge(7, 8);

    g.add_edge(7, 10);
    g.add_edge(8, 22);
    g.add_edge(8, 12);
    g.add_edge(9, 10);
    g.add_edge(10, 15);

    g.add_edge(10, 14);
    g.add_edge(10, 13);
    g.add_edge(11, 15);
    g.add_edge(11, 9);
    g.add_edge(12, 17);

    g.add_edge(12, 16);
    g.add_edge(13, 12);
    g.add_edge(14, 0);
    g.add_edge(14, 18);
    g.add_edge(16, 20);

    g.add_edge(17, 20);
    g.add_edge(17, 21);
    g.add_edge(18, 21);
    g.add_edge(18, 17);
    g.add_edge(18, 19);

    println!("Avec 2 proc :");
    println!("EFT schedule{}", etf(&mut g, 2));
    println!("hlfet schedule{}", hlfet(&mut g, 2));

    println!("Avec 3 proc :");
    println!("EFT schedule{}", etf(&mut g, 3));
    println!("hlfet schedule{}", hlfet(&mut g, 3));

    println!("Avec 4 proc :");
    println!("EFT schedule{}", etf(&mut g, 4));
    println!("hlfet schedule{}", hlfet(&mut g, 4));
}
