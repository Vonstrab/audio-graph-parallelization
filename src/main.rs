extern crate agp_lib;

use agp_lib::audiograph_parser;

use std::fs::File;
use std::io::*;

fn main() {
    let mut file = File::open("./Samples/aleatoire.pd").expect("Unable to open");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let ret = audiograph_parser::parse_audiograph(&contents);
}
