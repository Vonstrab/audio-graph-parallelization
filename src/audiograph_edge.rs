//! This module implements an edge for the audio graph

use std::fmt;

#[derive(Default, Debug, Clone)]
pub struct AGEdge {
    pub estimated_communication_cost: Option<f64>, // In milliseconds
}

impl AGEdge {
    pub fn new() -> AGEdge {
        AGEdge {
            estimated_communication_cost: Some(0.0), // Neglected for now.
        }
    }
}

impl fmt::Display for AGEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.estimated_communication_cost {
            Some(cost) => write!(f, "{}", cost),
            None => write!(f, "None"),
        }
    }
}
