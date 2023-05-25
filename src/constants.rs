use crate::cpu::interrupts::InterruptType;
use sdl2::pixels::Color;

pub const SCALE: i32 = 4;
pub const SCREEN_WIDTH: u32 = 18 * 8 * (SCALE as u32);
pub const SCREEN_HEIGHT: u32 = 28 * 8 * (SCALE as u32);

pub const MAIN_SCREEN_WIDTH: u32 = 800;
pub const MAIN_SCREEN_HEIGHT: u32 = 640;

pub const CLOCK_SPEED: usize = 4_194_304;
pub const MAX_CYCLES_PER_FRAME: usize = (CLOCK_SPEED as f32 / 59.7275) as usize;
pub const PC_AFTER_BOOT: u16 = 0x100;

pub const LINES_PER_FRAME: u8 = 154;
pub const TICKS_PER_LINE: u32 = 456;
pub const Y_RESOLUTION: u8 = 144;
pub const X_RESOLUTION: u8 = 160;
pub const BUFFER_SIZE: usize = (144 * 160) as usize;


pub const INTERRUPTS: [InterruptType; 5] = [
    InterruptType::VBlank,
    InterruptType::LcdStat,
    InterruptType::Timer,
    InterruptType::Serial,
    InterruptType::Joypad,
];

pub const TARGET_FRAME_TIME: u32 = 1000 / 60;

pub const TILE_COLORS: [Color; 4] = [
    Color::RGB(255, 255, 255),
    Color::RGB(169, 169, 169),
    Color::RGB(84, 84, 84),
    Color::RGB(0, 0, 0),
];

// Memory Map Addresses
pub const SERIAL_TRASFER_DATA: u16 = 0xFF01;
pub const SERIAL_TRANSFER_CONTROL: u16 = 0xFF02;

pub const LCDC: u16 = 0xFF40;

pub const INTERRUPT_FLAG: u16 = 0xFF0F;
pub const INTERRUPT_ENABLE: u16 = 0xFFFF;
