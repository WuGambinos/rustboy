mod cpu;
mod gameboy;
mod interconnect;
mod mmu;
mod ppu;
mod window;

pub use cpu::Cpu;
pub use mmu::Mmu;

use cpu::timer::Timer;
use gameboy::GameBoy;

use std::env;
use std::fs;
use std::path::Path;

#[macro_use]
extern crate text_io;
extern crate sdl2;

use raylib::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
//use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

fn main() {
    const SCREEN_WIDTH: u32 = 1024;
    const SCREEN_HEIGHT: u32 = 768;
    const SCALE: i32 = 4;

    const DEBUG_WIDTH: i32 = 16 * 8 * SCALE;
    const DEBUG_HEIGHT: i32 = 32 * 8 * SCALE;

    // Command Line Arguments
    let args: Vec<String> = env::args().collect();
    let test_rom = args[1].as_str();

    // Path to rom
    let rom_path: &Path = Path::new(test_rom);

    // Contents of rom
    let rom: Vec<u8> = read_file(rom_path).unwrap();

    // GameBoy
    let mut game_boy: GameBoy = GameBoy::new();

    // Read Rom into memory
    game_boy.interconnect.read_rom(&rom);

    // Put PC at beginning of ROM
    game_boy.cpu.pc = 0x100;

    //game_boy.interconnect.print_vram();
    /*let (mut rl, thread) = raylib::init()
        .size(DEBUG_WIDTH, DEBUG_HEIGHT)
        .title("Debug")
        .build();
    
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        window::update_debug_window(&mut d, &game_boy.interconnect);
    }*/

    /*loop {
        // GAME LOOP GOES HERE
        if !game_boy.cpu.halted {
            game_boy.cpu.execute_instruction(&mut game_boy.interconnect);
            if game_boy.interconnect.read_mem(0xFF02) == 0x81 {
                let c: char = game_boy.interconnect.read_mem(0xFF01) as char;
                print!("{}", c);
                game_boy.interconnect.write_mem(0xff02, 0x0);
            }
        } else {
            game_boy.interconnect.emu_cycles(1);

            let IF = game_boy.interconnect.read_mem(0xFF0F);

            if IF != 0 {
                game_boy.cpu.halted = false;
            }
        }
    }*/


    /*let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let main_window = video_subsystem
        .window("Rustboy", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let debug_window = video_subsystem
        .window("Debug", (16 * 8 * SCALE) as u32, (32 * 8 * SCALE) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = main_window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut canvas2 = debug_window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap(); */


    /* 'running: loop {
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

        window::update_debug_window(&mut canvas2, &game_boy.interconnect);
        // GAME LOOP GOES HERE
        if !game_boy.cpu.halted {
            game_boy.cpu.execute_instruction(&mut game_boy.interconnect);
            if game_boy.interconnect.read_mem(0xFF02) == 0x81 {
                let c: char = game_boy.interconnect.read_mem(0xFF01) as char;
                print!("{}", c);
                game_boy.interconnect.write_mem(0xff02, 0x0);
            }
        } else {
            game_boy.interconnect.emu_cycles(1);

            let IF = game_boy.interconnect.read_mem(0xFF0F);

            if IF != 0 {
                game_boy.cpu.halted = false;
            }
        }
    } */
}

fn update() {}

fn read_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    //Reads file contents into vector
    fs::read(path)
}
