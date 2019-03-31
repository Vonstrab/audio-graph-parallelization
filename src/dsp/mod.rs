use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub struct DspEdge {
    buffer: Vec<f32>,
    pub sample_rate: usize,
}

impl DspEdge {
    pub fn new(buffer_size: usize, sample_rate: usize) -> DspEdge {
        DspEdge {
            buffer: vec![0.0; buffer_size],
            sample_rate,
        }
    }

    pub fn buffer(&self) -> &[f32] {
        self.buffer.as_slice()
    }

    pub fn buffer_mut(&mut self) -> &mut [f32] {
        self.buffer.as_mut_slice()
    }
}

pub enum DspNode {
    Oscillator(Oscillator),
    Modulator(Modulator),
    InputsOutputsAdaptor(InputsOutputsAdaptor),
    Sink(Sink),
}

#[derive(Clone, Copy, Debug)]
pub struct Oscillator {
    phase: f32,
    frequency: u32,
    volume: f32,
}

fn sine_wave(phase: f32, volume: f32) -> f32 {
    (phase * std::f64::consts::PI as f32 * 2.0).sin() as f32 * volume
}

impl Oscillator {
    pub fn new(phase: f32, frequency: u32, volume: f32) -> Oscillator {
        Oscillator {
            phase,
            frequency,
            volume,
        }
    }

    pub fn process(&mut self, output: Arc<RwLock<DspEdge>>) {
        let sample_rate = output.read().unwrap().sample_rate;

        for sample in output.write().unwrap().buffer_mut().iter_mut() {
            *sample = sine_wave(self.phase, self.volume);
            self.phase += self.frequency as f32 / sample_rate as f32;
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Modulator {
    phase: f32,
    frequency: u32,
    volume: f32,
}

impl Modulator {
    pub fn new(phase: f32, frequency: u32, volume: f32) -> Modulator {
        Modulator {
            phase,
            frequency,
            volume,
        }
    }

    pub fn process(&mut self, input: Arc<RwLock<DspEdge>>, output: Arc<RwLock<DspEdge>>) {
        debug_assert_eq!(
            output.read().unwrap().buffer().len(),
            input.read().unwrap().buffer().len()
        );

        debug_assert!(input.read().unwrap().sample_rate == output.read().unwrap().sample_rate);
        let samplerate = input.read().unwrap().sample_rate;

        for (sample_out, sample_in) in output
            .write()
            .unwrap()
            .buffer_mut()
            .iter_mut()
            .zip(input.read().unwrap().buffer().iter())
        {
            *sample_out = *sample_in * sine_wave(self.phase, self.volume);
            self.phase += self.frequency as f32 / samplerate as f32;
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct InputsOutputsAdaptor {
    nb_inputs: usize,
    nb_outputs: usize,
    stride: usize,
}

fn mixer(output_buff: &mut [f32], input_buff: &[f32]) {
    for (s1, s2) in output_buff.iter_mut().zip(input_buff) {
        *s1 += *s2
    }
}

impl InputsOutputsAdaptor {
    pub fn new(nb_inputs: usize, nb_outputs: usize) -> InputsOutputsAdaptor {
        assert!(nb_inputs % nb_outputs == 0 || nb_outputs % nb_inputs == 0);

        let stride = if nb_outputs > nb_inputs {
            nb_outputs / nb_inputs
        } else {
            nb_inputs / nb_outputs
        };
        InputsOutputsAdaptor {
            nb_inputs,
            nb_outputs,
            stride,
        }
    }

    pub fn process(
        &mut self,
        inputs: Vec<Arc<RwLock<DspEdge>>>,
        mut outputs: Vec<Arc<RwLock<DspEdge>>>,
    ) {
        debug_assert!(
            self.nb_inputs % self.nb_outputs == 0 || self.nb_outputs % self.nb_inputs == 0
        );

        if self.nb_outputs > self.nb_inputs {
            for (i, group) in outputs.chunks_mut(self.stride).enumerate() {
                for output in group.iter_mut() {
                    output
                        .write()
                        .unwrap()
                        .buffer_mut()
                        .copy_from_slice(inputs[i].read().unwrap().buffer());
                }
            }
        } else {
            for (i, group) in inputs.chunks(self.stride).enumerate() {
                for input in group {
                    mixer(
                        outputs[i].write().unwrap().buffer_mut(),
                        input.read().unwrap().buffer(),
                    );
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Sink {
    nb_channels: usize,
    out_buffer: Option<*mut f32>,
    frames: Option<usize>,
}

impl Sink {
    pub fn new(nb_channels: usize) -> Sink {
        Sink {
            nb_channels,
            out_buffer: None,
            frames: None,
        }
    }

    pub fn set_buffer(&mut self, out_buffer: *mut f32, frames: u32) {
        self.out_buffer = Some(out_buffer);
        self.frames = Some(frames as usize);
    }

    pub fn process(&mut self, input: Arc<RwLock<DspEdge>>) {
        unsafe {
            let out_buffer =
                std::slice::from_raw_parts_mut(self.out_buffer.unwrap(), self.frames.unwrap());

            mixer(out_buffer, input.read().unwrap().buffer());
        }
    }
}

unsafe impl Send for Sink {}
