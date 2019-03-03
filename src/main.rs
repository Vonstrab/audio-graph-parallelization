extern crate agp_lib;

use agp_lib::parser;
use agp_lib::task_graph::graph::TaskGraph;
use agp_lib::task_graph::task::Task;

use agp_lib::scheduling::static_alg::*;

fn main() {
    let mut alea_pd = parser::parse("Samples/PD/aleatoire2.pd");

    println!("Alea ******************************");

    println!("Avec 2 proc :");
    println!("EFT schedule{}", etf(&mut alea_pd, 2));
    println!("Random schedule{}", random(&mut alea_pd, 2));
    println!("HLFET schedule{}", hlfet(&mut alea_pd, 2));

    println!("Avec 3 proc :");
    println!("EFT schedule{}", etf(&mut alea_pd, 3));
    println!("Random schedule{}", random(&mut alea_pd, 3));
    println!("hlfet schedule{}", hlfet(&mut alea_pd, 3));

    let mut simple_ag = parser::parse("Samples/AG/audiograph_test.ag");

    println!("Simple ******************************");

    println!("Avec 2 proc :");
    println!("EFT schedule{}", etf(&mut simple_ag, 2));
    println!("Random schedule{}", random(&mut simple_ag, 2));
    println!("HLFET schedule{}", hlfet(&mut simple_ag, 2));

    println!("Avec 3 proc :");
    println!("EFT schedule{}", etf(&mut simple_ag, 3));
    println!("Random schedule{}", random(&mut simple_ag, 3));
    println!("hlfet schedule{}", hlfet(&mut simple_ag, 3));
}
