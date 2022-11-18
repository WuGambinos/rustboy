mod cpu;
mod gameboy;
mod interconnect;
mod mmu;
mod ppu;
mod window;
mod constants;

pub use cpu::Cpu;
use interconnect::Interconnect;
pub use mmu::Mmu;

use cpu::timer::Timer;
use gameboy::GameBoy;
use sdl2::render::WindowCanvas;

use std::env;
use std::fs;
use std::path::Path;

/*#[macro_use]
extern crate text_io;
extern crate sdl2;
*/

use sdl2::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

fn main() {
    // Command Line Arguments
    let args: Vec<String> = env::args().collect();
    //let test_rom = args[1].as_str();
    let test_rom = "roms/blaargs/cpu_instrs/individual/01-special.gb";
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
    //game_boy.interconnect.read_boot(&boot);

    // Put PC at beginning of ROM
    game_boy.cpu.pc = 0x100;


    let sdl_context = sdl2::init().expect("Failed to start SDL");
    let mut canvas = window::init_window(&sdl_context);
    let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump");



    'running: loop {
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
        window::debug_window(&mut canvas, &game_boy.interconnect);
    }
}

fn update() {}

fn read_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    //Reads file contents into vector
    fs::read(path)
}

