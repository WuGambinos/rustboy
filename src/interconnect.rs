use core::time;

use sdl2::pixels::Color;

use crate::constants::*;
use crate::cpu::interrupts::request_interrupt;
use crate::cpu::interrupts::InterruptType;
use crate::cpu::timer::Timer;
use crate::lcd::Lcd;
use crate::lcd::LcdMode;
use crate::mmu::Mmu;
use crate::ppu::FetchState;
use crate::ppu::Ppu;

/// Struct used to link CPU to other components of system
///
/// Contains MMU and Timer (so far)
#[derive(Debug)]
pub struct Interconnect {
    pub mmu: Mmu,
    pub timer: Timer,
    pub ppu: Ppu,
    pub lcd: Lcd,
}

impl Interconnect {
    /// Constructor
    pub fn new() -> Self {
        let mut interconnect = Self {
            mmu: Mmu::new(),
            timer: Timer::new(),
            ppu: Ppu::new(),
            lcd: Lcd::new(),
        };
        interconnect.ppu_init();
        interconnect
    }

    pub fn ppu_init(&mut self) {
        self.lcd.set_lcd_stat_mode(LcdMode::OAM as u8);
    }

    /// Prints the state of the Timer
    pub fn print_timer(&self) {
        println!(
            "DIV: {:#X} TIMA: {:#X} TMA: {:#X} TAC: {:#X}",
            self.timer.div(),
            self.timer.tima(),
            self.timer.tma(),
            self.timer.tac()
        );
    }

    /// Prints state of vram
    pub fn print_vram(&self) {
        for i in (0x8000..0x9FFF).rev() {
            println!("Addr: {:#X} Val: {:#X}", i, self.read_mem(i));
        }
    }

    /// Prints state of ppu
    pub fn print_ppu(&self) {
        println!(
            "TICKS: {} FETCH STATE: {:?} line_x: {} pushed_x: {} fetch_x: {} LY: {} bgw_enable: {}",
            self.ppu.line_ticks(),
            self.ppu.fetch_state(),
            self.ppu.line_x(),
            self.ppu.pushed_x(),
            self.ppu.fetch_x(),
            self.lcd.ly(),
            self.lcd.lcdc_bgw_enabled() as u8
        );
        println!(
            "MODE: {:?} map_y: {} map_x: {} tile_y: {} fifo_x: {} fifo_length: {} stat: {:#X}",
            self.lcd.lcd_stat_mode(),
            self.ppu.map_y(),
            self.ppu.map_x(),
            self.ppu.tile_y(),
            self.ppu.fifo_x(),
            self.ppu.pixel_fifo().len(),
            self.lcd.lcd_stat(),
        );
    }

    pub fn dma_transfer(&mut self, value: u8) {
        let addr: u16 = (value as u16) << 8;

        for i in 0..0xA0 {
            self.write_mem(0xFE00 + i, self.read_mem(addr + i));
        }
    }

    /// Write u8 to mem/home/lajuan/Downloads/dmg-acid2.gbory
    pub fn write_mem(&mut self, addr: u16, value: u8) {
        // ROM Bank
        if (0x0000..0x8000).contains(&addr) {
            self.mmu.write_rom_bank(addr, value);
        }
        // VRAM
        else if (0x8000..0xA000).contains(&addr) {
            self.ppu.write_vram(addr, value);
        }
        // External RAM
        else if (0xA000..0xC000).contains(&addr) {
            self.mmu.write_external_ram(addr - 0xA000, value);
        }
        // Work RAM
        else if (0xC000..0xE000).contains(&addr) {
            self.mmu.write_work_ram(addr - 0xC000, value);
        }
        // OAM
        else if (0xFE00..0xFEA0).contains(&addr) {
            if self.ppu.dma_transferring() {
                return;
            }
            self.ppu.write_oam(addr, value);
        }
        // Timer
        else if (0xFF04..0xFF08).contains(&addr) {
            self.timer.timer_write(addr, value);
        }
        // LCD Control
        else if (0xFF40..0xFF4C).contains(&addr) {
            self.lcd.write(&mut self.ppu, addr, value);
        }
        // IO registers
        else if (0xFF00..0xFF80).contains(&addr) {
            self.mmu.write_io(addr - 0xFF00, value);
        }
        // High RAM (HRAM)
        else if (0xFF80..0xFFFF).contains(&addr) {
            self.mmu.write_hram(addr - 0xFF80, value);
        }
        // Interrupt Enable
        else if addr == 0xFFFF {
            self.mmu.enable_interrupt(value);
        } else {
            //println!("UNREACHABLE Addr: {:#X}", addr);
        }
    }

