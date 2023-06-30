use crate::constants::{
    MAIN_SCREEN_HEIGHT, MAIN_SCREEN_WIDTH, PC_AFTER_BOOT, SCREEN_HEIGHT, SCREEN_WIDTH,
};
use crate::cpu::Cpu;
use crate::frontend;
use crate::interconnect::Interconnect;

use anyhow::Error;
use anyhow::Result;

use std::fs;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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

    pub fn boot(&mut self, game: &str, headless: bool, skip_boot: bool) -> Result<(), Error> {
        let boot_rom = "roms/boot-rom.gb";
        let game_rom_path: &Path = Path::new(game);
        let boot_rom_path: &Path = Path::new(boot_rom);

        let game_rom: Vec<u8> = read_file(game_rom_path)?;
        let boot_rom: Vec<u8> = read_file(boot_rom_path)?;

        // TODO CHANGE LATER
        self.cpu.pc = if skip_boot {
            self.interconnect.load_game_rom(&game_rom);
            self.interconnect.boot_active = false;
            PC_AFTER_BOOT
        } else {
            self.interconnect.load_game_rom(&game_rom);
            self.interconnect.load_boot_rom(&boot_rom);
            0x0000
        };

        if headless {
            loop {
                self.cpu.run(&mut self.interconnect);
            }
        } else {
            let sdl_context = sdl2::init().expect("Failed to start SDL");
            let mut debug_window = frontend::init_window(&sdl_context, SCREEN_WIDTH, SCREEN_HEIGHT);
            let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump");

            let mut main_window =
                frontend::init_window(&sdl_context, MAIN_SCREEN_WIDTH, MAIN_SCREEN_HEIGHT);

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
                frontend::debug_window(&mut debug_window, &self.interconnect);
                frontend::main_window(&mut main_window, &self.interconnect);
            }
        }
        Ok(())
    }
}

fn read_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    fs::read(path)
}
