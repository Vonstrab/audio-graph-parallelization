extern crate agp_lib;

use agp_lib::audiograph_parser::parse_audiograph_from_file;
use agp_lib::puredata_parser::parse_puredata_from_file;

fn main() {
    parse_puredata_from_file("./Samples/PD/aleatoire.pd");

    parse_audiograph_from_file("./Samples/AG/test.ag");
}
