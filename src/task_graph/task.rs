use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Task {
    //Ranodom time between
    Random(f64, f64),
    //Constant time
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
