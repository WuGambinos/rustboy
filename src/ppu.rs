use crate::constants::*;
use modular_bitfield::prelude::*;
use sdl2::pixels::Color;
use std::collections::LinkedList;

#[derive(Debug, Clone)]
pub enum FetchState {
    Tile,
    Data0,
    Data1,
    Idle,
    Push,
}

#[derive(Debug, Clone)]
pub struct PixelFifoContext {
    fetch_state: FetchState,
    pixel_fifo: LinkedList<Color>,
    line_x: u8,
    pushed_x: u8,
    fetch_x: u8,
    bgw_fetch_data: [u8; 3],
    fetch_entry_data: [u8; 6],
    map_y: u8,
    map_x: u8,
    tile_y: u8,
    fifo_x: u8,
}

impl PixelFifoContext {
    fn new() -> Self {
        PixelFifoContext {
            fetch_state: FetchState::Tile,
            pixel_fifo: LinkedList::new(),
            line_x: 0,
            pushed_x: 0,
            fetch_x: 0,
            bgw_fetch_data: [0; 3],
            fetch_entry_data: [0; 6],
            map_y: 0,
            map_x: 0,
            tile_y: 0,
            fifo_x: 0,
        }
    }
}

/// Single Entry in OAM (Object Atribute Memory)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct OamAttr {
    // BG and Window over OBJ
    // (0=No, 1=BG and Windows colors 1-3 over the OBJ)
    bg_window: B1,

    // Y Flip
    // (0=Normal, 1=Vertically Mirrored)
    y_flip: B1,

    // X Flip
    // (0=Nomral, 1=Horizontally Mirrored)
    x_flip: B1,

    /// Palette Number
    /// **Non CGB Mode only** (0=OBP0, 1=OBP1)
    pn: B1,

    // Tile VRAM-BANK ***CGB Mode only**
    // (0=Bank 0, 1=Bank1)
    tile_vram_bank: B1,

    // Palette Number **CGB Mode Only**
    // (OBP0-7)
    cbg_pn: B3,
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
            oam_attr: OamAttr::new(),
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

/// Pixel Processing Unit
///
/// Used to display graphics
#[derive(Debug)]
pub struct Ppu {
    //Video RAM
    vram: [u8; 0x2000],

    //OAM
    oam: [OamEntry; 40],

    pub dma: Dma,

    pub current_frame: u32,
    pub line_ticks: u32,
    pub video_buffer: [Color; BUFFER_SIZE],
    pfc: PixelFifoContext,
}

impl Ppu {
    /// Constructor
    pub fn new() -> Self {
        Self {
            vram: [0; 0x2000],
            oam: [OamEntry::new(); 40],
            dma: Dma::new(),
            line_ticks: 0,
            current_frame: 0,
            video_buffer: [Color::RGB(0, 0, 0); BUFFER_SIZE],
            pfc: PixelFifoContext::new(),
        }
    }

    pub fn dma_transferring(&self) -> bool {
        self.dma.active
    }

    pub fn set_dma_active(&mut self, value: bool) {
        self.dma.active = value
    }

    pub fn dma_active(&self) -> bool {
        self.dma.active
    }

    pub fn set_dma_byte(&mut self, value: u8) {
        self.dma.byte = value
    }

    pub fn dma_byte(&self) -> u8 {
        self.dma.byte
    }

    pub fn set_dma_start_delay(&mut self, value: u8) {
        self.dma.start_delay = value
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

    pub fn set_line_ticks(&mut self, value: u32) {
        self.line_ticks = value;
    }

    pub fn line_ticks(&self) -> u32 {
        self.line_ticks
    }

    pub fn increase_line_ticks(&mut self) {
        self.line_ticks = self.line_ticks.wrapping_add(1);
    }

    pub fn current_frame(&self) -> u32 {
        self.current_frame
    }

    pub fn set_fetch_state(&mut self, state: FetchState) {
        self.pfc.fetch_state = state;
    }

    pub fn fetch_state(&self) -> &FetchState {
         &self.pfc.fetch_state
    }

    pub fn set_line_x(&mut self, value: u8) {
        self.pfc.line_x = value;
    }

    pub fn line_x(&self) -> u8 {
        self.pfc.line_x
    }

    pub fn set_pushed_x(&mut self, value: u8) {
        self.pfc.pushed_x = value;
    }

    pub fn pushed_x(&self) -> u8 {
        self.pfc.pushed_x
    }

    pub fn set_fetch_x(&mut self, value: u8) {
        self.pfc.fetch_x = value;
    }

    pub fn fetch_x(&self) -> u8 {
        self.pfc.fetch_x
    }

    pub fn set_map_y(&mut self, value: u8) {
        self.pfc.map_y = value;
    }

    pub fn map_y(&self) -> u8 {
        self.pfc.map_y
    }

    pub fn set_map_x(&mut self, value: u8) {
        self.pfc.map_x = value;
    }

    pub fn map_x(&self) -> u8 {
        self.pfc.map_x
    }

    pub fn set_tile_y(&mut self, value: u8) {
        self.pfc.tile_y = value;
    }

    pub fn tile_y(&self) -> u8 {
        self.pfc.tile_y
    }

    pub fn set_fifo_x(&mut self, value: u8) {
        self.pfc.fifo_x = value;
    }

    pub fn fifo_x(&self) -> u8 {
        self.pfc.fifo_x
    }

    pub fn pixel_fifo(&self) -> LinkedList<Color> {
        self.pfc.pixel_fifo.clone()
    }

    pub fn pixel_fifo_push(&mut self, color: Color) {
        self.pfc.pixel_fifo.push_back(color);
    }

    pub fn pixel_fifo_pop(&mut self) -> Color {
        match self.pfc.pixel_fifo.pop_front() {
            Some(color) => {
                 color
            }
            None => panic!("NO VALUE TO POP"),
        }
    }

    pub fn set_bgw_fetch_data(&mut self, index: usize, value: u8) {
        self.pfc.bgw_fetch_data[index] = value;
    }

    pub fn bgw_fetch_data(&self) -> [u8; 3] {
        self.pfc.bgw_fetch_data
    }

    pub fn write_oam(&mut self, addr: u16, value: u8) {
        if addr >= 0xFE00 {
            let index = ((addr - 0xFE00) % 40) as usize;
            let inner_index = ((addr - 0xFE00) % 4) as usize;
            match inner_index {
                0 => self.oam[index].y = value,
                1 => self.oam[index].x = value,
                2 => self.oam[index].tile = value,
                3 => self.oam[index].oam_attr = OamAttr::from_bytes([value]),
                _ => panic!("NOT AN INDEX"),
            }
        } else {
            let index = (addr % 40) as usize;
            let inner_index = (addr % 4) as usize;
            match inner_index {
                0 => self.oam[index].y = value,
                1 => self.oam[index].x = value,
                2 => self.oam[index].tile = value,
                3 => self.oam[index].oam_attr = OamAttr::from_bytes([value]),
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
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}
