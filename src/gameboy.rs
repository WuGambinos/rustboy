use crate::interconnect::Interconnect;
use crate::{Cpu, Timer};

use crate::Mmu;

use crate::cpu::instructions::*;

pub struct GameBoy {
    pub cpu: Cpu,
    pub mmu: Mmu,
    pub timer: Timer,
    pub interconnect: Interconnect,
}

impl GameBoy {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            mmu: Mmu::new(),
            timer: Timer::new(),
            interconnect: Interconnect::new(),
        }
    }

    pub fn write_mem(&mut self, addr: u16, value: u8) {
        if addr >= 0xFF04 && addr <= 0xFF07 {
            match addr {
                0xFF04 => self.timer.div = 0,

                0xFF05 => self.timer.tima = value,

                0xFF06 => self.timer.tma = value,

                0xFF07 => self.timer.tac = value,

                _ => (),
            }
        } else {
            self.mmu.memory[addr as usize] = value;
        }
    }

    pub fn read_mem(&self, addr: u16) -> u8 {
        if addr >= 0xFF04 && addr <= 0xFF07 {
            match addr {
                0xFF04 => return self.timer.div as u8,

                0xFF05 => return self.timer.tima,

                0xFF06 => return self.timer.tma,

                0xFF07 => return self.timer.tac,

                _ => return 0,
            }
        } else {
            self.mmu.memory[addr as usize]
        }
    }
}
