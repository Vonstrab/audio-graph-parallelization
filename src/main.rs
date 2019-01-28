extern crate agp_lib;

use agp_lib::puredata_parser::parse_puredata_from_file;
use agp_lib::audiograph_parser::parse_audiograph_from_file;

use std::fs::File;
use std::io::*;

fn main() {
    let mut pd_parse = parse_puredata_from_file("./Samples/PD/aleatoire.pd");

    let mut ag_parse = parse_audiograph_from_file("./Samples/AG/test.ag");
}
