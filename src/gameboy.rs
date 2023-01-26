use crate::interconnect::Interconnect;
use crate::window;
use crate::cpu::Cpu;

use anyhow::Error;
use anyhow::Result;

use std::fs;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;


///
/// Struct that represents the gameboy system
///
/// Contains the CPU and Interconnect
pub struct GameBoy {
    pub cpu: Cpu,
    pub interconnect: Interconnect,
}

impl GameBoy {
    /// Create new instance of Gameboy
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            interconnect: Interconnect::new(),
        }
    }

    pub fn start_up(&mut self, game: &str) -> Result<(), Error> {
        let boot_rom = "roms/blaargs/boot-rom.gb";

        // Path to rom
        let rom_path: &Path = Path::new(game);

        // Path to boot rom
        let boot_path: &Path = Path::new(boot_rom);

        // Contents of rom
        let rom: Vec<u8> = read_file(rom_path)?;

        // Contents of boot rom
        let _boot: Vec<u8> = read_file(boot_path)?;

        self.interconnect.read_rom(&rom);

        // Put PC where the game starts
        self.cpu.pc = 0x100;

        let sdl_context = sdl2::init().expect("Failed to start SDL");
        let mut debug = window::init_window(&sdl_context);
        let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump");

        let mut main_window = window::init_main_window(&sdl_context);

        'running: loop {
            self.cpu.run(&mut self.interconnect);
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
            window::debug_window(&mut debug, &self.interconnect);
            window::main_window(&mut main_window, &self.interconnect);
        }

        Ok(())
    }
}

fn read_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    //Reads file contents into vector
    fs::read(path)
}
