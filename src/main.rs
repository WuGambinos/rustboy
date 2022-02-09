mod chip;
pub use chip::*;
use std::fs;
use std::path::Path;

#[macro_use]
extern crate text_io;
fn main() {
    let file_name = "roms/tetris.gb";

    //Path to rom
    let path: &Path = Path::new(file_name);

    //Contents of rom
    let rom: Vec<u8> = read_file(&path).unwrap();

    let game_boy: Cpu = Cpu::new();
}

fn read_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    //Reads file contents into vector
    fs::read(path)
}
