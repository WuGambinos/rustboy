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
use sdl2::render::WindowCanvas;

use std::env;
use std::fs;
use std::path::Path;

#[macro_use]
extern crate text_io;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

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

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: WindowCanvas = window.into_canvas().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    loop {
        if !game_boy.cpu.halted {
            game_boy.cpu.execute_instruction(&mut game_boy.interconnect);
            if game_boy.interconnect.read_mem(0xFF02) == 0x81 {
                let c: char = game_boy.interconnect.read_mem(0xFF01) as char;
                print!("{}", c);
                if c == 'd' {
                    'running: loop {
                        canvas.clear();
                        for event in event_pump.poll_iter() {
                            match event {
                                Event::Quit { .. }
                                | Event::KeyDown {
                                    keycode: Some(Keycode::Escape),
                                    ..
                                } => break 'running,
                                _ => {}
                            }
                        }
                        debug_window(&mut canvas, &game_boy.interconnect);

                        canvas.present();
                    }
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
static TILE_COLORS: [Color; 4] = [
    Color::RGB(255, 255, 255),
    Color::RGB(169, 169, 169),
    Color::RGB(84, 84, 84),
    Color::RGB(0, 0, 0),
];

fn debug_window(canvas: &mut WindowCanvas, interconnect: &Interconnect) {
    let mut x_draw = 0;
    let mut y_draw = 0;
    let mut tile_num = 0;

    let _w: u32 = 16 * 8 * 4;
    let _h: u32 = 32 * 8 * 4;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
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

            let new_x = (x as i32) + ((7 - bit) * SCALE );
            let new_y = (y as i32) + ((tile_y as i32) / 2 * SCALE );

            let w = SCALE as u32;
            let h = SCALE as u32;
            //canvas.draw_rectangle(new_x, new_y, w, h, TILE_COLORS[color as usize]);
            //canvas.set_draw_color(TILE_COLORS[color as usize]);
            //canvas.fill_rect(sdl2::rect::Rect::new(w, h, new_x, new_y));
            canvas.set_draw_color(TILE_COLORS[color as usize]);
            //canvas.fill_rect(sdl2::rect::Rect::new(10, 10, 400, 400));
            canvas.fill_rect(sdl2::rect::Rect::new(new_x, new_y, w, h));
        }
    }
}
