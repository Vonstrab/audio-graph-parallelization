extern crate agp_lib;

use agp_lib::parser;
use agp_lib::task_graph::graph::TaskGraph;
use agp_lib::task_graph::task::Task;

use agp_lib::scheduling::static_alg::*;

fn main() {

    let mut simple_ag = parser::parse("Samples/AG/audiograph_wcet_test.ag");

    println!("WCET Test ******************************");

    println!("Avec 2 proc :");
    println!("EFT schedule{}", etf(&mut simple_ag, 2));
    println!("Random schedule{}", random(&mut simple_ag, 2));
    println!("HLFET schedule{}", hlfet(&mut simple_ag, 2));

    println!("Avec 3 proc :");
    println!("EFT schedule{}", etf(&mut simple_ag, 3));
    println!("Random schedule{}", random(&mut simple_ag, 3));
    println!("hlfet schedule{}", hlfet(&mut simple_ag, 3));



    let mut simple_ag = parser::parse("Samples/AG/downsampling_test.ag");

    println!("downsampling ******************************");

    println!("Avec 2 proc :");
    println!("EFT schedule{}", etf(&mut simple_ag, 2));
    println!("Random schedule{}", random(&mut simple_ag, 2));
    println!("HLFET schedule{}", hlfet(&mut simple_ag, 2));

    println!("Avec 3 proc :");
    println!("EFT schedule{}", etf(&mut simple_ag, 3));
    println!("Random schedule{}", random(&mut simple_ag, 3));
    println!("hlfet schedule{}", hlfet(&mut simple_ag, 3));


}
