extern crate agp_lib;

use agp_lib::puredata_parser::graph_from_pd;

use std::fs::File;
use std::io::*;

fn main() {
    let mut file = File::open("./Samples/PD/aleatoire.pd").expect("Unable to open");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("fail to read file");
    graph_from_pd(&contents);
}
