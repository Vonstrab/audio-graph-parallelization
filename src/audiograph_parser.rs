//! Parse a fileformat describing audiographs

use std::fs::File;
use std::io::*;

use pest::Parser;

#[derive(Parser)]
#[grammar = "audiograph.pest"]
pub struct AudiographParser;

pub fn parse_audiograph(audiograph: &str) -> bool {
    let parse_result =
        AudiographParser::parse(Rule::file, audiograph).unwrap_or_else(|e| panic!("{}", e));

    for pair in parse_result {
        let span = pair.clone().into_span();
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", span);
        println!("Text:    {}", span.as_str());
    } 
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_audiograph_test() {
      //TODO
    }

}
