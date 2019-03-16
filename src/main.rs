extern crate agp_lib;

use agp_lib::parser;

use agp_lib::scheduling::static_alg::*;

fn main() {
    let mut simple_ag = parser::parse("Samples/AG/audiograph_wcet_test.ag")
        .expect("Failed parsing the audio graph\n");

    println!("WCET Test ******************************");

    println!("With 2 processors:");
    println!("EFT schedule{}", etf(&mut simple_ag, 2));
    println!("Random schedule{}", random(&mut simple_ag, 2));
    println!("HLFET schedule{}", hlfet(&mut simple_ag, 2));

    println!("With 3 processors:");
    println!("EFT schedule{}", etf(&mut simple_ag, 3));
    println!("Random schedule{}", random(&mut simple_ag, 3));
    println!("hlfet schedule{}", hlfet(&mut simple_ag, 3));

    let mut simple_ag =
        parser::parse("Samples/AG/downsampling_test.ag").expect("Failed parsing the audio graph\n");

    println!("downsampling ******************************");

    println!("With 2 processors:");
    println!("Random schedule{}", random(&mut simple_ag, 2));
    println!("EFT schedule{}", etf(&mut simple_ag, 2));
    println!("HLFET schedule{}", hlfet(&mut simple_ag, 2));

    println!("With 3 processors:");
    println!("Random schedule{}", random(&mut simple_ag, 3));
    println!("EFT schedule{}", etf(&mut simple_ag, 3));
    println!("hlfet schedule{}", hlfet(&mut simple_ag, 3));
}
