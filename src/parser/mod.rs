pub mod audiograph;
pub mod puredata;

use task_graph::graph;

pub fn parse(filename: &str) -> graph::TaskGraph {
    let graph: graph::TaskGraph;

    if filename.ends_with(".pd") {
        graph = self::puredata::parser::parse(filename).unwrap();
    } else if filename.ends_with(".ag") {
        //ARguments are test values TODO
        graph = self::audiograph::parser::parse(filename).unwrap();
    } else {
        panic!("Wrong File extension!\nWe support puredata files (.pd)");
    }

    graph
}
