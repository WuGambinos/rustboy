#![allow(clippy::must_use_candidate)]

pub mod mmu;
pub mod ppu;

use log::debug;
use log::warn;

use crate::constants::*;
use crate::cpu::interrupts::request_interrupt;
use crate::cpu::interrupts::InterruptType;
use crate::cpu::timer::Timer;
use crate::interconnect::mmu::Mmu;
use crate::interconnect::ppu::Ppu;
use modular_bitfield::prelude::*;

#[bitfield]
#[derive(Debug)]
pub struct JoypadInput {
    right_or_a: B1,
    left_or_b: B1,
    up_or_select: B1,
    down_or_start: B1,
    select_direction: B1,
    select_action: B1,
    empty: B2,
}

#[derive(Debug)]
pub struct SerialOutput {
    buffer: Vec<u8>,
}

impl SerialOutput {
    fn new() -> SerialOutput {
        SerialOutput { buffer: Vec::new() }
    }

    pub fn write_byte(&mut self, data: u8) {
        self.buffer.push(data);
    }

    pub fn read_bytes(&self) -> Vec<u8> {
        self.buffer.clone()
    }

    pub fn output(&mut self) {
        let result = String::from_utf8(self.buffer.clone());

        match result {
            Ok(s) => print!("{}", s),
            Err(e) => println!("Error: {}", e),
        }

        self.buffer.clear();
    }
}

/// Struct used to link CPU to other components of system
///
/// Contains MMU and Timer (so far)
#[derive(Debug)]
pub struct Interconnect {
    pub mmu: Mmu,
    pub timer: Timer,
    pub ppu: Ppu,
    pub serial: SerialOutput,
    pub joypad: JoypadInput,
    pub boot_active: bool,
    pub write_enabled: bool,
}

impl Interconnect {
    pub fn new() -> Self {
        Self {
            mmu: Mmu::new(),
            timer: Timer::new(),
            ppu: Ppu::new(),
            serial: SerialOutput::new(),
            joypad: JoypadInput::new(),
            boot_active: true,
            write_enabled: true,
        }
    }

    pub fn log_timer(&self) {
        debug!(
            "DIV: {:#X} TIMA: {:#X} TMA: {:#X} TAC: {:#X}",
            self.timer.div(),
            self.timer.tima(),
            self.timer.tma(),
            self.timer.tac()
        );
    }

    pub fn log_vram(&self) {
        for i in (0x8000..0x9FFF).rev() {
            debug!("Addr: {:#X} Val: {:#X}", i, self.read_mem(i));
        }
    }

    pub fn dma_transfer(&mut self, value: u8) {
        let addr: u16 = u16::from(value) << 8;

        for i in 0..0xA0 {
            self.write_mem(0xFE00 + i, self.read_mem(addr + i));
        }
    }

    pub fn write_mem(&mut self, addr: u16, value: u8) {
        if ROM_BANK.contains(&addr) {
            if self.write_enabled {
                self.mmu.write_rom_bank(addr, value);
            }
        } else if VRAM.contains(&addr) {
            self.ppu.write_vram(addr, value);
        } else if EXTERNAL_RAM.contains(&addr) {
            self.mmu.write_external_ram(addr - 0xA000, value);
        } else if WORK_RAM.contains(&addr) {
            self.mmu.write_work_ram(addr - 0xC000, value);
        } else if OAM.contains(&addr) {
            if self.ppu.dma_transferring() {
                return;
            }
            self.ppu.write_oam(addr, value);
        } else if TIMER.contains(&addr) {
            self.timer.timer_write(addr, value);
        } else if LCD.contains(&addr) {
            self.ppu.write_lcd(addr, value)
        } else if IO.contains(&addr) {
            self.mmu.write_io(addr - 0xFF00, value);
        } else if HIGH_RAM.contains(&addr) {
            self.mmu.write_hram(addr - 0xFF80, value);
        } else if addr == INTERRUPT_ENABLE {
            self.mmu.enable_interrupt(value);
        } else {
            warn!("UNREACHABLE Addr: {:#X}", addr);
        }
    }

