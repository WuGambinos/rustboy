use crate::interconnect::Interconnect;

use sdl2::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

const SCALE: i32 = 4;
const SCREEN_WIDTH: u32 = 16 * 8 * (SCALE as u32); 
const SCREEN_HEIGHT: u32 = 32 * 8 * (SCALE as u32);

static TILE_COLORS: [Color; 4] = [
    Color::RGB(255, 255, 255),
    Color::RGB(169, 169, 169),
    Color::RGB(84, 84, 84),
    Color::RGB(0, 0, 0),
];

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

pub fn debug_window(canvas: &mut WindowCanvas, interconnect: &Interconnect) {
    let mut x_draw = 0;
    let mut y_draw = 0;
    let mut tile_num = 0;

    let _w: u32 = 16 * 8 * 4;
    let _h: u32 = 32 * 8 * 4;

    canvas.set_draw_color(Color::RGB(17, 17, 17));
    //canvas.clear();
    canvas.fill_rect(sdl2::rect::Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT));
    let addr: u16 = 0x8000;

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
    canvas.present();
}

fn display_tile(
    canvas: &mut WindowCanvas,
    interconnect: &Interconnect,
    start_loc: u16,
    tile_num: u16,
    x: u32,
    y: u32,
) {
    for tile_y in (0..16).step_by(2) {
        let addr: u16 = start_loc + (tile_num * 16) + tile_y;

        /*let b1: u8 = interconnect.read_mem(addr);
            let b2: u8 = interconnect.read_mem(addr + 1);

            for bit in (0..7).rev() {
                let mut hi: u8 = b1 & (1 << bit) << 1;
                let mut lo: u8 = b2 & (1 << bit);

                hi = (hi == 0) as u8;
                hi = (hi == 0) as u8;
                hi = hi << 1;

                lo = (lo == 0) as u8;
                lo = (lo == 0) as u8;

                let color: u8 = hi | lo;

                let new_x = (x as i32) + ((7 - bit) * SCALE);
                let new_y = (y as i32) + ((tile_y as i32) / 2 * SCALE);

                let w = SCALE as i32;
                let h = SCALE as i32;
                //println!("ADDR_VALUE: {:#X} ADDR1_VALUE {:#X} ADDR: {:#X} ADDR+1: {:#X} HI: {:#X} LO: {:#X} COLOR: {}",b1, b2, addr, addr +1, hi, lo, color);
            canvas.draw_rectangle(new_x, new_y, w, h, TILE_COLORS[color as usize]);
            p
        }*/
        // Get First BYTE
        let second_byte: u8 = interconnect.read_mem(addr);

        // Get Second BYTE
        let first_byte: u8 = interconnect.read_mem(addr + 1);

        // Index for tile color
        let mut color: u8 = 0;

        // Iterate over bits of first and second byte
        for bit in (0..8).rev() {
            let first_bit = (first_byte >> bit) & 1;
            let second_bit = (second_byte >> bit) & 1;
            if first_bit == 0 && second_bit == 0 {
                color = 0
            } else if first_bit == 0 && second_bit == 1 {
                color = 1;
            } else if first_bit == 1 && second_bit == 0 {
                color = 2
            } else {
                color = 3;
            }

            let new_x = (x as i32) + ((7 - bit) * SCALE);
            let new_y = (y as i32) + ((tile_y as i32) / 2 * SCALE);

            let w = SCALE as u32;
            let h = SCALE as u32;

            
            canvas.set_draw_color(TILE_COLORS[color as usize]);
            canvas.fill_rect(sdl2::rect::Rect::new(new_x, new_y, w, h));
            
        }
    }
}
