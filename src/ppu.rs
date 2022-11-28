use modular_bitfield::prelude::*;
use crate::constants::*;

/// Single Entry in OAM (Object Atribute Memory)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct OamEntry {
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

#[derive(Debug)]
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
    pub video_buffer: [u8; BUFFER_SIZE],
    pub prev_frame_time: u32,
    pub start_timer: u32,
    pub frame_count: u32,
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
            video_buffer: [0; BUFFER_SIZE],
            prev_frame_time: 0,
            start_timer: 0,
            frame_count: 0,
        }
    }


    pub fn increase_line_ticks(&mut self) {
        self.line_ticks = self.line_ticks.wrapping_add(1);
    }

    pub fn write_oam(&mut self, addr: u16, value: u8) {
        let index = ((addr - 0xFE00) % 40) as usize;
        self.oam[index] = OamEntry::from_bytes([value]);
    }

    pub fn read_oam(&self, addr: u16) -> u8 {
        let index = ((addr - 0xFE00) % 40) as usize;
        self.oam[index].into_bytes()[0]
    }

    pub fn write_vram(&mut self, addr: u16, value: u8) {
        self.vram[(addr - 0x8000) as usize] = value;
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        self.vram[(addr - 0x8000) as usize]
    }

    pub fn dma_transferring(&self) -> bool {
        self.dma.active
    }

}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}
