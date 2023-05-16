#![allow(clippy::must_use_candidate)]
use crate::ppu::Ppu;
use log::{debug, info, warn};
use sdl2::pixels::Color;

#[derive(Debug)]
pub enum LcdMode {
    HBlank = 0,
    VBlank = 1,
    OAM = 2,
    Transfer = 3,
}

const DEFAULT_COLORS: [Color; 4] = [
    Color::RGB(255, 255, 255),
    Color::RGB(169, 169, 169),
    Color::RGB(84, 84, 84),
    Color::RGB(0, 0, 0),
];

#[allow(dead_code)]
#[derive(Debug)]
pub struct Lcd {
    ///Bit 7 LCD and PPU enable    0=Off, 1=On
    ///
    ///Bit 6 Window tile map area    0=9800-9BFF, 1=9C00-9FFF
    ///
    ///Bit 5 Window enable    0=Off, 1=On
    ///
    ///Bit 4 BG and Window tile data area    0=8800-97FF, 1=8000-8FFF
    ///
    ///Bit 3 BG tile map area    0=9800-9BFF, 1=9C00-9FFF
    ///
    ///Bit 2 OBJ size    0=8x8, 1=8x16
    ///
    ///Bit 1 OBJ enable    0=Off, 1=On
    ///
    ///Bit 0 BG and Window enable/priority    0=Off, 1=On
    lcdc: u8,

    /// Bit 6 - LYC = LY STAT interrupt source
    ///
    /// Bit 5 - Mode 2 OAM STAT interrupt source
    ///
    /// Bit 4 - Mode 1 VBlank STAT Interrupt source
    ///
    /// Bit 3 - Mode 0 HBlank STAT Interrupt source
    ///
    /// Bit 2 - LYC = LY Flag
    ///
    /// Bit 1-0 - Mode Flag
    ///
    /// 0 - HBlank
    ///
    /// 1 - VBlank
    ///
    /// 2 -  Searching OAM
    ///
    /// 3 - Transferring Data to LCD Controller
    lcd_stat: u8,

    /// Specity the top left y coordinate of the visiable 16x144 pixel area
    /// within the 256x256 pixels BG map
    scy: u8,

    /// Specity the top left x coordinate of the visiable 16x144 pixel area
    /// within the 256x256 pixels BG map
    scx: u8,

    /// Indicates currently horizontal line, which might be about to be drawn
    /// ,or just drawn. values between 144 and 153 indicate VBlank period
    ly: u8,

    /// When LYC=LY, the "LYC=LY" flag in the STAT register is set and (if enabled)
    /// a STAT interrupt is requested
    lyc: u8,

    /// Specify upper left y coordinate of window
    wy: u8,

    /// Specicy upper left (x+7) coordinate of window
    wx: u8,

    /// OAM DMA source address & start
    dma: u8,

    /// Background Palette(Non-CGB Mode only)
    bg_palette: u8,

    /// Object Palette 0, 1 - These registesr assign gray shades to the color indexes
    /// of the OBJs that use the coressponding palette
    pub obj_palette: [u8; 2],

    pub bg_colors: [Color; 4],
    pub sp1_colors: [Color; 4],
    pub sp2_colors: [Color; 4],
}

impl Lcd {
    pub fn new() -> Self {
        let mut initial_state = Self {
            lcdc: 0x91,
            lcd_stat: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            wy: 0,
            wx: 0,

            dma: 0,
            bg_palette: 0xFC,

            obj_palette: [0xFF; 2],

            bg_colors: [Color::RGB(0, 0, 0); 4],
            sp1_colors: [Color::RGB(0, 0, 0); 4],
            sp2_colors: [Color::RGB(0, 0, 0); 4],
        };

        for (i, color) in DEFAULT_COLORS.iter().enumerate() {
            initial_state.bg_colors[i] = *color;
            initial_state.sp1_colors[i] = *color;
            initial_state.sp2_colors[i] = *color;
        }

        initial_state
    }

    pub fn read(&self, addr: u16) -> u8 {
        let index: u8 = (addr - 0xFF40) as u8;

        match index {
            0 => self.lcdc,
            1 => self.lcd_stat,
            2 => self.scy,
            3 => self.scx,
            4 => self.ly,
            5 => self.lyc,
            6 => self.dma,
            0xA => self.wy,
            0xB => self.wx,
            _ => panic!("Not an index"),
        }
    }

    pub fn write(&mut self, ppu: &mut Ppu, addr: u16, value: u8) {
        let index: u8 = (addr - 0xFF40) as u8;

        match index {
            0x0 => self.lcdc = value,
            0x1 => self.lcd_stat = value,
            0x2 => self.scy = value,
            0x3 => self.scx = value,
            0x4 => self.ly = value,
            0x5 => self.lyc = value,
            0x6 => {
                info!("DMA START");
                self.dma_start(ppu, value);
            }
            0x7 => self.update_palette(value, 0),
            0x8 => self.update_palette(value & 0b1111_1100, 1),
            0x9 => self.update_palette(value & 0b1111_1100, 2),
            0xA => self.wy = value,
            0xB => self.wx = value,
            _ => panic!("Not an index"),
        }
    }

