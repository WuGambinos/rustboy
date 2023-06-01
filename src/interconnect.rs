#![allow(clippy::must_use_candidate)]
use core::time;

use log::debug;
use log::info;
use log::warn;

use crate::constants::*;
use crate::cpu::interrupts::request_interrupt;
use crate::cpu::interrupts::InterruptType;
use crate::cpu::timer::Timer;
use crate::mmu::Mmu;
use crate::ppu::Control;
use crate::ppu::FetchState;
use crate::ppu::LcdMode;
use crate::ppu::Ppu;
use crate::ppu::Stat;

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
}

impl Interconnect {
    pub fn new() -> Self {
        Self {
            mmu: Mmu::new(),
            timer: Timer::new(),
            ppu: Ppu::new(),
            serial: SerialOutput::new(),
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
            self.mmu.write_rom_bank(addr, value);
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
        }
        // Interrupt Enable
        else if addr == INTERRUPT_ENABLE {
            self.mmu.enable_interrupt(value);
        } else {
            warn!("UNREACHABLE Addr: {:#X}", addr);
        }
    }

    pub fn read_mem(&self, addr: u16) -> u8 {
        // ROM Bank
        if ROM_BANK.contains(&addr) {
            self.mmu.read_rom_bank(addr)
        }
        // VRAM
        else if VRAM.contains(&addr) {
            self.ppu.read_vram(addr)
        }
        // External RAM
        else if EXTERNAL_RAM.contains(&addr) {
            self.mmu.read_external_ram(addr - 0xA000)
        }
        // Work RAM
        else if WORK_RAM.contains(&addr) {
            self.mmu.read_work_ram(addr - 0xC000)
        }
        // OAM
        else if OAM.contains(&addr) {
            if self.ppu.dma_transferring() {
                0xFF
            } else {
                self.ppu.read_oam(addr)
            }
        }
        // Timer
        else if TIMER.contains(&addr) {
            self.timer.timer_read(addr)
        }
        // LCD
        else if LCD.contains(&addr) {
            self.ppu.read_lcd(addr)
        }
        // IO Regsiters
        else if IO.contains(&addr) {
            self.mmu.read_io(addr - 0xFF00)
        }
        // High RAM
        else if HIGH_RAM.contains(&addr) {
            self.mmu.read_hram(addr - 0xFF80)
        }
        // Interrupt Enable
        else if addr == INTERRUPT_ENABLE {
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
            self.write_mem(i as u16, rom[i]);
        }
    }

    pub fn emu_tick(&mut self, cyc: u32) {
        // Convert M cycles to T cycles
        let cycles = cyc * 4;

        for _ in 0..cyc {
            self.ppu_tick();
        }

        // Used to get cycle count over in main loop
        self.timer.set_internal_ticks(u64::from(cyc));

        let div_value: u8 = self.timer.div_clock.next(cycles) as u8;
        self.timer.set_div(div_value);

        let timer_enabled: bool = (self.timer.tac() & 0x04) != 0x00;
        if timer_enabled {
            let n = self.timer.tma_clock.next(cycles);

            for _ in 0..n {
                let tima_value = self.timer.tima().wrapping_add(1);
                self.timer.set_tima(tima_value);

                if self.timer.tima() == 0x00 {
                    self.timer.set_tima(self.timer.tma());
                    request_interrupt(self, InterruptType::Timer);
                }
            }
        }

        let dma_cycles = cyc / 4;
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

        if !self.ppu.dma_active() {
            info!("DMA DONE!");
            let secs = time::Duration::from_secs(1);
            std::thread::sleep(secs);
        }
    }

    /****************************************************
     * PPU FUNCTIONS
     ****************************************************/

    pub fn increment_ly(&mut self) {
        let value = self.ppu.ly().wrapping_add(1);
        self.ppu.set_ly(value);

        if self.ppu.ly() == self.ppu.lyc() {
            self.ppu.stat.set(Stat::LYC_LY_EQ_FLAG, true);

            if self.ppu.stat().contains(Stat::LYC_LY_EQ_INTERRUPT) {
                request_interrupt(self, InterruptType::LcdStat);
            } else {
                self.ppu.stat.set(Stat::LYC_LY_EQ_FLAG, false);
            }
        }
    }

    pub fn add_pixel(&mut self) {}

    pub fn push_pixel(&mut self) {
        if self.ppu.pixel_fifo.fifo.len() > 8 {
            let pixel_data: sdl2::pixels::Color = self.ppu.pixel_fifo.pop_fifo().unwrap();

            if self.ppu.pixel_fifo.x() >= (self.ppu.scroll_x() % 8) {
                self.ppu.video_buffer[0] = pixel_data;
            }
        }
    }

    pub fn process(&mut self) {
        let ly = self.ppu.ly();
        let scroll_y = self.ppu.scroll_y();

        self.ppu.pixel_fifo.map_x = (self.ppu.pixel_fifo.x() + self.ppu.scroll_x()) & 0x1F;
        self.ppu.pixel_fifo.map_y = ((ly + scroll_y) / 8) * 32;

        self.ppu.pixel_fifo.tile_data = ((ly + scroll_y) & 0xFF) / 8;

        if self.ppu.line_ticks() % 2 == 0 {
            self.ppu_fetch();
        }

        self.push_pixel();
    }

