//! Implement a Node for the Audio Graph

#[derive(Default, Debug)]
pub struct AudioNode {
    //Node Name
    pub audio_object: String,
    //Node x and y positions
    pub xpos: i64,
    pub ypos: i64,
    //Node List of Arguments
    pub args: Vec<String>,
}

impl AudioNode {
    /**
     * Create a new empty Node
     **/
    pub fn new() -> AudioNode {
        AudioNode {
            audio_object: String::default(),
            xpos: -1,
            ypos: -1,
            args: Vec::new(),
        }
    }
    /**
     * Set Node name
     */
    pub fn set_object(&mut self, value: String) {
        self.audio_object = value;
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
