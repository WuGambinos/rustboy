pub mod cpu;
pub use cpu::Cpu;

pub mod mmu;
pub use mmu::*;

use std::fs;
use std::path::Path;
use crate::cpu::instructions::{cp_r_r, inc_16bit, ld_8bit, ld_a_from_io, or_r_r};

#[macro_use]
extern crate text_io;

fn main() {
    let test_rom = "roms/cpu_instrs/individual/01-special.gb";
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




  /*for i in 0..9 {


        while  cpu.pc != 0xC05A {
            cpu.emulate_cycle(&mut mmu);
            counter += 1;
        }

      cpu.print_state(&mmu);
      println!("COUNTER: {}", counter);
      println!("{}-ith Iteration", i+1);
      println!();

      cpu.emulate_cycle(&mut mmu);

        /*if mmu.read_mem(0xFF02) == 0x81 {
            let c: char = mmu.read_mem(0xFF01) as char;
            print!("{}", c);
            mmu.write_mem(0xff02, 0x0);
        }*/

    }

    cpu.print_state(&mmu);
    println!("COUNTER: {}", counter);
    println!();

    let mut n = 31;
    for _ in 0..n {
        cpu.emulate_cycle(&mut mmu);
        println!();
        cpu.print_state(&mmu);
    }*/


    //n = 250;

    /*for  _ in 0..(n*18) {
        cpu.emulate_cycle(&mut mmu);
        println!();
        cpu.print_state(&mmu);
    }*/

    /*for _ in 0..30 {
        while cpu.pc != 0xC00C {
            cpu.emulate_cycle(&mut mmu);
        }

        //println!();
        //cpu.print_state(&mmu);

        cpu.emulate_cycle(&mut mmu);
    }*/

    /*for _ in 0..17 {
        while cpu.pc != 0xC01C {
            cpu.emulate_cycle(&mut mmu);
        }
        println!();
        cpu.print_state(&mmu);

        cpu.emulate_cycle(&mut mmu);

    }*/


    loop  {
        cpu.emulate_cycle(&mut mmu);
        println!();
        cpu.print_state(&mmu);
    }

    /*while cond {

        while cpu.pc != 0xC24C{
            cpu.emulate_cycle(&mut mmu);
            println!();
            cpu.print_state(&mmu);
            //println!("PC: {:#X}", cpu.pc);

            cond = false;
        }
    }*/


    //mmu.write_mem(0xDFFD, 0xD0);

    /*loop {
        cpu.emulate_cycle(&mut mmu);

        if cpu.pc == 0xCA03 {
            println!();
            cpu.print_state(&mmu);
            break;
        }
    }*/



}

fn read_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    //Reads file contents into vector
    fs::read(path)
}



