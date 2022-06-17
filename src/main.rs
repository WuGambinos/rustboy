pub mod cpu;
pub use cpu::Cpu;

mod gameboy;
pub mod mmu;

pub mod interconnect;

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
    //let test_rom = args[1].as_str();
    let test_rom = "roms/blaargs/cpu_instrs/individual/02-interrupts.gb";
    let test_rom2 = "roms/blaargs/instr_timing/instr_timing.gb";
    let test_rom3 = "roms/mooneye_tests/acceptance/timer/div_write.gb";

    //Path to rom
    let rom_path: &Path = Path::new(test_rom);

    //Contents of rom
    let rom: Vec<u8> = read_file(&rom_path).unwrap();

    //GameBoy
    let mut game_boy: GameBoy = GameBoy::new();

    //Read Rom into memory
    game_boy.interconnect.read_rom(&rom);

    game_boy.cpu.pc = 0x100;

    loop {
        game_boy.cpu.execute_instruction(&mut game_boy.interconnect);
        if game_boy.interconnect.read_mem(0xFF02) == 0x81 {
            let c: char = game_boy.interconnect.read_mem(0xFF01) as char;
            print!("{}", c);
            game_boy.interconnect.write_mem(0xff02, 0x0);
        }

        //("OPCODE: {:#X} CYCLE PASSED: {}", cpu.opcode, cycles_passed);
    }
}

fn read_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    //Reads file contents into vector
    fs::read(path)
}
