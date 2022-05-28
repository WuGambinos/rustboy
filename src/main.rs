pub mod cpu;
pub use cpu::Cpu;

mod gameboy;
pub mod mmu;

use gameboy::GameBoy;
pub use mmu::*;

use crate::cpu::timer::Timer;
use std::env;
use std::fs;
use std::path::Path;

#[macro_use]
extern crate text_io;

fn main() {
    //Command Line Arguments
    let args: Vec<String> = env::args().collect();
    let test_rom = args[1].as_str();
    //let file_name = "roms/tetris.gb";

    //Path to rom
    let rom_path: &Path = Path::new(test_rom);

    //Contents of rom
    let rom: Vec<u8> = read_file(&rom_path).unwrap();

    //GameBoy
    let mut game_boy: GameBoy = GameBoy::new();

    game_boy.cpu.pc = 0x100;

    //Read Rom into memory
    game_boy.mmu.read_rom(&rom);

    game_boy.cpu.pc = 0x100;

    loop {
        game_boy.execute_instruction();
        let cycles_passed = (game_boy.cpu.timer.internal_ticks - game_boy.cpu.last_cycle) * 4;
        game_boy.timer.do_cycle(cycles_passed);

        if game_boy.mmu.read_mem(0xFF02) == 0x81 {
            let c: char = game_boy.mmu.read_mem(0xFF01) as char;
            print!("{}", c);
            game_boy.mmu.write_mem(0xff02, 0x0);
        }

        //("OPCODE: {:#X} CYCLE PASSED: {}", cpu.opcode, cycles_passed);
    }
}

fn read_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    //Reads file contents into vector
    fs::read(path)
}