    /// Read u8 value from memory
    pub fn read_mem(&self, addr: u16) -> u8 {
        // ROM Bank
        if (0x0000..0x8000).contains(&addr) {
            self.mmu.read_rom_bank(addr)
        }
        // VRAM
        else if (0x8000..0xA000).contains(&addr) {
            self.ppu.read_vram(addr)
        }
        // External RAM
        else if (0xA000..0xC000).contains(&addr) {
            self.mmu.read_external_ram(addr - 0xA000)
        }
        // Work RAM
        else if (0xC000..0xE000).contains(&addr) {
            self.mmu.read_work_ram(addr - 0xC000)
        }
        // OAM
        else if (0xFE00..0xFEA0).contains(&addr) {
            if self.ppu.dma_transferring() {
                0xFF
            } else {
                self.ppu.read_oam(addr)
            }
        }
        // Timer
        else if (0xFF04..0xFF08).contains(&addr) {
            self.timer.timer_read(addr)
        }
        // LCD Control
        else if (0xFF40..0xFF4C).contains(&addr) {
            self.lcd.read(addr)
        }
        // IO Regsiters
        else if (0xFF00..0xFF80).contains(&addr) {
            self.mmu.read_io(addr - 0xFF00)
        }
        // High RAM
        else if (0xFF80..0xFFFF).contains(&addr) {
            self.mmu.read_hram(addr - 0xFF80)
        }
        // Interrupt Enable
        else if addr == 0xFFFF {
            self.mmu.read_interrupt_enable()
        } else {
            println!("NOT REACHABLE ADDR: {:#X}", addr);
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

        for _ in 0..cyc {
            self.ppu_tick();
        }

        // Used to get cycle count over in main loop
        self.timer.set_internal_ticks(cyc as u64);

        // Increase Div
        let div_value = self.timer.div_clock.next(cycles) as u8;
        self.timer.set_div(div_value);

        if (self.timer.tac() & 0x04) != 0x00 {
            let n = self.timer.tma_clock.next(cycles);

            for _ in 0..n {
                let tima_value = self.timer.tima().wrapping_add(1);
                self.timer.set_tima(tima_value);

                if self.timer.tima() == 0x00 {
                    self.timer.set_tima(self.timer.tma());

                    // Trigger Interrupt
                    request_interrupt(self, InterruptType::Timer);
                }
            }
        }

        for _ in 0..(cyc / 4) {
            self.dma_tick();
        }
    }

    /// Set initial dma state
    pub fn dma_start(&mut self, value: u8) {
        self.ppu.set_dma_active(true);
        self.ppu.set_dma_byte(0);
        self.ppu.set_dma_start_delay(2);
        self.ppu.set_dma_value(value);
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

        let addr: u16 = ((self.ppu.dma_value() as u16) * 0x100) + (self.ppu.dma_byte() as u16);

        self.ppu
            .write_oam(self.ppu.dma_byte() as u16, self.read_mem(addr));

        let byte_value = self.ppu.dma_byte().wrapping_add(1);
        self.ppu.set_dma_byte(byte_value);

        self.ppu.set_dma_active(self.ppu.dma_byte() < 0xA0);

        if !self.ppu.dma_active() {
            //println!("DMA DONE!");
            let secs = time::Duration::from_secs(1);
            std::thread::sleep(secs);
        }
    }

    pub fn ppu_tick(&mut self) {
        self.ppu.increase_line_ticks();

        match self.lcd.lcd_stat_mode() {
            LcdMode::OAM => self.ppu_mode_oam(),
            LcdMode::Transfer => self.ppu_mode_transfer(),
            LcdMode::VBlank => self.ppu_mode_vblank(),
            LcdMode::HBlank => self.ppu_mode_hblank(),
        }
    }

    pub fn increment_ly(&mut self) {
        let value = self.lcd.ly().wrapping_add(1);
        self.lcd.set_ly(value);

        if self.lcd.ly() == self.lcd.lyc() {
            self.lcd.set_lyc_ly_flag(1);

            if self.lcd.lcd_stat_interrupt(SI_LYC) {
                request_interrupt(self, InterruptType::LcdStat);
            }
        } else {
            self.lcd.set_lyc_ly_flag(0);
        }
    }

    pub fn ppu_mode_oam(&mut self) {
        if self.ppu.line_ticks() >= 80 {
            self.lcd.set_lcd_stat_mode(LcdMode::Transfer as u8);

            self.ppu.set_fetch_state(FetchState::Tile);
            self.ppu.set_line_x(0);
            self.ppu.set_fetch_x(0);
            self.ppu.set_pushed_x(0);
            self.ppu.set_fifo_x(0);
        }
    }

    pub fn ppu_mode_transfer(&mut self) {
        self.pipeline_process();
        if self.ppu.pushed_x() as u32 >= X_RES as u32 {
            self.pipeline_fifo_reset();
            self.lcd.set_lcd_stat_mode(LcdMode::HBlank as u8);

            if self.lcd.lcd_stat_interrupt(SI_HBLANK) {
                request_interrupt(self, InterruptType::LcdStat)
            }
        }
    }

    pub fn ppu_mode_vblank(&mut self) {
        if self.ppu.line_ticks() >= TICKS_PER_LINE as u32 {
            self.increment_ly();

            if self.lcd.ly() >= LINES_PER_FRAME {
                self.lcd.set_lcd_stat_mode(LcdMode::OAM as u8);
                self.lcd.set_ly(0);
            }
            self.ppu.set_line_ticks(0);
        }
    }

    pub fn ppu_mode_hblank(&mut self) {
        if self.ppu.line_ticks() >= TICKS_PER_LINE as u32 {
            self.increment_ly();

            if self.lcd.ly() >= Y_RES {
                self.lcd.set_lcd_stat_mode(LcdMode::VBlank as u8);
                request_interrupt(self, InterruptType::VBlank);

                if self.lcd.lcd_stat_interrupt(SI_VBLANK) {
                    request_interrupt(self, InterruptType::LcdStat);
                }
            } else {
                self.lcd.set_lcd_stat_mode(LcdMode::OAM as u8);
            }

            self.ppu.set_line_ticks(0);
        }
    }

    fn pipline_fifo_add(&mut self) -> bool {
        if self.ppu.pixel_fifo().len() > 8 {
            // fifo is full
            return false;
        }

        let x: i16 = (self.ppu.fetch_x() - (8 - (self.lcd.scx() % 8))) as i16;

        let second_byte: u8 = self.ppu.bgw_fetch_data()[1];
        let first_byte: u8 = self.ppu.bgw_fetch_data()[2];

        let mut color: u8;

        for bit in (0..8).rev() {
            let first_bit = (first_byte >> bit) & 1;
            let second_bit = (second_byte >> bit) & 1;

            if first_bit == 0 && second_bit == 0 {
                color = 1;
            } else if first_bit == 0 && second_bit == 1 {
                color = 0;
            } else if first_bit == 1 && second_bit == 0 {
                color = 2;
            } else {
                color = 3;
            }

            let new_color = TILE_COLORS[color as usize];

            if x >= 0 {
                self.ppu.pixel_fifo_push(new_color);
                self.ppu.set_fifo_x(self.ppu.fifo_x().wrapping_add(1));
            }
        }

        true
    }

    fn pipeline_fetch(&mut self) {
        match self.ppu.fetch_state() {
            FetchState::Tile => {
                if self.lcd.lcdc_bgw_enabled() {
                    let addr: u16 = self.lcd.lcdc_bg_tile_map_addr()
                        + ((self.ppu.map_x() / 8) as u16)
                        + ((self.ppu.map_y() / 8) as u32 * 32) as u16;
                    let value: u8 = self.read_mem(addr);
                    self.ppu.set_bgw_fetch_data(0, value);

                    if self.lcd.lcdc_bgw_data_area() == 0x8800 {
                        let value: u8 = self.ppu.bgw_fetch_data()[0].wrapping_add(128);
                        self.ppu.set_bgw_fetch_data(0, value);
                    }
                }
                self.ppu.set_fetch_state(FetchState::Data0);
                self.ppu.set_fetch_x(self.ppu.fetch_x().wrapping_add(8));
            }
            FetchState::Data0 => {
                let addr: u16 = self.lcd.lcdc_bgw_data_area()
                    + (self.ppu.bgw_fetch_data()[0] as u16 * 16)
                    + self.ppu.tile_y() as u16;
                self.ppu.set_bgw_fetch_data(1, self.read_mem(addr));

                self.ppu.set_fetch_state(FetchState::Data1);
            }
            FetchState::Data1 => {
                let addr: u16 = self.lcd.lcdc_bgw_data_area()
                    + ((self.ppu.bgw_fetch_data()[0] as u16 * 16) + (self.ppu.tile_y() as u16 + 1));
                self.ppu.set_bgw_fetch_data(2, self.read_mem(addr));

                self.ppu.set_fetch_state(FetchState::Idle);
            }
            FetchState::Idle => {
                self.ppu.set_fetch_state(FetchState::Push);
            }
            FetchState::Push => {
                if self.pipline_fifo_add() {
                    self.ppu.set_fetch_state(FetchState::Tile);
                }
            }
        }
    }

    fn pipline_push_pixel(&mut self) {
        if self.ppu.pixel_fifo().len() > 8 {
            let pixel_data: Color = self.ppu.pixel_fifo_pop();

            if self.ppu.line_x() >= (self.lcd.scx() % 8) {
                let index =
                    (self.ppu.pushed_x() as u32 + (self.lcd.ly() as u32 * X_RES as u32)) as usize;
                self.ppu.video_buffer[index] = pixel_data;

                self.ppu.set_pushed_x(self.ppu.pushed_x().wrapping_add(1));
            }
            self.ppu.set_line_x(self.ppu.line_x().wrapping_add(1));
        }
    }

    fn pipeline_process(&mut self) {
        self.ppu
            .set_map_y(self.lcd.ly().wrapping_add(self.lcd.scy()));
        self.ppu
            .set_map_x(self.ppu.fetch_x().wrapping_add(self.lcd.scx()));
        self.ppu
            .set_tile_y(((self.lcd.ly().wrapping_add(self.lcd.scy())) % 8) * 2);

        if self.ppu.line_ticks() & 1 != 1 {
            self.pipeline_fetch();
        }

        self.pipline_push_pixel();
    }

    fn pipeline_fifo_reset(&mut self) {
        while !self.ppu.pixel_fifo().is_empty() {
            self.ppu.pixel_fifo_pop();
        }

        //self.ppu.pixel_fifo_push(Color::RGB(0,0,0));
    }
}

impl Default for Interconnect {
    fn default() -> Self {
        Self::new()
    }
}