    /// Fetcher grabs a row of 8 pixels at a time to be fed to either fifo
    pub fn ppu_fetch(&mut self) {
        match self.ppu.pixel_fifo.fetch_state() {
            FetchState::Tile => {
                let bg_and_window_enabled: bool = self.ppu.control().contains(Control::BG_WINDOW);
                if bg_and_window_enabled {
                    let addr: u16 = self.ppu.bg_tile_map_addr()
                        + self.ppu.pixel_fifo.map_x as u16
                        + self.ppu.pixel_fifo.map_y as u16;

                    self.ppu.pixel_fifo.bg_window_data[0] = self.read_mem(addr);

                    let fetch_window_pixels: bool = self.ppu.bg_window_data_area() == 0x8800;
                    if fetch_window_pixels {
                        self.ppu.pixel_fifo.bg_window_data[0] =
                            self.ppu.pixel_fifo.bg_window_data[0].wrapping_add(128);
                    }

                    self.ppu.pixel_fifo.set_fetch_state(FetchState::Data0);
                    self.ppu
                        .pixel_fifo
                        .set_x(self.ppu.pixel_fifo.x().wrapping_add(8));
                }
            }
            FetchState::Data0 => {
                let addr: u16 = self.ppu.bg_window_data_area()
                    + (self.ppu.pixel_fifo.bg_window_data[0] as u16 * 16) as u16
                    + self.ppu.pixel_fifo.tile_data as u16;

                self.ppu.pixel_fifo.bg_window_data[1] = self.read_mem(addr);
                self.ppu.pixel_fifo.set_fetch_state(FetchState::Data1);
            }
            FetchState::Data1 => {
                let addr: u16 = self.ppu.bg_window_data_area()
                    + (self.ppu.pixel_fifo.bg_window_data[0] as u16 * 16) as u16
                    + (self.ppu.pixel_fifo.tile_data as u16 + 1);

                self.ppu.pixel_fifo.bg_window_data[2] = self.read_mem(addr);
                self.ppu.pixel_fifo.set_fetch_state(FetchState::Sleep);
            }
            FetchState::Sleep => {
                self.ppu.pixel_fifo.set_fetch_state(FetchState::Push);
            }
            FetchState::Push => {
                self.add_pixel();
            }
        }
    }

    /// Search OAM for Sprites whose Y coordinate
    /// overlaps this line
    ///
    /// Duration: 80 "dots"
    pub fn oam_mode(&mut self) {
        let oam_is_over = self.ppu.line_ticks() >= 80;
        if oam_is_over {
            self.ppu.set_stat_mode(LcdMode::Transfer);

            self.ppu.pixel_fifo.set_fetch_state(FetchState::Tile);
            self.ppu.pixel_fifo.set_x(0);
        }
    }

    /// Reading OAM and VRAM to generate picture
    ///
    /// Duration: 168-291 "dots", depends on sprite count
    pub fn transfer_mode(&mut self) {
        // Pipeline TODO
        if self.ppu.pixel_fifo.x() >= X_RESOLUTION {
            self.ppu.pixel_fifo.clear();

            self.ppu.set_stat_mode(LcdMode::HBlank);

            if self.ppu.stat().contains(Stat::HBLANK_INTERRUPT) {
                request_interrupt(self, InterruptType::LcdStat);
            }
        }
    }

    /// Duration: 4560 "dots" (10 scanlines)
    pub fn vblank_mode(&mut self) {
        let end_of_scanline = self.ppu.line_ticks() >= TICKS_PER_LINE;
        if end_of_scanline {
            self.increment_ly();

            let onto_next_screen = self.ppu.ly() >= LINES_PER_FRAME;
            if onto_next_screen {
                self.ppu.set_stat_mode(LcdMode::Oam);
                self.ppu.set_ly(0);
            }

            self.ppu.set_line_ticks(0);
        }
    }

    /// Duration: 87-204 "dots"
    pub fn hblank_mode(&mut self) {
        let end_of_scanline = self.ppu.line_ticks() >= TICKS_PER_LINE;
        if end_of_scanline {
            self.increment_ly();

            if self.ppu.ly() == Y_RESOLUTION {
                self.ppu.set_stat_mode(LcdMode::VBlank);
                request_interrupt(self, InterruptType::VBlank);

                if self.ppu.stat().contains(Stat::VBLANK_INTERRUPT) {
                    request_interrupt(self, InterruptType::VBlank);
                }
            } else {
                self.ppu.set_stat_mode(LcdMode::Oam);
            }

            self.ppu.set_line_ticks(0);
        }
    }

    pub fn ppu_tick(&mut self) {
        self.ppu.increment_line_ticks();

        match self.ppu.stat_mode() {
            LcdMode::Oam => self.oam_mode(),
            LcdMode::Transfer => self.transfer_mode(),
            LcdMode::VBlank => self.vblank_mode(),
            LcdMode::HBlank => self.hblank_mode(),
        }
    }
}

impl Default for Interconnect {
    fn default() -> Self {
        Self::new()
    }
}
