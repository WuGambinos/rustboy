/*mod cpu;
pub use cpu::*;
mod mmu;
pub use mmu::*;*/
mod cpu;
use cpu::Cpu;

mod mmu;
use mmu::*;

#[macro_use]
extern crate text_io;
use std::fs;
use std::path::Path;

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