    pub fn dma_start(&mut self, ppu: &mut Ppu, value: u8) {
        ppu.dma.active = true;
        ppu.dma.byte = 0;
        ppu.dma.start_delay = 2;
        ppu.dma.value = value;
    }

    pub fn update_palette(&mut self, pal_data: u8, pal: u8) {
        let mut pal_colors = self.bg_colors;
        match pal {
            1 => pal_colors = self.sp1_colors,
            2 => pal_colors = self.sp2_colors,
            _ => warn!("NOT A VALID PAL: {}", pal),
        }

        pal_colors[0] = DEFAULT_COLORS[(pal_data & 0b11) as usize];
        pal_colors[1] = DEFAULT_COLORS[((pal_data >> 2) & 0b11) as usize];
        pal_colors[2] = DEFAULT_COLORS[((pal_data >> 4) & 0b11) as usize];
        pal_colors[3] = DEFAULT_COLORS[((pal_data >> 6) & 0b11) as usize];
    }

    pub fn set_lcd_stat(&mut self, value: u8) {
        self.lcd_stat = value;
    }

    pub fn lcd_stat(&self) -> u8 {
        self.lcd_stat
    }

    pub fn set_lcdc(&mut self, value: u8) {
        self.lcdc = value;
    }

    pub fn lcdc(&self) -> u8 {
        self.lcdc
    }

    pub fn set_ly(&mut self, value: u8) {
        self.ly = value;
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

    pub fn set_scx(&mut self, value: u8) {
        self.scx = value;
    }

    pub fn scx(&self) -> u8 {
        self.scx
    }

    pub fn set_scy(&mut self, value: u8) {
        self.scy = value;
    }

    pub fn scy(&self) -> u8 {
        self.scy
    }

    pub fn set_wy(&mut self, value: u8) {
        self.wy = value;
    }

    pub fn wy(&mut self) -> u8 {
        self.wy
    }

    pub fn set_wx(&mut self, value: u8) {
        self.wx = value;
    }

    pub fn wx(&mut self) -> u8 {
        self.wx
    }

    pub fn set_dma(&mut self, value: u8) {
        self.dma = value;
    }

    pub fn dma(&self) -> u8 {
        self.dma
    }

    /************************************************************
     * LCDC Functions
     * **********************************************************/

    /// Check if background and window should be enabled
    pub fn lcdc_bgw_enabled(&self) -> bool {
        self.lcdc & 1 == 1
    }

    /// Check if sprites need to be displayed or not
    pub fn lcdc_obj_enabled(&self) -> bool {
        (self.lcdc >> 1) & 1 == 1
    }

    pub fn lcdc_obj_height(&self) -> u8 {
        let bit = (self.lcdc >> 2) & 1;

        if bit == 0 {
            8
        } else {
            16
        }
    }

    pub fn lcdc_bg_tile_map_addr(&self) -> u16 {
        let bit = (self.lcdc >> 3) & 1;

        if bit == 0 {
            0x9800
        } else {
            0x9C00
        }
    }

    pub fn lcdc_bgw_data_area(&self) -> u16 {
        let bit = (self.lcdc >> 4) & 1;

        if bit == 0 {
            0x8800
        } else {
            0x8000
        }
    }

    pub fn lcdc_window_enable(&self) -> bool {
        (self.lcdc >> 5) & 1 == 1
    }

    pub fn lcdc_window_map_area(&self) -> u16 {
        let bit = (self.lcdc >> 6) & 1;

        if bit == 0 {
            0x9800
        } else {
            0x9C00
        }
    }

    pub fn lcdc_lcd_enable(&self) -> bool {
        (self.lcdc >> 7) & 1 == 1
    }

    /************************************************************
     * STAT Functions
     * **********************************************************/

    pub fn lcd_stat_mode(&self) -> LcdMode {
        let bits = self.lcd_stat & 0b11;

        match bits {
            0 => LcdMode::HBlank,
            1 => LcdMode::VBlank,
            2 => LcdMode::OAM,
            3 => LcdMode::Transfer,
            _ => panic!("Not an LCD Mode"),
        }
    }

    pub fn set_lcd_stat_mode(&mut self, mode: u8) {
        self.lcd_stat &= !0b11;
        self.lcd_stat |= mode;
    }

    pub fn lyc_ly_flag(&self) -> bool {
        (self.lcd_stat >> 2) & 1 == 1
    }

    pub fn set_lyc_ly_flag(&mut self, on: u8) {
        if on == 1 {
            self.lcd_stat |= 1 << 2;
        } else {
            self.lcd_stat &= !(1 << 2);
        }
    }

    pub fn lcd_stat_interrupt(&mut self, stat_interrupt: u8) -> bool {
        self.lcd_stat & stat_interrupt != 0
    }

    pub fn log_lcd(&self) {
        debug!("LCDC: {:X} MODE: {:?} STAT: {:#X} SCY: {:#X} SCX: {:#X} LY: {:#X} LYC: {:#X} WY: {:#X} WX: {:#X} DMA: {:#X}",
    self.lcdc,
    self.lcd_stat_mode(),
    self.lcd_stat,
    self.scy,
    self.scx,
    self.ly,
    self.lyc,
    self.wy,
    self.wx,
    self.dma,
);
    }
}
