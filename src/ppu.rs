#![allow(clippy::must_use_candidate)]
use std::collections::VecDeque;

use crate::constants::*;
use bitflags::*;
use sdl2::pixels::Color;

bitflags! {
    #[derive(Clone, Copy, Debug)]
    struct OamAttr: u8 {
        const PALETTE_NUMBER_CGB    = 0b00000111;
        const TILE_VRAM_BANK        = 0b00001000;
        const PALETTE_NUMBER        = 0b00010000;
        const X_FLIP                = 0b00100000;
        const Y_FLIP                = 0b01000000;
        const BG_WINDOW             = 0b10000000;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct OamEntry {
    y: u8,
    x: u8,
    tile: u8,
    oam_attr: OamAttr,
}

impl OamEntry {
    fn new() -> Self {
        Self {
            y: 0,
            x: 0,
            tile: 0,
            oam_attr: OamAttr::empty(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Dma {
    pub active: bool,
    pub byte: u8,
    pub value: u8,
    pub start_delay: u8,
}

impl Dma {
    fn new() -> Self {
        Self {
            active: false,
            byte: 0,
            start_delay: 0,
            value: 0,
        }
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
   pub struct Control: u8 {
        const BG_WINDOW                 = 0b00000001;
        const SPRITE_ENABLE             = 0b00000010;
        const SPRITE_SIZE               = 0b00000100;
        const BG_TILE_MAP_AREA          = 0b00001000;
        const BG_WINDOW_TILE_DATA_AREA  = 0b00010000;
        const WINDOW_ENABLE             = 0b00100000;
        const WINDOW_TILE_MAP_AREA      = 0b01000000;
        const LCD_PPU_ENABLE            = 0b10000000;

    }
}

#[derive(Debug, Clone, Copy)]
pub enum LcdMode {
    HBlank,
    VBlank,
    Oam,
    Transfer,
}

#[derive(Debug, Clone, Copy)]
pub enum FetchState {
    /// Determines which background/window tile to fetch pixels from
    Tile,

    /// First of the slice's two bitplanes is fetched
    Data0,

    /// Second of the slice's two bitplanes is fetched
    Data1,

    Push,

    Sleep,
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct Stat: u8 {
        const LYC_LY_EQ_FLAG        = 0b00000100;
        const HBLANK_INTERRUPT      = 0b00001000;
        const VBLANK_INTERRUPT      = 0b00010000;
        const OAM_INTERRUPT         = 0b00100000;
        const LYC_LY_EQ_INTERRUPT   = 0b01000000;
    }
}

#[derive(Debug)]
pub struct PixelFifoInfo {
    fetch_state: FetchState,
    fifo: VecDeque<Color>,
    x: u8,
}

impl PixelFifoInfo {
    fn new() -> PixelFifoInfo {
        PixelFifoInfo {
            fetch_state: FetchState::Tile,
            fifo: VecDeque::new(),
            x: 0,
        }
    }

    pub fn clear(&mut self) {
        self.fifo.clear();
    }

    pub fn set_x(&mut self, value: u8) {
        self.x = value
    }

    pub fn x(&self) -> u8 {
        self.x
    }

    pub fn set_fetch_state(&mut self, state: FetchState) {
        self.fetch_state = state
    }

    pub fn fetch_state(&self) -> FetchState {
        self.fetch_state
    }
}

/// Pixel Processing Unit
///
/// Used to display graphics
#[derive(Debug)]
pub struct Ppu {
    //Video RAM
    vram: [u8; 0x2000],

    //OAM
    oam: [OamEntry; 40],

    current_frame: u32,
    line_ticks: u32,

    pub dma: Dma,
    pub video_buffer: [Color; BUFFER_SIZE],
    pub pixel_fifo: PixelFifoInfo,

    // LCD status
    pub stat: Stat,

    pub mode: LcdMode,

    // LCD control
    pub control: Control,

    // Viewport X position
    scroll_x: u8,

    // Viewport Y position
    scroll_y: u8,

    // LCD Y coordinate (read only)
    ly: u8,

    // LY compare
    lyc: u8,

    // Window X position (Top left)
    window_x: u8,

    // Window Y position (Top left)
    window_y: u8,

    // Background palette (Non-CGB mode only)
    bg_palette: u8,

    // Object palettes data (Non-CGB mode only)
    object_palette: [u8; 2],
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            vram: [0; 0x2000],
            oam: [OamEntry::new(); 40],
            dma: Dma::new(),
            pixel_fifo: PixelFifoInfo::new(),
            line_ticks: 0,
            current_frame: 0,
            video_buffer: [Color::RGB(0, 0, 0); BUFFER_SIZE],
            stat: Stat::empty(),
            mode: LcdMode::Oam,
            control: Control::empty(),
            scroll_x: 0,
            scroll_y: 0,
            ly: 0,
            lyc: 0,
            window_x: 0,
            window_y: 0,
            bg_palette: 0,
            object_palette: [0; 2],
        }
    }

    pub fn dma_transferring(&self) -> bool {
        self.dma.active
    }

    pub fn set_dma_active(&mut self, value: bool) {
        self.dma.active = value;
    }

    pub fn dma_active(&self) -> bool {
        self.dma.active
    }

    pub fn set_dma_byte(&mut self, value: u8) {
        self.dma.byte = value;
    }

    pub fn dma_byte(&self) -> u8 {
        self.dma.byte
    }

    pub fn set_dma_start_delay(&mut self, value: u8) {
        self.dma.start_delay = value;
    }

    pub fn dma_start_delay(&self) -> u8 {
        self.dma.start_delay
    }

    pub fn set_dma_value(&mut self, value: u8) {
        self.dma.value = value;
    }

    pub fn dma_value(&self) -> u8 {
        self.dma.value
    }

    pub fn dma_start(&mut self, value: u8) {
        self.dma.active = true;
        self.dma.byte = 0;
        self.dma.start_delay = 2;
        self.dma.value = value;
    }

    pub fn set_line_ticks(&mut self, value: u32) {
        self.line_ticks = value;
    }

    pub fn line_ticks(&self) -> u32 {
        self.line_ticks
    }

    pub fn increment_line_ticks(&mut self) {
        self.line_ticks = self.line_ticks.wrapping_add(1);
    }

    pub fn current_frame(&self) -> u32 {
        self.current_frame
    }

    pub fn set_ly(&mut self, value: u8) {
        self.ly = value
    }

    pub fn ly(&self) -> u8 {
        self.ly
    }

    pub fn set_lyc(&mut self, value: u8) {
        self.lyc = value;
    }

    pub fn lyc(&self) -> u8 {
        self.lyc
    }

    pub fn set_scroll_x(&mut self, value: u8) {
        self.scroll_x = value;
    }

    pub fn scroll_x(&self) -> u8 {
        self.scroll_x
    }

    pub fn set_scroll_y(&mut self, value: u8) {
        self.scroll_y = value;
    }

    pub fn scroll_y(&self) -> u8 {
        self.scroll_y
    }

    pub fn set_window_x(&mut self, value: u8) {
        self.window_x = value;
    }

    pub fn window_x(&self) -> u8 {
        self.window_x
    }

    pub fn set_window_y(&mut self, value: u8) {
        self.window_y = value;
    }

    pub fn window_y(&self) -> u8 {
        self.window_y
    }

    pub fn set_stat_mode(&mut self, mode: LcdMode) {
        self.mode = mode
    }

    pub fn stat_mode(&self) -> LcdMode {
        self.mode
    }

    pub fn stat(&self) -> Stat {
        self.stat
    }

    pub fn control(&self) -> Control {
        self.control
    }

    pub fn write_oam(&mut self, addr: u16, value: u8) {
        if addr >= 0xFE00 {
            let index = ((addr - 0xFE00) % 40) as usize;
            let inner_index = ((addr - 0xFE00) % 4) as usize;
            match inner_index {
                0 => self.oam[index].y = value,
                1 => self.oam[index].x = value,
                2 => self.oam[index].tile = value,
                3 => self.oam[index].oam_attr = OamAttr::from_bits_truncate(value),
                _ => panic!("NOT AN INDEX"),
            }
        } else {
            let index = (addr % 40) as usize;
            let inner_index = (addr % 4) as usize;
            match inner_index {
                0 => self.oam[index].y = value,
                1 => self.oam[index].x = value,
                2 => self.oam[index].tile = value,
                3 => self.oam[index].oam_attr = OamAttr::from_bits_truncate(value),
                _ => panic!("NOT AN INDEX"),
            }
        }
    }

    pub fn read_oam(&self, addr: u16) -> u8 {
        let index = ((addr - 0xFE00) % 40) as usize;
        let inner_index = ((addr - 0xFE00) % 4) as usize;
        match inner_index {
            0 => self.oam[index].y,
            1 => self.oam[index].x,
            2 => self.oam[index].tile,
            3 => self.oam[index].oam_attr.bits(),
            _ => panic!("NOT AN INDEX"),
        }
    }

    pub fn write_vram(&mut self, addr: u16, value: u8) {
        self.vram[(addr - 0x8000) as usize] = value;
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        self.vram[(addr - 0x8000) as usize]
    }

    pub fn write_lcd(&mut self, addr: u16, value: u8) {
        let index: u8 = (addr - 0xFF40) as u8;

        match index {
            0x0 => self.control = Control::from_bits_truncate(value),
            0x1 => self.write_stat(value),
            0x2 => self.scroll_y = value,
            0x3 => self.scroll_x = value,
            0x4 => self.ly = value,
            0x5 => self.lyc = value,
            0x6 => {
                log::info!("DMA START");
                self.dma_start(value);
            }
            0x7 => {
                log::warn!("NOT IMPLEMENTED: {:#X}", addr);
            }
            0x8 => {
                log::warn!("NOT IMPLEMENTED: {:#X}", addr);
            }
            0x9 => {
                log::warn!("NOT IMPLEMENTED: {:#X}", addr);
            }
            0xA => self.window_y = value,
            0xB => self.window_x = value,
            _ => panic!("NOT AN INDEX"),
        }
    }

    pub fn read_lcd(&self, addr: u16) -> u8 {
        let index: u8 = (addr - 0xFF40) as u8;

        match index {
            0x0 => self.control.bits(),
            0x1 => self.read_stat(),
            0x2 => self.scroll_y,
            0x3 => self.scroll_x,
            0x4 => self.ly,
            0x5 => self.lyc,
            0x6 => {
                log::warn!("NOT IMPLEMENTED: {:#X}", addr);
                0
            }
            0xA => self.window_y,
            0xB => self.window_x,
            _ => panic!("NOT AN INDEX"),
        }
    }

    pub fn read_stat(&self) -> u8 {
        self.stat.bits() | self.mode as u8
    }

    pub fn write_stat(&mut self, value: u8) {
        self.stat = Stat::from_bits_truncate(value);

        self.mode = match value & 0b11 {
            0b00 => LcdMode::HBlank,
            0b01 => LcdMode::VBlank,
            0b10 => LcdMode::Oam,
            0b11 => LcdMode::Transfer,
            _ => panic!("NOT A MODE"),
        };
    }

    pub fn bg_tile_map_addr(&self) -> u16 {
        if self.control().contains(Control::BG_TILE_MAP_AREA) {
            0x9C00
        } else {
            0x9800
        }
    }

    pub fn bg_window_data_area(&self) -> u16 {
        if self.control().contains(Control::BG_WINDOW_TILE_DATA_AREA) {
            0x8000
        } else {
            0x8800
        }
    }
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}
