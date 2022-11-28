use crate::cpu::interrupts::InterruptType;

pub const SCALE: i32 = 4;
pub const SCREEN_WIDTH: u32 = 18 * 8 * (SCALE as u32);
pub const SCREEN_HEIGHT: u32 = 28 * 8 * (SCALE as u32);

pub const CLOCK_SPEED: usize = 4194304;
pub const MAX_CYCLES_PER_FRAME: usize = (CLOCK_SPEED as f32 / 59.7275) as usize;

pub const LINES_PER_FRAME: u8 = 154;
pub const TICKS_PER_LINE: u16 = 456;
pub const Y_RES: u8 = 144;
pub const X_RES: u8 = 160;
pub const BUFFER_SIZE: usize = (144 * 160) as usize;

pub const SI_HBLANK: u8 = (1 << 3);
pub const SI_VBLANK: u8 = (1 << 4);
pub const SI_OAM: u8 = (1 << 5);
pub const SI_LYC: u8 = (1 << 6);

pub const INTERRUPTS: [InterruptType; 5] = [
    InterruptType::VBlank,
    InterruptType::LcdStat,
    InterruptType::Timer,
    InterruptType::Serial,
    InterruptType::Joypad,
];

pub const INTERRUPT_ENABLE: u16 = 0xFFFF;
pub const INTERRUPT_FLAG: u16 = 0xFF0F;


pub const TARGET_FRAME_TIME: u32 = 1000 / 60;
