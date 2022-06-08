use crate::{Mmu, Timer};

#[derive(Debug)]
pub struct Interconnect {
    pub mmu: Mmu,
    pub timer: Timer,
}

impl Interconnect {
    pub fn new() -> Self {
        Self {
            mmu: Mmu::new(),
            timer: Timer::new(),
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

    pub fn read_rom(&mut self, rom: &Vec<u8>) {
        for i in 0..rom.len() {
            self.write_mem(i as u16, rom[i]);
        }
    }

    // Execute the timer cycle
    pub fn do_cycle(&mut self, ticks: u32, interconnect: &mut Interconnect) {
        self.timer.internal_ticks += ticks;

        while self.timer.internal_ticks >= 256 {
            self.timer.div = self.timer.div.wrapping_add(1);
            self.timer.internal_ticks -= 256;
        }

        //Is Clock Enabled
        if self.timer.is_clock_enabled() {
            // Increment counter
            self.timer.tima.wrapping_add(1);

            if self.timer.tima == 0 {
                self.timer.tima = self.timer.tma;

                //REQUEST INTERRUPT
            }
        }
    }
}
