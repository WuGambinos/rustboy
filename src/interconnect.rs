use core::time;

use crate::cpu::interrupts::interrupt_request;
use crate::cpu::interrupts::InterruptType;
use crate::ppu::PPU;
use crate::{Mmu, Timer};

static mut ly: u8 = 0;

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
        for i in (0x8000..0x9FFF).rev() {
            println!("Addr: {:#X} Val: {:#X}", i, self.read_mem(i));
        }
    }

    pub fn dma_transfer(&mut self, value: u8) {
        let addr: u16 = (value as u16) << 8;

        for i in 0..0xA0 {
            self.write_mem(0xFE00 + i, self.read_mem(addr + i));
        }
    }

    /// Write u8 to memory
    pub fn write_mem(&mut self, addr: u16, value: u8) {
        // ROM Bank
        if (0x0000..0x8000).contains(&addr) {
            self.mmu.rom_bank[addr as usize] = value;
        }
        // VRAM
        else if (0x8000..0xA000).contains(&addr) {
            self.ppu.write_vram(addr, value);
        }
        // External RAM
        else if (0xA000..0xC000).contains(&addr) {
            self.mmu.external_ram[(addr - 0xA000) as usize] = value;
        }
        // Work RAM
        else if (0xC000..0xE000).contains(&addr) {
            self.mmu.work_ram[(addr - 0xC000) as usize] = value;
        }
        // OAM
        else if (0xFE00..0xFEA0).contains(&addr) {
            self.ppu.write_oam(addr, value);
        }
        // Timer
        else if (0xFF04..0xFF08).contains(&addr) {
            self.timer.timer_write(addr, value);
        }
        // Trigger DMA
        else if addr == 0xFF46 {
            self.dma_start(value);
            println!("DMA TRIGGERED");
            std::process::exit(0);
        }
        // IO registers
        else if (0xFF00..0xFF80).contains(&addr) {
            self.mmu.io[(addr - 0xFF00) as usize] = value;
        }
        // High RAM (HRAM)
        else if (0xFF80..0xFFFF).contains(&addr) {
            self.mmu.hram[(addr - 0xFF80) as usize] = value;
        }
        // Interrupt Enable
        else if addr == 0xFFFF {
            self.mmu.interrupt_enable = value;
        } else {
            //println!("UNREACHABLE Addr: {:#X}", addr);
        }
    }

    /// Read u8 value from memory
    pub fn read_mem(&self, addr: u16) -> u8 {
        // ROM Bank
        if (0x0000..0x8000).contains(&addr) {
            self.mmu.rom_bank[addr as usize]
        }
        // VRAM
        else if (0x8000..0xA000).contains(&addr) {
            self.ppu.read_vram(addr)
        }
        // External RAM
        else if (0xA000..0xC000).contains(&addr) {
            self.mmu.external_ram[(addr - 0xA000) as usize]
        }
        // Work RAM
        else if (0xC000..0xE000).contains(&addr) {
            self.mmu.work_ram[(addr - 0xC000) as usize]
        }
        // OAM
        else if (0xFE00..0xFEA0).contains(&addr) {
            self.ppu.read_oam(addr)
        }
        // Timer
        else if (0xFF04..0xFF08).contains(&addr) {
            self.timer.timer_read(addr)
        } else if addr == 0xFF44 {
            unsafe {
                let new_ly = ly.wrapping_add(1);
                ly = new_ly;
                ly
            }
        }
        // IO Regsiters
        else if (0xFF00..0xFF80).contains(&addr) {
            self.mmu.io[(addr - 0xFF00) as usize]
        }
        // High RAM
        else if (0xFF80..0xFFFF).contains(&addr) {
            self.mmu.hram[(addr - 0xFF80) as usize]
        }
        // Interrupt Enable
        else if addr == 0xFFFF {
            self.mmu.interrupt_enable
        } else {
            //println!("NOT REACHABLE ADDR: {:#X}", addr);
            0
        }
    }

    /// Read gameboy rom and write it into memory
    pub fn read_rom(&mut self, rom: &[u8]) {
        for (i, _) in rom.iter().enumerate() {
            self.write_mem(i as u16, rom[i]);
        }
    }

    // Read gameboy boot rom and write it into memory
    pub fn read_boot(&mut self, rom: &[u8]) {
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

        self.dma_tick();
    }

    pub fn dma_start(&mut self, value: u8) {
        self.ppu.dma.active = true;
        self.ppu.dma.byte = 0;
        self.ppu.dma.start_delay = 2;
        self.ppu.dma.value = value;
    }

    pub fn dma_tick(&mut self) {
        if !self.ppu.dma.active {
            return;
        }

        if self.ppu.dma.start_delay > 0 {
            self.ppu.dma.start_delay = self.ppu.dma.start_delay.wrapping_sub(1);
            return;
        }

        let addr: u16 = (((self.ppu.dma.value as u16) * 0x100) as u16) + (self.ppu.dma.byte as u16);

        self.ppu
            .write_oam(self.ppu.dma.byte as u16, self.read_mem(addr));

        self.ppu.dma.byte = self.ppu.dma.byte.wrapping_add(1);

        self.ppu.dma.active = self.ppu.dma.byte < 0xA0;

        if !self.ppu.dma.active {
            println!("DMA DONE!");
            let secs = time::Duration::from_secs(2);
            std::thread::sleep(secs);
        }
    }
}

impl Default for Interconnect {
    fn default() -> Self {
        Self::new()
    }
}
