#![allow(clippy::must_use_candidate)]
use crate::constants::*;
use log::warn;
use modular_bitfield::prelude::*;
use sdl2::pixels::Color;
use std::collections::VecDeque;

pub enum PaletteType {
    Background,
    Sprite0,
    Sprite1,
}

#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct SpriteAttribute {
    bg_window: B1,
    y_flip: B1,
    x_flip: B1,
    palette_number: B1,
    tile_vram_bank: B1,
    palette_number_cgb: B3,
}

#[derive(Debug, Copy, Clone)]
pub struct SpriteEntry {
    y: u8,
    x: u8,
    tile: u8,
    oam_attr: SpriteAttribute,
}

impl SpriteEntry {
    fn new() -> Self {
        Self {
            y: 0,
            x: 0,
            tile: 0,
            oam_attr: SpriteAttribute::new(),
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

#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Control {
    pub bg_window: B1,
    pub sprite_enable: B1,
    pub sprite_size: B1,
    pub bg_tile_map_area: B1,
    pub bg_window_tile_data_area: B1,
    pub window_enable: B1,
    pub window_tile_map_area: B1,
    pub lcd_ppu_enable: B1,
}

#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Stat {
    pub mode: B2,
    pub lyc_ly_compare: B1,
    pub hblank_interrupt_soruce: B1,
    pub vblank_interrupt_source: B1,
    pub oam_interrupt_source: B1,
    pub lyc_ly_interrupt_source: B1,
    empty: B1,
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

#[derive(Debug)]
pub struct PixelFifoInfo {
    pub fetch_state: FetchState,
    pub fifo: VecDeque<Color>,
    pub line_x: u8,
    pub pushed_x: u8,
    pub fetch_x: u8,
    pub fifo_x: u8,
    pub map_y: u8,
    pub map_x: u8,
    pub tile_data: u8,
    pub bg_window_data: [u8; 3],
}

impl PixelFifoInfo {
    fn new() -> PixelFifoInfo {
        PixelFifoInfo {
            fetch_state: FetchState::Tile,
            fifo: VecDeque::new(),
            line_x: 0,
            pushed_x: 0,
            fetch_x: 0,
            fifo_x: 0,
            tile_data: 0,
            map_y: 0,
            map_x: 0,
            bg_window_data: [0; 3],
        }
    }

    pub fn fifo(&self) -> VecDeque<Color> {
        self.fifo.clone()
    }

    pub fn push(&mut self, color: Color) {
        self.fifo.push_back(color);
    }

    pub fn pop_fifo(&mut self) -> Color {
        match self.fifo.pop_front() {
            Some(color) => color,
            None => panic!("NO PIXEL TO POP"),
        }
    }

    pub fn clear(&mut self) {
        self.fifo.clear();
    }

    pub fn set_line_x(&mut self, value: u8) {
        self.line_x = value
    }

    pub fn line_x(&self) -> u8 {
        self.line_x
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
    oam: [SpriteEntry; 40],

    current_frame: u32,
    line_ticks: u32,

    pub dma: Dma,
    pub video_buffer: [Color; BUFFER_SIZE],
    pub pixel_fifo: PixelFifoInfo,

    // LCD status
    pub stat: Stat,

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
    pub bg_palette: [Color; 4],

    // Sprite palettes data (Non-CGB mode only)
    pub sprite0_palette: [Color; 4],
    pub sprite1_palette: [Color; 4],
}

impl Ppu {
    pub fn new() -> Self {
        let mut ppu = Self {
            vram: [0; 0x2000],
            oam: [SpriteEntry::new(); 40],
            dma: Dma::new(),
            pixel_fifo: PixelFifoInfo::new(),
            line_ticks: 0,
            current_frame: 0,
            video_buffer: [Color::RGB(0, 0, 0); BUFFER_SIZE],

            stat: Stat::new(),
            control: Control::from_bytes([0x91]),

            scroll_x: 0,
            scroll_y: 0,
            ly: 0,
            lyc: 0,
            window_x: 0,
            window_y: 0,

            bg_palette: TILE_COLORS,
            sprite0_palette: TILE_COLORS,
            sprite1_palette: TILE_COLORS,
        };

        ppu.set_stat_mode(LcdMode::Oam);
        ppu
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
    pub fn set_map_y(&mut self, value: u8) {
        self.pixel_fifo.map_y = value;
    }

    pub fn map_y(&self) -> u8 {
        self.pixel_fifo.map_y
    }

    pub fn set_map_x(&mut self, value: u8) {
        self.pixel_fifo.map_x = value;
    }

    pub fn map_x(&self) -> u8 {
        self.pixel_fifo.map_x
    }

    pub fn stat(&self) -> Stat {
        self.stat
    }

    pub fn control(&self) -> Control {
        self.control
    }

    pub fn pixel_fifo(&self) -> VecDeque<Color> {
        self.pixel_fifo.fifo.clone()
    }

    pub fn fetch_x(&self) -> u8 {
        self.pixel_fifo.fetch_x
    }

    pub fn set_fetch_x(&mut self, value: u8) {
        self.pixel_fifo.fetch_x = value;
    }

    pub fn fifo_x(&self) -> u8 {
        self.pixel_fifo.fifo_x
    }

    pub fn set_fifo_x(&mut self, value: u8) {
        self.pixel_fifo.fifo_x = value;
    }

    pub fn pushed_x(&self) -> u8 {
        self.pixel_fifo.pushed_x
    }

    pub fn set_pushed_x(&mut self, value: u8) {
        self.pixel_fifo.pushed_x = value;
    }

    pub fn line_x(&self) -> u8 {
        self.pixel_fifo.line_x
    }

    pub fn set_line_x(&mut self, value: u8) {
        self.pixel_fifo.line_x = value;
    }

    pub fn tile_data(&self) -> u8 {
        self.pixel_fifo.tile_data
    }

    pub fn set_tile_data(&mut self, value: u8) {
        self.pixel_fifo.tile_data = value;
    }

    pub fn update_palette(&mut self, palette_type: PaletteType, palette_data: u8) {
        let palette_colors = match palette_type {
            PaletteType::Background => &mut self.bg_palette,
            PaletteType::Sprite0 => &mut self.sprite0_palette,
            PaletteType::Sprite1 => &mut self.sprite1_palette,
        };

        palette_colors[0] = TILE_COLORS[(palette_data & 0b11) as usize];
        palette_colors[1] = TILE_COLORS[((palette_data >> 2) & 0b11) as usize];
        palette_colors[2] = TILE_COLORS[((palette_data >> 4) & 0b11) as usize];
        palette_colors[3] = TILE_COLORS[((palette_data >> 6) & 0b11) as usize];
    }

    pub fn write_oam(&mut self, addr: u16, value: u8) {
        if addr >= 0xFE00 {
            let index = ((addr - 0xFE00) % 40) as usize;
            let inner_index = ((addr - 0xFE00) % 4) as usize;
            match inner_index {
                0 => self.oam[index].y = value,
                1 => self.oam[index].x = value,
                2 => self.oam[index].tile = value,
                3 => self.oam[index].oam_attr = SpriteAttribute::from_bytes([value]),
                _ => panic!("NOT AN INDEX"),
            }
        } else {
            let index = (addr % 40) as usize;
            let inner_index = (addr % 4) as usize;
            match inner_index {
                0 => self.oam[index].y = value,
                1 => self.oam[index].x = value,
                2 => self.oam[index].tile = value,
                3 => self.oam[index].oam_attr = SpriteAttribute::from_bytes([value]),
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
            3 => self.oam[index].oam_attr.into_bytes()[0],
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
            0x0 => self.control = Control::from_bytes([value]),
            0x1 => self.stat = Stat::from_bytes([value]),
            0x2 => self.scroll_y = value,
            0x3 => self.scroll_x = value,
            0x4 => self.ly = value,
            0x5 => self.lyc = value,
            0x6 => {
                log::info!("DMA START");
                self.dma_start(value);
            }
            0x7 => {
                self.update_palette(PaletteType::Background, value);
            }
            0x8 => {
                self.update_palette(PaletteType::Sprite0, value & 0b11111100);
            }
            0x9 => {
                self.update_palette(PaletteType::Sprite1, value & 0b11111100);
            }
            0xA => self.window_y = value,
            0xB => self.window_x = value,
            _ => panic!("NOT AN INDEX"),
        }
    }

    pub fn read_lcd(&self, addr: u16) -> u8 {
        let index: u8 = (addr - 0xFF40) as u8;

        match index {
            0x0 => self.control.bytes[0],
            0x1 => self.stat.bytes[0],
            0x2 => self.scroll_y,
            0x3 => self.scroll_x,
            0x4 => self.ly,
            0x5 => self.lyc,
            0x6 => {
                log::warn!("MAY BE A BUG HERE: DMA: {:?}", self.dma.value);
                self.dma.value
            }
            0xA => self.window_y,
            0xB => self.window_x,
            _ => panic!("NOT AN INDEX"),
        }
    }

    pub fn bg_tile_map_addr(&self) -> u16 {
        if self.control().bg_tile_map_area() == 1 {
            0x9C00
        } else {
            0x9800
        }
    }

    pub fn bg_window_data_area(&self) -> u16 {
        if self.control().bg_window_tile_data_area() == 1 {
            0x8000
        } else {
            0x8800
        }
    }

    pub fn window_map_area(&self) -> u16 {
        if self.control().window_tile_map_area() == 1 {
            0x9C00
        } else {
            0x9800
        }
    }

    pub fn stat_mode(&self) -> LcdMode {
        let bits = self.stat().bytes[0] & 0b11;

        match bits {
            0 => LcdMode::HBlank,
            1 => LcdMode::VBlank,
            2 => LcdMode::Oam,
            3 => LcdMode::Transfer,
            _ => panic!("NOT AN LCD MODE"),
        }
    }

    pub fn set_stat_mode(&mut self, mode: LcdMode) {
        let bits = self.stat.bytes[0] & !0b11;
        let value = bits | mode as u8;
        self.stat = Stat::from_bytes([value]);
    }
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}
