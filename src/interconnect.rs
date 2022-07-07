use crate::cpu::interrupts::interrupt_request;
use crate::cpu::interrupts::InterruptType;
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

    pub fn print_timer(&self) {
        println!(
            "DIV: {:#X} TIMA: {:#X} TMA: {:#X} TAC: {:#X}",
            self.timer.div, self.timer.tima, self.timer.tma, self.timer.tac
        );
    }

    pub fn write_mem(&mut self, addr: u16, value: u8) {
        if (0xFF04..=0xFF07).contains(&addr) {
            self.timer.timer_write(addr, value);
        } else {
            self.mmu.memory[addr as usize] = value;
        }
    }

    pub fn read_mem(&self, addr: u16) -> u8 {
        if (0xFF04..=0xFF07).contains(&addr) {
            self.timer.timer_read(addr)
        } else {
            self.mmu.memory[addr as usize]
        }
    }

    pub fn read_rom(&mut self, rom: &[u8]) {
        for (i, _) in rom.iter().enumerate() {
            self.write_mem(i as u16, rom[i]);
        }
    }

    /*  pub fn timer_tick(&mut self) {
        let prev_div: u16 = self.timer.div;

        //Div++
        self.timer.div = self.timer.div.wrapping_add(1);

        let mut timer_update: bool = false;

        match self.timer.tac & (0b11) {
            0b00 => timer_update = (prev_div & (1 << 9)) > 0 && (!(self.timer.div & (1 << 9))) == 1,

            0b01 => timer_update = (prev_div & (1 << 3)) > 0 && (!(self.timer.div & (1 << 3))) == 1,

            0b10 => timer_update = (prev_div & (1 << 5)) > 0 && (!(self.timer.div & (1 << 5))) == 1,

            0b11 => timer_update = (prev_div & (1 << 7)) > 0 && (!(self.timer.div & (1 << 7))) == 1,
            _ => (),
        }

        if timer_update && ((self.timer.tac & (1 << 2)) > 0) {
            self.timer.tima = self.timer.tima.wrapping_add(1);

            if self.timer.tima == 0xFF {
                self.timer.tima = self.timer.tma;

                //Request Interrupt
                interrupt_request(self, InterruptType::Timer);
            }
        }
    }*/

    /*pub fn emu_cycles(&mut self, cpu_cycles: u64) {
        let n: u64 = cpu_cycles * 4;
        for _ in 0..n {
            self.timer_tick();
            self.timer.internal_ticks = self.timer.internal_ticks.wrapping_add(1);
        }
    }*/

    pub fn emu_cycles(&mut self, cyc: u32) {

        let cycles = cyc * 4;
        self.timer.div = self
            .timer
            .div
            .wrapping_add(self.timer.div_clock.next(cycles) as u8);
        if (self.timer.tac & 0x04) != 0x00 {
            let n = self.timer.tma_clock.next(cycles);

            for _ in 0..n {
                self.timer.tima = self.timer.tima.wrapping_add(1);

                if self.timer.tima == 0x00 {
                    self.timer.tima = self.timer.tma;
                    interrupt_request(self, InterruptType::Timer);
                }
            }
        }
    }
}

impl Default for Interconnect {
    fn default() -> Self {
        Self::new()
    }
}
