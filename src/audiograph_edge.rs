//! This module implements an edge for the audio graph

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
