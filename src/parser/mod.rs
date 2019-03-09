pub mod audiograph;
pub mod puredata;

use task_graph::graph;

pub fn parse(filename: &str) -> Option<graph::TaskGraph> {
    if filename.ends_with(".pd") {
        Some(self::puredata::parser::parse(filename).unwrap())
    } else if filename.ends_with(".ag") {
        Some(self::audiograph::parser::parse(filename).unwrap())
    } else {
        None
    }
}
