use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use crossbeam::channel::Receiver;

pub enum MeasureDestination {
    Stdout(String),       // Content
    Stderr(String),       // Content
    File(String, String), // Filename and content
}

pub struct Measure {
    pub rx: Receiver<MeasureDestination>,
    pub files: HashMap<String, File>,
}

impl Measure {
    pub fn new(rx: Receiver<MeasureDestination>) -> Measure {
        Measure {
            rx,
            files: HashMap::new(),
        }
    }

    pub fn receive(&mut self) {
        loop {
            match self.rx.recv() {
                Ok(dest) => {
                    // println!("Measure, channel: {}, msg: {}", channel, message);
                    self.write(dest).unwrap();
                }
                Err(_) => {
                    break;
                }
            }
        }
    }

    pub fn write(&mut self, dest: MeasureDestination) -> std::io::Result<()> {
        match dest {
            MeasureDestination::Stdout(msg) => println!("{}", msg),
            MeasureDestination::Stderr(msg) => eprintln!("{}", msg),
            MeasureDestination::File(path, msg) => {
                if self.files.contains_key(&path) {
                    let file = self.files.get_mut(&path).unwrap();

                    write!(file, "{}", msg)?;
                } else {
                    let mut file = std::fs::OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open(&path)?;

                    self.files.insert(path, file.try_clone().unwrap());

                    write!(file, "{}", msg)?;
                }
            }
        }

        Ok(())
    }
}
