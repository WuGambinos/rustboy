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

   /* while cond {
        cpu.execute_instruction(&mut mmu);
        if cpu.pc == 0x0050 {
            cond = false;
        }
    }*/

    //cpu.print_state(&mmu);

    let mut counter = 0;

    for _ in 0..189691 {
        cpu.execute_instruction(&mut mmu);
    }
    mmu.write_mem(0xDFFB, 0xC0);
    mmu.write_mem(0xDFFC, 0xC2);
    cpu.print_state(&mmu);

    println!();
    cpu.execute_instruction(&mut mmu);


    println!();
    cpu.print_state(&mmu);

    for _ in 0..13 {
        cpu.execute_instruction(&mut mmu);
        println!();
        cpu.print_state(&mmu);

    }

      /*loop {
           cpu.execute_instruction(&mut mmu);
           if mmu.read_mem(0xFF02) == 0x81 {
                let c: char = mmu.read_mem(0xFF01) as char;
                print!("{}", c);
                mmu.write_mem(0xff02, 0x0);
            }

            println!();
            cpu.print_state(&mmu);
            counter += 1;
       }

       */
}

fn read_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    //Reads file contents into vector
    fs::read(path)
}



