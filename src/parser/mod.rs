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

#[cfg(test)]
mod parser_test {
    use super::*;

    #[test]
    fn parse_audiograph() {
        let graph_1 = parse("Samples/AG/downsampling_test.ag").unwrap();
        assert_eq!(graph_1.get_topological_order().len(), 5);
        let graph_2 = parse("Samples/AG/audiograph_wcet_test.ag").unwrap();
        assert_eq!(graph_2.get_topological_order().len(), 3);
    }

    #[test]
    fn parse_puredata() {
        let graph_1 = parse("Samples/PD/aleatoire.pd").unwrap();
        assert_eq!(graph_1.get_topological_order().len(), 9);
        let graph_2 = parse("Samples/PD/Tonalite.pd").unwrap();
        assert_eq!(graph_2.get_topological_order().len(), 12);
    }
}
