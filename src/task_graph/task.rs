// This enum is incomplete for now
#[derive(Clone)]
pub enum Task {
    //Ranodom time between
    Random(f64,f64),
    //Constant time
    Constant(f64),
    Puredata {
        // Pure Data related informations
        object_name: String, // Pure Data object's name
        xpos: i64,   // Pure Data node's X position
        ypos: i64,   // Pure Data node's Y position
        args: Vec<String>,   // Pure Data object's list of arguments
    },
    Audiograph{
    id: String, // `AGNode`'s ID
    text: Option<String>,
    wcet: Option<f64>,
    }
}