    pub fn read_mem(&self, addr: u16) -> u8 {
        if self.boot_active && BOOT.contains(&addr) {
            self.mmu.read_boot(addr)
        } else if ROM_BANK.contains(&addr) {
            self.mmu.read_rom_bank(addr)
        } else if VRAM.contains(&addr) {
            self.ppu.read_vram(addr)
        } else if EXTERNAL_RAM.contains(&addr) {
            self.mmu.read_external_ram(addr - 0xA000)
        } else if WORK_RAM.contains(&addr) {
            self.mmu.read_work_ram(addr - 0xC000)
        } else if OAM.contains(&addr) {
            if self.ppu.dma_transferring() {
                0xFF
            } else {
                self.ppu.read_oam(addr)
            }
        } else if TIMER.contains(&addr) {
            self.timer.timer_read(addr)
        } else if LCD.contains(&addr) {
            self.ppu.read_lcd(addr)
        } else if IO.contains(&addr) {
            if addr == 0xFF00 {
                //0x1F
                0x1D
            } else {
                self.mmu.read_io(addr - 0xFF00)
            }
        } else if HIGH_RAM.contains(&addr) {
            self.mmu.read_hram(addr - 0xFF80)
        } else if addr == INTERRUPT_ENABLE {
            self.mmu.read_interrupt_enable()
        } else {
            warn!("NOT REACHABLE ADDR: {:#X}", addr);
            0
        }
    }

    pub fn load_game_rom(&mut self, rom: &[u8]) {
        for (i, _) in rom.iter().enumerate() {
            self.write_mem(i as u16, rom[i]);
        }
    }

    pub fn load_boot_rom(&mut self, rom: &[u8]) {
        for (i, _) in rom.iter().enumerate() {
            self.mmu.write_boot(i as u16, rom[i]);
        }
    }

    pub fn emu_tick(&mut self, m_cycles: u32) {
        // Convert M cycles to T cycles
        let t_cycles = m_cycles * 4;

        for _ in 0..t_cycles {
            let interrupts = self.ppu.tick();

            for int in interrupts {
                request_interrupt(self, int);
            }
        }

        // Used to get cycle count over in main loop
        self.timer.set_internal_ticks(u64::from(t_cycles));

        let div_value: u8 = self.timer.div_clock.next(t_cycles) as u8;
        self.timer.set_div(div_value);

        let timer_enabled: bool = (self.timer.tac() & 0x04) != 0x00;
        if timer_enabled {
            let n = self.timer.tma_clock.next(t_cycles);

            for _ in 0..n {
                let tima_value = self.timer.tima().wrapping_add(1);
                self.timer.set_tima(tima_value);

                if self.timer.tima() == 0x00 {
                    self.timer.set_tima(self.timer.tma());
                    request_interrupt(self, InterruptType::Timer);
                }
            }
        }

        let dma_cycles = m_cycles;
        for _ in 0..dma_cycles {
            self.dma_tick();
        }
    }

    pub fn dma_tick(&mut self) {
        if !self.ppu.dma_active() {
            return;
        }

        if self.ppu.dma_start_delay() > 0 {
            let delay_value = self.ppu.dma_start_delay().wrapping_add(1);
            self.ppu.set_dma_start_delay(delay_value);
            return;
        }

        let addr: u16 = (u16::from(self.ppu.dma_value()) * 0x100) + u16::from(self.ppu.dma_byte());

        self.ppu
            .write_oam(u16::from(self.ppu.dma_byte()), self.read_mem(addr));

        let byte_value = self.ppu.dma_byte().wrapping_add(1);
        self.ppu.set_dma_byte(byte_value);

        self.ppu.set_dma_active(self.ppu.dma_byte() < 0xA0);
    }
}

impl Default for Interconnect {
    fn default() -> Self {
        Self::new()
    }
}
