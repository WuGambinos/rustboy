pub mod cpu;
pub use cpu::Cpu;

pub mod mmu;
pub use mmu::*;

use std::fs;
use std::path::Path;

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


    loop {
        cpu.emulate_cycle(&mut mmu);

        if mmu.read_mem(0xFF02) == 0x81 {
            let c: char = mmu.read_mem(0xFF01) as char;
            print!("{}", c);

            mmu.write_mem(0xff02, 0x0);
        }

    }

    //Emulate a cpu cycle
    //loop {
    //cpu.emulate_cycle(&mut mmu);


    /*if cpu.pc == 0xC7B8 {
            println!();
            cpu.print_status(&mmu);
            println!("Counter: {}", counter);
        }*/

    /*for _ in 0..10 {
        while cpu.pc != 0xC7B8 {
            cpu.emulate_cycle(&mut mmu);
            counter += 1;
        };

        cpu.print_status(&mmu);
        println!();

        if mmu.read_mem(0xFF02) == 0x81 {
            let c: char = mmu.read_mem(0xFF01) as char;
            print!("{}", c);
            if c == 'F' {
                break;
            }

            mmu.write_mem(0xff02, 0x0);
        }

        cpu.emulate_cycle(&mut mmu);
    }

    cpu.print_status(&mmu);

    for i in 0..2 {
        while cpu.pc != 0xC0AA {
            cpu.emulate_cycle(&mut mmu);
        }

        println!();
        cpu.print_status(&mut mmu);

        if i != 1 {
            cpu.emulate_cycle(&mut mmu);
        }

    }
    /*for i in 0..1100 {
        cpu.emulate_cycle(&mut mmu);
        println!();
        cpu.print_status(&mmu);
    }*/

    println!();
cpu.print_status(&mmu);

    loop {
        cpu.emulate_cycle(&mut mmu);
        println!();
        cpu.print_status(&mmu);
    }*/

    //}

        /*for _ in 0..16 {
        while cpu.pc != 0x20D {
            cpu.emulate_cycle(&mut mmu);
        }

        //cpu.print_status(&mmu);
        cpu.emulate_cycle(&mut mmu);
        //println!();
    }
    cpu.emulate_cycle(&mut mmu);*/


        //8814
        /*let mut count = 0;

    let end = 180630;
    let old_end = 364250;

    for _ in 0..61 {
        cpu.emulate_cycle(&mut mmu);
        /*cpu.print_status(&mmu);
        println!();*/
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
