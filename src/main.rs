pub mod cpu;
pub use cpu::Cpu;

pub mod mmu;
pub use mmu::*;

use std::fs;
use std::path::Path;

#[macro_use]
extern crate text_io;

fn main() {
    let file_name = "roms/tetris.gb";

    //Path to rom
    let rom_path: &Path = Path::new(file_name);

    //Contents of rom
    let rom: Vec<u8> = read_file(&rom_path).unwrap();

    //Cpu
    let mut cpu = Cpu::new();

    //Mapped Memory Unit
    let mut mmu: Mmu = Mmu::new();
}

fn read_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    //Reads file contents into vector
    fs::read(path)
}
