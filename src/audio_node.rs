
#[derive(Default,Debug)]
pub struct AudioNode{
    pub audio_object:String ,
    pub xpos:i64 ,
    pub ypos: i64 ,
    pub args:Vec <String>
}

impl AudioNode {
    pub fn new()-> AudioNode{
        AudioNode{
            audio_object : String::default(),
            xpos:-1,
            ypos:-1,
            args : Vec::new(),
        }
    }
    pub fn set_object(& mut self, value :String){
        self.audio_object=value;
    }
    pub fn set_pos(& mut self,x:i64,y:i64){
        self.xpos=x;
        self.ypos =y;
    }
    pub fn add_arg(& mut self,arg:String){
        self.args.push(arg);
    }
}