//! Implement a Node for the Audio Graph

#[derive(Default, Debug, Clone)]
pub struct AudioNode {
    //Node Name
    pub object_name: String,
    //Node_id
    pub id: String,
    //Node x and y positions
    pub xpos: i64,
    pub ypos: i64,
    //Node List of Arguments
    pub args: Vec<String>,
    pub nb_inlets: i64,
    pub nb_outlets: i64,
    pub text: Option<String>,
    pub wcet: Option<f64>,
}

impl AudioNode {
    /**
     * Create a new empty Node
     **/
    pub fn new() -> AudioNode {
        AudioNode {
            id: String::default(),
            object_name: String::default(),
            xpos: -1,
            ypos: -1,
            args: Vec::new(),
            nb_inlets: -1,
            nb_outlets: -1,
            text: None,
            wcet: None,
        }
    }
    /**
     * Set Node Position
     */
    pub fn set_pos(&mut self, x: i64, y: i64) {
        self.xpos = x;
        self.ypos = y;
    }
    /**
     * Add an argument to the actual list
     **/
    pub fn add_arg(&mut self, arg: String) {
        self.args.push(arg);
    }
}
