use crate::constants::PC_AFTER_BOOT;
use crate::cpu::Cpu;
use crate::interconnect::cartridge::Cartridge;
use crate::interconnect::cartridge_info::ram_size;
use crate::interconnect::Interconnect;

use anyhow::Error;
use anyhow::Result;

use std::fs;
use std::path::Path;

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

    pub fn boot(&mut self, game: &str, skip_boot: bool) -> Result<(), Error> {
        let boot_rom = "roms/bootix_dmg.bin";
        let game_rom_path: &Path = Path::new(game);
        let boot_rom_path: &Path = Path::new(boot_rom);

        let game_rom: Vec<u8> = read_file(game_rom_path)?;
        let boot_rom: Vec<u8> = read_file(boot_rom_path)?;

        let cart_type = game_rom[0x147];
        let rom_size = game_rom[0x148];
        let ram_s = game_rom[0x149];

        let ram = vec![0x00; ram_size(ram_s) as usize];

        self.interconnect.cartridge = Cartridge::new(&game_rom, &ram);
        println!("CART TYPE: {:#X}", cart_type);
        println!("ROM_SIZE: {:#X}", rom_size);
        println!("RAM_SIZE: {:#X} KiB", ram_size(ram_s));
        println!("CHECKSUM: {}", self.interconnect.cartridge.checksum());

        self.cpu.pc = if skip_boot {
            //self.interconnect.load_game_rom(&game_rom);
            self.interconnect.boot_active = false;

            PC_AFTER_BOOT
        } else {
            self.interconnect.load_game_rom(&game_rom);
            self.interconnect.load_boot_rom(&boot_rom);
            0x0000
        };
        Ok(())
    }
}

fn read_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    fs::read(path)
}
