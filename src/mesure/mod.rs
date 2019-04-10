use std::fs::File;
use std::io::Write;
use std::sync::mpsc;

use std::collections::HashMap;

pub struct Mesure {
    pub rx: mpsc::Receiver<(String, String)>,
    pub files: HashMap<String, File>,
}

impl Mesure {
    pub fn new(rx: mpsc::Receiver<(String, String)>) -> Mesure {
        Mesure {
            rx,
            files: HashMap::new(),
        }
    }

    pub fn receive(&mut self) {
        loop {
            match self.rx.recv() {
                Ok((chan, msg)) => {
                    let channel = chan;
                    let message = msg;
                    // println!("Mesure , channel : {} , msg : {}", channel, message);
                    self.write(&channel, &message).unwrap();
                }
                Err(_) => {
                    break;
                }
            }
        }
    }

    pub fn write(&mut self, channel: &String, msg: &String) -> std::io::Result<()> {
        if channel == "stdout" {
            println!("{}", msg);
        } else if channel == "stderr" {
            eprintln!("{}", msg);
        } else {
            if self.files.contains_key(channel) {
                let file = self.files.get_mut(channel).unwrap();
                write!(file, "{}", msg)?;
            } else {
                let path = std::path::Path::new(channel);
                let mut file = std::fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(path)?;

                self.files
                    .insert(channel.to_string(), file.try_clone().unwrap());

                write!(file, "{}", msg)?;
            }
        }
        Ok(())
    }
}
