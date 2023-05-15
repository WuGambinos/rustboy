use crate::constants::{
    MAIN_SCREEN_HEIGHT, MAIN_SCREEN_WIDTH, SCALE, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_COLORS, X_RES,
    Y_RES,
};
use crate::interconnect::Interconnect;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

pub fn init_window(sdl_context: &Sdl) -> WindowCanvas {
    let video_subsystem = sdl_context.video().expect("failed to access subsystem");
    let window = video_subsystem
        .window("rust-sdl2 demo", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("failed to create window");

    let canvas: WindowCanvas = window
        .into_canvas()
        .build()
        .expect("failed to get sdl canvas");

    canvas
}

pub fn init_main_window(sdl_context: &Sdl) -> WindowCanvas {
    let video_subsystem = sdl_context.video().expect("failed to access subsystem");
    let window = video_subsystem
        .window("rust-sdl2 demo", MAIN_SCREEN_WIDTH, MAIN_SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("failed to create window");

    let canvas: WindowCanvas = window
        .into_canvas()
        .build()
        .expect("failed to get sdl canvas");

    canvas
}

pub fn main_window(canvas: &mut WindowCanvas, interconnect: &Interconnect) {
    let video_buffer = interconnect.ppu.video_buffer;
    for line_num in 0..Y_RES {
        for x in 0..X_RES {
            let new_x = i32::from(u16::from(x) * (SCALE as u16));
            let new_y = i32::from(u16::from(line_num) * (SCALE as u16));
            let w: u32 = SCALE as u32;
            let h: u32 = SCALE as u32;

            let index = (u32::from(x) + (u32::from(line_num) * u32::from(X_RES))) as usize;
            let color = video_buffer[index];
            canvas.set_draw_color(color);
            canvas
                .fill_rect(Rect::new(new_x, new_y, w, h))
                .expect("Rectangle could not be filled");
        }
    }

    canvas.present();
}

pub fn debug_window(canvas: &mut WindowCanvas, interconnect: &Interconnect) {
    let mut x_draw: i32 = 0;
    let mut y_draw: i32 = 0;
    let mut tile_num: u16 = 0;

    canvas.set_draw_color(Color::RGB(17, 17, 17));
    //canvas.clear();
    canvas
        .fill_rect(sdl2::rect::Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT))
        .expect("Rectangle could not be filled");

    let addr: u16 = 0x8000;

    for y in 0..24 {
        for x in 0..16 {
            display_tile(
                canvas,
                interconnect,
                addr,
                tile_num,
                x_draw + (x * SCALE), //as u32,
                y_draw + (y * SCALE), //as u32,
            );
            x_draw += 8 * SCALE;
            tile_num += 1;
        }
        y_draw += 8 * SCALE;
        x_draw = 0;
    }
    canvas.present();
}

fn display_tile(
    canvas: &mut WindowCanvas,
    interconnect: &Interconnect,
    start_loc: u16,
    tile_num: u16,
    x: i32,
    y: i32,
) {
    for tile_y in (0..16).step_by(2) {
        let addr: u16 = start_loc + (tile_num * 16) + tile_y;

        // Get First BYTE
        let second_byte: u8 = interconnect.read_mem(addr);

        // Get Second BYTE
        let first_byte: u8 = interconnect.read_mem(addr + 1);

        // Index for tile color
        let mut color: u8;

        // Iterate over bits of first and second byte
        for bit in (0..8).rev() {
            let first_bit = (first_byte >> bit) & 1;
            let second_bit = (second_byte >> bit) & 1;
            if first_bit == 0 && second_bit == 0 {
                color = 0;
            } else if first_bit == 0 && second_bit == 1 {
                color = 1;
            } else if first_bit == 1 && second_bit == 0 {
                color = 2;
            } else {
                color = 3;
            }

            let new_x = x + ((7 - bit) * SCALE);
            let new_y = y + ((i32::from(tile_y)) / 2 * SCALE);

            let w = SCALE as u32;
            let h = SCALE as u32;

            canvas.set_draw_color(TILE_COLORS[color as usize]);
            canvas
                .fill_rect(sdl2::rect::Rect::new(new_x, new_y, w, h))
                .expect("Rectangle could not be filled");
        }
    }
}
