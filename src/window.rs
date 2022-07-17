extern crate sdl2;
use crate::interconnect::Interconnect;
/*use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;*/
use raylib::prelude::*;

/*static tile_colors: [Color; 4] = [
    Color::RGB(255, 255, 255),
    Color::RGB(170, 170, 170),
    Color::RGB(85, 85, 85),
    Color::RGB(0, 0, 0),
];*/

static tile_colors: [Color; 4] = [
    Color::WHITE,
    Color::GRAY,
    Color::DARKGRAY,
    Color::BLACK,
];

static SCALE: i32 = 4;

pub fn update_debug_window(canvas: &mut RaylibDrawHandle, interconnect: &Interconnect) {
    let mut x_draw = 0;
    let mut y_draw = 0;
    let mut tile_num = 0;

    let w: u32 = 16 * 8 * 4;
    let h: u32 = 32 * 8 * 4;

    canvas.clear_background(Color::GRAY);

    let addr: u16 = 0x8000;

    // 384 tiles 24x16

    for y in 0..24 {
        for x in 0..16 {
            display_tile(
                canvas,
                interconnect,
                addr,
                tile_num,
                (x_draw + (x * SCALE)) as u32,
                (y_draw + (y * SCALE)) as u32,
            );
            x_draw += 8 * SCALE;
            tile_num += 1;
        }
        y_draw += 8 * SCALE;
        x_draw = 0;
    }
}

pub fn display_tile(
    canvas: &mut RaylibDrawHandle,
    interconnect: &Interconnect,
    start_loc: u16,
    tile_num: u16,
    x: u32,
    y: u32,
) {
    for tile_y in (0..16).step_by(2) {
        let addr: u16 = start_loc + (tile_num * 16) + tile_y;

        // Get first byte
        let b1: u8 = interconnect.read_mem(addr);

        // Get second byte
        let b2: u8 = interconnect.read_mem(addr + 1);

        for bit in (0..7).rev() {
            let hi: u8 = !!(b1 & (1 << bit)) << 1;
            let low: u8 = !!(b2 & (1 << bit));

            let color: u8 = hi | low;

            let new_x = (x as i32) + ((7 - bit) * SCALE);
            let new_y = (y as i32) + ((tile_y as i32) / 2 * SCALE);

            let w = SCALE as i32;
            let h = SCALE as i32;

            canvas.draw_rectangle(new_x, new_y, w, h, tile_colors[color as usize]);
        }
    }
}

/*
pub fn update_debug_window(canvas: &mut Canvas<Window>, interconnect: &Interconnect) {
    let mut x_draw = 0;
    let mut y_draw = 0;
    let mut tile_num = 0;

    let w: u32 = 16 * 8 * 4;
    let h: u32 = 32 * 8 * 4;

    // Create rectangle that is the size of screen
    let rc = Rect::new(0, 0, w, h);

    // Give rectangle Gray color
    canvas.set_draw_color(Color::RGB(105, 105, 105));

    // Fill Screen with gray rectangle
    canvas.fill_rect(rc);

    canvas.present();

    let addr: u16 = 0x8000;

    // 384 tiles 24x16

    for y in 0..24 {
        for x in 0..16 {
            display_tile(
                canvas,
                interconnect,
                addr,
                tile_num,
                (x_draw + (x * SCALE)) as u32,
                (y_draw + (y * SCALE)) as u32,
            );
            x_draw += 8 * SCALE;
            tile_num += 1;
        }
        y_draw += 8 * SCALE;
        x_draw = 0;
    }
}*/

/*
pub fn display_tile(
    canvas: &mut Canvas<Window>,
    interconnect: &Interconnect,
    start_loc: u16,
    tile_num: u16,
    x: u32,
    y: u32,
) {
    for tile_y in (0..16).step_by(2) {
        let addr: u16 = start_loc + (tile_num * 16) + tile_y;

        // Get first byte
        let b1: u8 = interconnect.read_mem(addr);

        // Get second byte
        let b2: u8 = interconnect.read_mem(addr + 1);

        for bit in (0..7).rev() {
            let hi: u8 = !!(b1 & (1 << bit)) << 1;
            let low: u8 = !!(b2 & (1 << bit));

            let color: u8 = hi | low;

            let new_x = (x as i32) + ((7 - bit) * SCALE);
            let new_y = (y as i32) + ((tile_y as i32) / 2 * SCALE);

            let w = SCALE as u32;
            let h = SCALE as u32;

            let rc = Rect::new(new_x, new_y, w, h);
            canvas.set_draw_color(tile_colors[color as usize]);

            canvas.fill_rect(rc);

            canvas.present();
        }
    }
}*/
