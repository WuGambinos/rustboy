pub const SCALE: i32 = 4;
pub const SCREEN_WIDTH: u32 = 18 * 8 * (SCALE as u32);
pub const SCREEN_HEIGHT: u32 = 28 * 8 * (SCALE as u32);

pub const CLOCK_SPEED: usize = 4194304;
pub const MAX_CYCLES_PER_FRAME: usize = (CLOCK_SPEED as f32 / 59.7275) as usize;


