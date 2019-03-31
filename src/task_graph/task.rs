use std::collections::HashMap;
use std::fmt;

use crate::dsp::{DspNode, InputsOutputsAdaptor, Modulator, Oscillator, Sink};

#[derive(Clone, Debug, PartialEq)]
pub enum Task {
    // Random time interval
    Random(f64, f64),
    // Constant time
    Constant(f64),

    Puredata {
        // Pure Data related informations
        object_name: String, // Pure Data object's name
        xpos: i64,           // Pure Data node's X position
        ypos: i64,           // Pure Data node's Y position
        args: Vec<String>,   // Pure Data object's list of arguments
    },

    Audiograph {
        wcet: Option<f64>,
        id: String, // `AGNode`'s ID
        nb_inlets: u32,
        nb_outlets: u32,
        class_name: String,
        text: Option<String>,
        more: HashMap<String, String>,
        volume: f32,
    },
}

impl Task {
    pub fn new_ag() -> Task {
        Task::Audiograph {
            id: String::new(),
            nb_inlets: 0,
            nb_outlets: 0,
            class_name: String::new(),
            text: None,
            wcet: None,
            volume: 1.,
            more: HashMap::new(),
        }
    }
}

pub struct DspTask {
    pub id: String,
    pub dsp: DspNode,
}

impl DspTask {
    pub fn new_oscillator(id: String, frequency: u32, volume: f32) -> DspTask {
        let osc = DspNode::Oscillator(Oscillator::new(0.0, frequency, volume));

        DspTask { id, dsp: osc }
    }

    pub fn new_modulator(id: String, frequency: u32, volume: f32) -> DspTask {
        let modul = DspNode::Modulator(Modulator::new(0.0, frequency, volume));

        DspTask { id, dsp: modul }
    }

    pub fn new_io_adaptor(id: String, nb_inputs: usize, nb_outputs: usize) -> DspTask {
        let io_adaptor =
            DspNode::InputsOutputsAdaptor(InputsOutputsAdaptor::new(nb_inputs, nb_outputs));

        DspTask {
            id,
            dsp: io_adaptor,
        }
    }

    pub fn new_sink(id: String, nb_channels: usize) -> DspTask {
        let io_adaptor = DspNode::Sink(Sink::new(nb_channels));

        DspTask {
            id,
            dsp: io_adaptor,
        }
    }
}

impl fmt::Debug for DspTask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dsp task!")
    }
}
