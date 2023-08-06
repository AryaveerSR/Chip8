use cpal::{
    self,
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BuildStreamError, FromSample, Sample, SizedSample, Stream,
};
use std::{
    f32::consts,
    sync::mpsc::{channel, Sender},
    thread,
};

/// Structure for general-purpose registers.
/// Simplies accessing them from instructions.
#[derive(Debug)]
pub struct VariableRegisters {
    pub v0: u8,
    pub v1: u8,
    pub v2: u8,
    pub v3: u8,
    pub v4: u8,
    pub v5: u8,
    pub v6: u8,
    pub v7: u8,
    pub v8: u8,
    pub v9: u8,
    pub va: u8,
    pub vb: u8,
    pub vc: u8,
    pub vd: u8,
    pub ve: u8,
    /// It is also used as flag register for instructions.
    /// Many instructions set it to 1 or 0 based on some rule (eg: carry flag)
    pub vf: u8,
}

impl VariableRegisters {
    pub fn get(&self, reg: u8) -> u8 {
        match reg {
            0 => self.v0,
            1 => self.v1,
            2 => self.v2,
            3 => self.v3,
            4 => self.v4,
            5 => self.v5,
            6 => self.v6,
            7 => self.v7,
            8 => self.v8,
            9 => self.v9,
            10 => self.va,
            11 => self.vb,
            12 => self.vc,
            13 => self.vd,
            14 => self.ve,
            15 => self.vf,
            _ => panic!(),
        }
    }

    pub fn set(&mut self, reg: u8, val: u8) {
        match reg {
            0 => self.v0 = val,
            1 => self.v1 = val,
            2 => self.v2 = val,
            3 => self.v3 = val,
            4 => self.v4 = val,
            5 => self.v5 = val,
            6 => self.v6 = val,
            7 => self.v7 = val,
            8 => self.v8 = val,
            9 => self.v9 = val,
            10 => self.va = val,
            11 => self.vb = val,
            12 => self.vc = val,
            13 => self.vd = val,
            14 => self.ve = val,
            15 => self.vf = val,
            _ => panic!(),
        };
    }

    pub fn new() -> Self {
        VariableRegisters {
            v0: 0,
            v1: 0,
            v2: 0,
            v3: 0,
            v4: 0,
            v5: 0,
            v6: 0,
            v7: 0,
            v8: 0,
            v9: 0,
            va: 0,
            vb: 0,
            vc: 0,
            vd: 0,
            ve: 0,
            vf: 0,
        }
    }
}

// Contains helpful methods for parsing instructions
#[derive(Debug)]
pub struct Instruction(pub u16);

impl Instruction {
    // Group of 4 bits. Index from most to least significant
    pub fn get_nib(&self, index: u8) -> u8 {
        ((self.0 & (0x000f << 4 * (3 - index as i8))) >> (4 * (3 - index as i8))) as u8
    }

    // Lowest 8 bits (lower byte)
    pub fn get_lbyte(&self) -> u8 {
        (self.0 & 0x00ff) as u8
    }

    // Lowest 12 bits
    pub fn get_addr(&self) -> u16 {
        (self.0 & 0x0fff) as u16
    }

    pub fn from_u16(n: u16) -> Self {
        Instruction(n)
    }
}

/// Behavior Configurations for conflicting implementations
#[derive(Debug)]
pub struct BehaviorConfig {
    /// Does it reset V(F) register to 0 for 8xy1, 8xy2 and 8xy3 instructions
    pub vf_reset: bool,
    /// Does it increment I register on save and load operations
    pub increment_i_on_save_load: bool,
}

impl BehaviorConfig {
    pub fn default() -> Self {
        BehaviorConfig {
            vf_reset: true,
            increment_i_on_save_load: true,
        }
    }
}

enum BeeperMessage {
    Play,
    Pause,
    Stop,
}

/// A simple Beeper implementation using `cpal` crate
#[derive(Debug)]
pub struct Beeper {
    tx: Sender<BeeperMessage>,
    is_on: bool,
}

impl Beeper {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();
        let config = device.default_output_config().unwrap();

        let (tx, rx) = channel::<BeeperMessage>();

        thread::spawn(move || {
            let stream = match config.sample_format() {
                cpal::SampleFormat::F32 => Self::create_stream::<f32>(&device, &config.into()),
                cpal::SampleFormat::I16 => Self::create_stream::<i16>(&device, &config.into()),
                cpal::SampleFormat::U16 => Self::create_stream::<u16>(&device, &config.into()),
                cpal::SampleFormat::I8 => Self::create_stream::<i8>(&device, &config.into()),
                cpal::SampleFormat::I32 => Self::create_stream::<i32>(&device, &config.into()),
                cpal::SampleFormat::I64 => Self::create_stream::<i64>(&device, &config.into()),
                cpal::SampleFormat::U8 => Self::create_stream::<u8>(&device, &config.into()),
                cpal::SampleFormat::U64 => Self::create_stream::<u64>(&device, &config.into()),
                cpal::SampleFormat::F64 => Self::create_stream::<f64>(&device, &config.into()),
                _ => panic!(),
            };

            match stream {
                Ok(stream) => {
                    stream.pause().unwrap();
                    loop {
                        match rx.recv() {
                            Ok(BeeperMessage::Play) => stream.play().unwrap(),
                            Ok(BeeperMessage::Pause) => stream.pause().unwrap(),
                            Ok(BeeperMessage::Stop) => {
                                stream.pause().unwrap();
                                return;
                            }
                            Err(err) => panic!("{}", err),
                        }
                    }
                }
                Err(err) => panic!("{}", err),
            }
        });

        Beeper { tx, is_on: false }
    }

    pub fn update(&mut self, state: bool) {
        if state != self.is_on {
            if state {
                self.tx.send(BeeperMessage::Play).unwrap();
            } else {
                self.tx.send(BeeperMessage::Pause).unwrap();
            }
            self.is_on = state;
        }
    }

    pub fn stop(&self) {
        self.tx.send(BeeperMessage::Stop).unwrap();
    }

    fn create_stream<T>(
        device: &cpal::Device,
        config: &cpal::StreamConfig,
    ) -> Result<Stream, BuildStreamError>
    where
        T: SizedSample + FromSample<f32>,
    {
        let sample_rate = config.sample_rate.0 as f32;
        let channels = config.channels as usize;

        let mut sample_clock = 0f32;
        let mut next_value = move || {
            sample_clock = (sample_clock + 1.0) % sample_rate;
            (sample_clock * 440.0 * 2.0 * consts::PI / sample_rate).sin()
        };

        let err_fn = |err| panic!("{}", err);

        device.build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                Self::write_data(data, channels, &mut next_value)
            },
            err_fn,
            None,
        )
    }

    fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
    where
        T: Sample + FromSample<f32>,
    {
        for frame in output.chunks_mut(channels) {
            let value: T = T::from_sample(next_sample());
            for sample in frame.iter_mut() {
                *sample = value;
            }
        }
    }
}
