pub mod cpu;
pub use cpu::Cpu;

pub mod mmu;
pub use mmu::*;

use std::fs;
use std::path::Path;

#[macro_use]
extern crate text_io;

fn main() {
    let test_rom = "roms/cpu_instrs/individual/02-interrupts.gb";
    let file_name = "roms/tetris.gb";

    //Path to rom
    let rom_path: &Path = Path::new(test_rom);

    //Contents of rom
    let rom: Vec<u8> = read_file(&rom_path).unwrap();

    //Cpu
    let mut cpu = Cpu::new();

    //Mapped Memory Unit
    let mut mmu: Mmu = Mmu::new();

    //Read Rom into memory
    mmu.read_rom(&rom);

    cpu.pc = 0x100;

    let mut counter = 0;

    let mut cond = true;

    while cond {
        cpu.emulate_cycle(&mut mmu);
        println!();
        cpu.print_state(&mmu);

        if cpu.opcode == 0xF8 {
            cpu.print_state(&mmu);
            cond = false;
        }
    }




  /* loop {
       cpu.emulate_cycle(&mut mmu);
       if mmu.read_mem(0xFF02) == 0x81 {
            let c: char = mmu.read_mem(0xFF01) as char;
            print!("{}", c);
            mmu.write_mem(0xff02, 0x0);
        }
   }*/
}

fn read_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    //Reads file contents into vector
    fs::read(path)
}



