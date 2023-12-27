use modular_bitfield::{bitfield, specifiers::*};

use crate::nth_bit;

const DUTY_CYCLES: [[bool; 8]; 4] = [
    [false, false, false, false, false, false, false, true],
    [true, false, false, false, false, false, false, true],
    [true, false, false, false, false, true, true, true],
    [false, true, true, true, true, true, true, false],
];

pub trait Channel {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct Nr10 {
    step: B3,
    direction: B1,
    pace: B3,
    empty: B1,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct Nr14 {
    period: B3,
    empty: B3,
    length_en: B1,
    trigger: B1,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct Nr0 {
    shift: B3,
    sweep_direction: B1,
    period: B3,
    empty: B1,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct Nr2 {
    sweep_pace: B3,
    envelope_dir: B1,
    inital_volume: B4,
}



struct VolumeEnvelope {
    finished: bool,
    timer: i32,
    starting_volume: i8,
    add_mode: bool,
    period: i8,
    volume: i8,
}

impl VolumeEnvelope {
    fn new() -> VolumeEnvelope {
        VolumeEnvelope {
            finished: false,
            timer: 0,
            starting_volume: 0,
            add_mode: false,
            period: 0,
            volume: 0,
        }
    }
    fn _tick(&mut self) {
        if self.finished {
            return;
        }

        if self.timer <= 0 {
            self.timer = if self.period != 0 {
                self.period as i32
            } else {
                8
            };

            if self.add_mode && self.volume < 15 {
                self.volume += 1;
            } else if !self.add_mode && self.volume > 0 {
                self.volume += 1;
            }

            if self.volume == 0 || self.volume == 15 {
                self.finished = true;
            }
        }
    }

    fn turn_off(&mut self) {
        self.finished = true;
        self.timer = 0;

        self.starting_volume = 0;
        self.add_mode = false;
        self.period = 0;

        self.volume = 0;
    }

    fn set_nr2(&mut self, value: i8) {
        self.starting_volume = value >> 4;
        self.add_mode = nth_bit!(value, 3) != 0;
        self.period = value & 0b111;
    }

    fn nr2(&self) -> i8 {
        let add_m = (self.add_mode as i8) << 3;

        (self.starting_volume << 4) | (add_m) | self.period
    }

    fn volume(&self) -> i8 {
        if self.period > 0 {
            return self.volume;
        } else {
            return self.starting_volume;
        }
    }

    fn trigger(&mut self) {
        self.volume = self.starting_volume;
        self.finished = false;

        self.timer = if self.period != 0 {
            self.period as i32
        } else {
            8
        };
    }
}

struct LengthCounter {
    enabled: bool,
    full_length: i32,
    length: i32,
    frame_sequencer: i32,
}

impl LengthCounter {
    fn new() -> LengthCounter {
        LengthCounter {
            enabled: false,
            full_length: 0,
            length: 0,
            frame_sequencer: 0,
        }
    }

    fn tick(&mut self) {
        if self.enabled && self.length > 0 {
            self.length = self.length - 1;
        }
    }

    fn turn_of(&mut self) {
        self.enabled = false;
        self.frame_sequencer = 0;
    }

    fn set_nr4(&mut self, value: i8) {
        let enable = nth_bit!(value, 6) != 0;
        let trigger = nth_bit!(value, 7) != 0;

        if self.enabled {
            if trigger && self.length == 0 {
                self.length = self.full_length - 1;
            } else {
                self.length = self.full_length;
            }
        } else if enable {
            if (self.frame_sequencer & 1) == 0 {
                if self.length != 0 {
                    self.length = self.length - 1;
                }

                if trigger && self.length == 0 {
                    self.length = self.full_length - 1;
                }
            }
        } else {
            if trigger && self.length == 0 {
                self.length = self.full_length;
            }
        }

        self.enabled = enable;
    }
}

pub struct Square1 {
    nr11: u8,
    nr12: u8,
    nr13: u8,
    nr14: u8,
}

pub struct Square2 {
    nr21: u8,
    nr22: u8,
    nr23: u8,
    nr24: u8,
}

/*
impl Channel for Square {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF10 => (),
            0xFF11 => (self.duty << 6) | 0x3F,
            0xFF12 => self.volume_envolpe,
            0xFF13 => 0xFF,
            0xFF14 => (),
            _ => 0,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {}
}
*/

/*
pub struct Wave {}
impl Channel for Wave {}

pub struct Noise {}
impl Channel for Noise {}

pub struct Control {}

impl Channel for Control {}
*/

/*
pub struct Channel {
    sequence: u8,
    duty: u8,
    length: u8,
}
*/

pub struct Apu {}

impl Apu {}
