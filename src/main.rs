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

    let mut setf = etf(&mut g, 2);
    let mut srandom = random(&mut g, 2);
    let mut shlfet = hlfet(&mut g, 2);

    println!("Avec 2 proc :");
    println!("EFT schedule{}", setf);
    println!("Random schedule{}", srandom);
    println!("HLFET schedule{}", shlfet);

    setf.output("visual/etf.txt");
    srandom.output("visual/rand.txt");
    shlfet.output("visual/hlfet.txt");

    setf = etf(&mut g, 3);
    srandom = random(&mut g, 3);
    shlfet = hlfet(&mut g, 3);

    println!("Avec 3 proc :");
    println!("EFT schedule{}", setf);
    println!("Random schedule{}", srandom);
    println!("hlfet schedule{}", shlfet);

    setf.output("visual/etf3.txt");
    srandom.output("visual/rand3.txt");
    shlfet.output("visual/hlfet3.txt");

    setf = etf(&mut g, 4);
    srandom = random(&mut g, 4);
    shlfet = hlfet(&mut g, 4);

    println!("Avec 4 proc :");
    println!("EFT schedule{}", setf);
    println!("Random schedule{}", srandom);
    println!("hlfet schedule{}", shlfet);

    setf.output("visual/etf4.txt");
    srandom.output("visual/rand44.txt");
    shlfet.output("visual/hlfet4.txt");

    //    agp_lib::task_graph::graph::run_dot(&g, "G1");

    // println!("\n\nGraphe 2 ******************************");

    // g = TaskGraph::new(24, 21);
    // nodes_idx = Vec::new();

    // for i in 0..24 {
    //     nodes_idx.push(g.add_task(Task::A));
    //     g.set_wcet(i, 1.0);
    // }

    // g.add_edge(0, 19);
    // g.add_edge(1, 6);
    // g.add_edge(1, 2);
    // g.add_edge(2, 7);
    // g.add_edge(3, 7);

    // g.add_edge(4, 9);
    // g.add_edge(5, 11);
    // g.add_edge(6, 22);
    // g.add_edge(6, 8);
    // g.add_edge(7, 8);

    // g.add_edge(7, 10);
    // g.add_edge(8, 22);
    // g.add_edge(8, 12);
    // g.add_edge(9, 10);
    // g.add_edge(10, 15);

    // g.add_edge(10, 14);
    // g.add_edge(10, 13);
    // g.add_edge(11, 15);
    // g.add_edge(11, 9);
    // g.add_edge(12, 17);

    // g.add_edge(12, 16);
    // g.add_edge(13, 12);
    // g.add_edge(14, 0);
    // g.add_edge(14, 18);
    // g.add_edge(16, 20);

    // g.add_edge(17, 20);
    // g.add_edge(17, 21);
    // g.add_edge(18, 21);
    // g.add_edge(18, 17);
    // g.add_edge(18, 19);

    // println!("Avec 2 proc :");
    // println!("EFT schedule{}", etf(&mut g, 2));
    // println!("Random schedule{}", random(&mut g, 2));
    // println!("hlfet schedule{}", hlfet(&mut g, 2));

    // println!("Avec 3 proc :");
    // println!("EFT schedule{}", etf(&mut g, 3));
    // println!("Random schedule{}", random(&mut g, 3));
    // println!("hlfet schedule{}", hlfet(&mut g, 3));

    // println!("Avec 4 proc :");
    // println!("EFT schedule{}", etf(&mut g, 4));
    // println!("Random schedule{}", random(&mut g, 4));
    // println!("hlfet schedule{}", hlfet(&mut g, 4));

    // agp_lib::task_graph::graph::run_dot(&g, "G2");

    // println!("\n\nGraphe 3 ******************************");

    // g = TaskGraph::new(33, 34);
    // nodes_idx = Vec::new();

    // for i in 0..33 {
    //     nodes_idx.push(g.add_task(Task::A));
    //     g.set_wcet(i, 1.0);
    // }

    // g.add_edge(0, 6);
    // g.add_edge(1, 8);
    // g.add_edge(2, 8);
    // g.add_edge(3, 9);
    // g.add_edge(4, 10);

    // g.add_edge(5, 11);
    // g.add_edge(6, 17);
    // g.add_edge(7, 16);
    // g.add_edge(8, 15);
    // g.add_edge(9, 14);

    // g.add_edge(10, 13);
    // g.add_edge(11, 12);
    // g.add_edge(17, 19);
    // g.add_edge(16, 20);
    // g.add_edge(15, 20);

    // g.add_edge(14, 21);
    // g.add_edge(13, 21);
    // g.add_edge(13, 22);
    // g.add_edge(12, 22);
    // g.add_edge(12, 23);

    // g.add_edge(19, 24);
    // g.add_edge(20, 24);
    // g.add_edge(20, 25);
    // g.add_edge(21, 25);
    // g.add_edge(21, 26);

    // g.add_edge(22, 26);
    // g.add_edge(23, 26);
    // g.add_edge(24, 27);
    // g.add_edge(25, 29);
    // g.add_edge(26, 29);

    // g.add_edge(27, 28);
    // g.add_edge(28, 31);
    // g.add_edge(29, 30);
    // g.add_edge(30, 32);

    // println!("Avec 3 proc :");
    // println!("EFT schedule{}", etf(&mut g, 3));
    // println!("Random schedule{}", random(&mut g, 3));
    // println!("hlfet schedule{}", hlfet(&mut g, 3));

    // println!("Avec 4 proc :");
    // println!("EFT schedule{}", etf(&mut g, 4));
    // println!("Random schedule{}", random(&mut g, 4));
    // println!("hlfet schedule{}", hlfet(&mut g, 4));

    // println!("Avec 5 proc :");
    // println!("EFT schedule{}", etf(&mut g, 5));
    // println!("Random schedule{}", random(&mut g, 5));
    // println!("hlfet schedule{}", hlfet(&mut g, 5));

    // agp_lib::task_graph::graph::run_dot(&g, "G3");
}
