// ImGui window constants
pub const SCALE: i32 = 3;
pub const TILE_SCALE: i32 = 2;

pub const WINDOW_WIDTH: f32 = 1200.0;
pub const WINDOW_HEIGHT: f32 = 700.0;

pub const TILE_SCREEN_WIDTH: u32 = 18 * 8 * (TILE_SCALE as u32);
pub const TILE_SCREEN_HEIGHT: u32 = 28 * 8 * (TILE_SCALE as u32);

pub const GB_SCREEN_WIDTH: u32 = 160 * (SCALE as u32);
pub const GB_SCREEN_HEIGHT: u32 = 150 * (SCALE as u32);
pub const GB_SCREEN_X: f32 = 0.0;
pub const GB_SCREEN_Y: f32 = 50.0;
pub const GB_POS: [f32; 2] = [GB_SCREEN_X, GB_SCREEN_Y];
pub const GB_SCREEN_SIZE: [f32; 2] = [
    (GB_SCREEN_WIDTH + 10) as f32,
    (GB_SCREEN_HEIGHT + 10) as f32,
];
