mod cpu;
mod gameboy;
mod interconnect;
mod mmu;
mod ppu;
mod window;

pub use cpu::Cpu;
use interconnect::Interconnect;
pub use mmu::Mmu;

use cpu::timer::Timer;
use gameboy::GameBoy;

use std::env;
use std::fs;
use std::path::Path;

#[macro_use]
extern crate text_io;

use raylib::prelude::*;

fn main() {
    const SCREEN_WIDTH: u32 = 1024;
    const SCREEN_HEIGHT: u32 = 768;
    const SCALE: i32 = 4;

    //const DEBUG_WIDTH: i32 = 16 * 8 * SCALE;
    //const DEBUG_HEIGHT: i32 = 32 * 8 * SCALE;

    // Command Line Arguments
    let args: Vec<String> = env::args().collect();
    let test_rom = args[1].as_str();
    let boot_rom = "roms/blaargs/boot-rom.gb";

    // Path to rom
    let rom_path: &Path = Path::new(test_rom);

    // Path to boot rom
    let boot_path: &Path = Path::new(boot_rom);

    // Contents of rom
    let rom: Vec<u8> = read_file(rom_path).unwrap();

    // Contents of boot rom
    let boot: Vec<u8> = read_file(boot_path).unwrap();

    // GameBoy
    let mut game_boy: GameBoy = GameBoy::new();

    // Read Rom into memory
    game_boy.interconnect.read_rom(&rom);

    // Read boot rom into memory
    game_boy.interconnect.read_boot(&boot);

    // Put PC at beginning of ROM
    game_boy.cpu.pc = 0x000;

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Debug")
        .build();

    loop {
        // GAME LOOP GOES HERE
        /*if game_boy.cpu.pc > 0x100 {
            // Start draw loop
            while !rl.window_should_close() {
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::WHITE);
                //draw_logo(&mut d, &game_boy.interconnect);
                //window::update_debug_window(&mut d, &game_boy.interconnect);
                debug_window(&mut d, &game_boy.interconnect);

                // Game loop goes here
            }
        }*/
        if !game_boy.cpu.halted {
            game_boy.cpu.execute_instruction(&mut game_boy.interconnect);
            if game_boy.interconnect.read_mem(0xFF02) == 0x81 {
                let c: char = game_boy.interconnect.read_mem(0xFF01) as char;
                print!("{}", c);
                if c == 'd' {
                    while !rl.window_should_close() {
                        let mut d = rl.begin_drawing(&thread);
                        d.clear_background(Color::WHITE);
                        //draw_logo(&mut d, &game_boy.interconnect);
                        //window::update_debug_window(&mut d, &game_boy.interconnect);
                        debug_window(&mut d, &game_boy.interconnect);

                        // Game loop goes here
                    }
                    break;
                }
                game_boy.interconnect.write_mem(0xff02, 0x0);
            }
        } else {
            game_boy.interconnect.emu_cycles(1);

            let IF = game_boy.interconnect.read_mem(0xFF0F);

            if IF != 0 {
                game_boy.cpu.halted = false;
            }
        }
    }
}

fn update() {}

fn read_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    //Reads file contents into vector
    fs::read(path)
}

static SCALE: i32 = 4;
static TILE_COLORS: [Color; 4] = [Color::WHITE, Color::GRAY, Color::DARKGRAY, Color::BLACK];

fn debug_window(canvas: &mut RaylibDrawHandle, interconnect: &Interconnect) {
    let mut x_draw = 0;
    let mut y_draw = 0;
    let mut tile_num = 0;

    let _w: u32 = 16 * 8 * 4;
    let _h: u32 = 32 * 8 * 4;

    canvas.clear_background(Color::WHITE);
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
}

fn display_tile(
    canvas: &mut RaylibDrawHandle,
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
        }*/
        // Get First BYTE
        let second_byte: u8 = interconnect.read_mem(addr);

        // Get Second BYTE
        let first_byte: u8 = interconnect.read_mem(addr + 1);

        // Index for tile color
        let mut color: u8 = 0;

        // Iterate over bits of first and second byte
        for bit in (0..7).rev() {
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

            let w = SCALE as i32;
            let h = SCALE as i32;
            canvas.draw_rectangle(new_x, new_y, w, h,TILE_COLORS[color as usize]);
        }
    }
}

