use crate::cpu::interrupts::interrupt_request;
use crate::cpu::interrupts::InterruptType;
use crate::ppu::PPU;
use crate::{Mmu, Timer};

/// Struct used to link CPU to other components of system
///
/// Contains MMU and Timer (so far)
#[derive(Debug)]
pub struct Interconnect {
    pub mmu: Mmu,
    pub timer: Timer,
    pub ppu: PPU,
}

impl Interconnect {
    /// Constructor
    pub fn new() -> Self {
        Self {
            mmu: Mmu::new(),
            timer: Timer::new(),
            ppu: PPU::new(),
        }
    }

    /// Prints the state of the Timer
    pub fn print_timer(&self) {
        println!(
            "DIV: {:#X} TIMA: {:#X} TMA: {:#X} TAC: {:#X}",
            self.timer.div, self.timer.tima, self.timer.tma, self.timer.tac
        );
    }

    pub fn print_vram(&self) {
        for i in 0x8000..0x9FFF {
            println!("Addr: {:#X} Val: {:#X}", i, self.read_mem(i));
        }
    }

    pub fn print_oam() {

    }

    /// Write u8 to memory
    pub fn write_mem(&mut self, addr: u16, value: u8) {
        if (0x8000..0x9FFF).contains(&addr) {
            self.ppu.write_vram(addr, value);
        } else if (0xFE00..=0xFE9F).contains(&addr) {
            self.ppu.write_oam(addr, value);
        } else if (0xFF04..=0xFF07).contains(&addr) {
            self.timer.timer_write(addr, value);
        } else {
            self.mmu.memory[addr as usize] = value;
        }
    }

    /// Read u8 value from memory
    pub fn read_mem(&self, addr: u16) -> u8 {
        if (0x8000..0x9FFF).contains(&addr) {
            self.ppu.read_vram(addr)
        } else if (0xFE00..=0xFE9F).contains(&addr) {
            self.ppu.read_oam(addr)
        } else if (0xFF04..=0xFF07).contains(&addr) {
            self.timer.timer_read(addr)
        } else {
            self.mmu.memory[addr as usize]
        }
    }

    /// Read gameboy rom and write it into memory
    pub fn read_rom(&mut self, rom: &[u8]) {
        for (i, _) in rom.iter().enumerate() {
            self.write_mem(i as u16, rom[i]);
        }
    }

    /// Tick Timer
    pub fn emu_cycles(&mut self, cyc: u32) {
        // Convert M cycles to T cycles
        let cycles = cyc * 4;

        // Increase Div
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

                    // Trigger Interrupt
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