fn draw_logo(d: &mut RaylibDrawHandle, interconnect: &Interconnect) {
    /* let mut test_tile: [u8; 16] = [
        0xF0, 0x00, 0xF0, 0x00, 0xFC, 0x00, 0xFC, 0x00, 0xFC, 0x00, 0xFC, 0x00, 0xF3, 0x00, 0xF3,
        0x00,
    ];*/

    let mut test_tile: [u8; 16] = [
        0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56, 0x38,
        0x7C,
    ];

    const NUM_TILES: usize = 32;
    let mut tiles: [[u8; 8]; NUM_TILES] = [[0; 8]; NUM_TILES];

    // Array to keep tile data for current row
    let mut row_tile: [u8; 8] = [0; 8];

    // White Background
    d.clear_background(Color::WHITE);

    let mut a = 0;
    let mut b = 1;
    for i in 0x8000..0x8020 {
        let first_byte = interconnect.read_mem(b + i);
        let second_byte = interconnect.read_mem(a + i);

        a = a + 1;
        b = b + 1;
        let mut index = 0;
        row_tile = [0; 8];

        let x: usize = (i - 0x8000) as usize;
        // Iterate Over Each Bit
        for j in (0..8).rev() {
            let first_bit = (first_byte >> j) & 1;
            let second_bit = (second_byte >> j) & 1;
            if first_bit == 0 && second_bit == 0 {
                row_tile[index] = 0
            } else if first_bit == 0 && second_bit == 1 {
                row_tile[index] = 1;
            } else if first_bit == 1 && second_bit == 0 {
                row_tile[index] = 2
            } else {
                row_tile[index] = 3;
            }
            index += 1;
        }
        tiles[(i - 0x8000) as usize] = row_tile;

        const PIXEL_W: usize = 10;
        const PIXEL_H: usize = 10;

        let mut x_count: usize = 0;
        for (y, row) in tiles.iter().enumerate() {
            x_count = y % 7;
            for (x, color) in row.iter().enumerate() {
                if y > 15 {
                    match color {
                        0 => d.draw_rectangle(
                            (x * PIXEL_W) as i32,
                            (y * PIXEL_H) as i32,
                            PIXEL_W as i32,
                            PIXEL_H as i32,
                            Color::RED,
                        ),
                        1 => d.draw_rectangle(
                            (x * PIXEL_W) as i32,
                            (y * PIXEL_H) as i32,
                            PIXEL_W as i32,
                            PIXEL_H as i32,
                            Color::GREEN,
                        ),
                        2 => d.draw_rectangle(
                            (x * PIXEL_W) as i32,
                            (y * PIXEL_H) as i32,
                            PIXEL_W as i32,
                            PIXEL_H as i32,
                            Color::BLUE,
                        ),
                        3 => d.draw_rectangle(
                            (x * PIXEL_W) as i32,
                            (y * PIXEL_H) as i32,
                            PIXEL_W as i32,
                            PIXEL_H as i32,
                            Color::PURPLE,
                        ),
                        _ => (),
                    }
                } else {
                    match color {
                        0 => d.draw_rectangle(
                            (x * PIXEL_W) as i32,
                            (y * PIXEL_H) as i32,
                            PIXEL_W as i32,
                            PIXEL_H as i32,
                            Color::WHITE,
                        ),
                        1 => d.draw_rectangle(
                            (x * PIXEL_W) as i32,
                            (y * PIXEL_H) as i32,
                            PIXEL_W as i32,
                            PIXEL_H as i32,
                            Color::LIGHTGRAY,
                        ),
                        2 => d.draw_rectangle(
                            (x * PIXEL_W) as i32,
                            (y * PIXEL_H) as i32,
                            PIXEL_W as i32,
                            PIXEL_H as i32,
                            Color::GRAY,
                        ),
                        3 => d.draw_rectangle(
                            (x * PIXEL_W) as i32,
                            (y * PIXEL_H) as i32,
                            PIXEL_W as i32,
                            PIXEL_H as i32,
                            Color::DARKGRAY,
                        ),
                        _ => (),
                    }
                }

                println!("X: {} Y: {}", x, y);
            }
        }
    }

    /*
    // Loop over tile
    let mut k = 0;
    let mut m = 1;
    for i in 0..8 {
        let first_byte = test_tile[m];
        let second_byte = test_tile[k];

        k = k + 2;
        m = m + 2;
        let mut index = 0;
        row_tile = [0; 8];

        // Iterate Over Each Bit
        for j in (0..8).rev() {
            let first_bit = (first_byte >> j) & 1;
            let second_bit = (second_byte >> j) & 1;
            if first_bit == 0 && second_bit == 0 {
                row_tile[index] = 0
            } else if first_bit == 0 && second_bit == 1 {
                row_tile[index] = 1;
            } else if first_bit == 1 && second_bit == 0 {
                row_tile[index] = 2
            } else {
                row_tile[index] = 3;
            }
            index += 1;
        }
        tiles[i] = row_tile;

        println!(
            "FIRST BYTE: {:X} SECOND BYTE: {:X}",
            first_byte, second_byte
        );
        println!("{:?}", row_tile);
    }

    for (x, row) in tiles.iter().enumerate() {
        for (y, color) in row.iter().enumerate() {
            match color {
                0 => d.draw_rectangle((y * 30) as i32, (x * 30) as i32, 30, 30, Color::WHITE),
                1 => d.draw_rectangle((y * 30) as i32, (x * 30) as i32, 30, 30, Color::LIGHTGRAY),
                2 => d.draw_rectangle((y * 30) as i32, (x * 30) as i32, 30, 30, Color::GRAY),
                3 => d.draw_rectangle((y * 30) as i32, (x * 30) as i32, 30, 30, Color::DARKGRAY),
                _ => (),
            }
        }
    }*/
}
