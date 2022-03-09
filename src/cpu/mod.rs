pub mod instructions;
use crate::Mmu;

use self::instructions::*;

///Struct that represents flags of the Gameboy CPU
struct Flags {
    zero_flag: u8,
    sub_flag: u8,
    half_carry_flag: u8,
    carry_flag: u8,
}

impl Flags {
    fn new() -> Self {
        Flags {
            zero_flag: 0,
            sub_flag: 0,
            half_carry_flag: 0,
            carry_flag: 0,
        }
    }
}

///Struct that represents registers for the Gameboy CPU
#[derive(Copy, Clone)]
struct Registers {
    //Accumulator
    a: u8,

    //B Register
    b: u8,

    //C Register
    c: u8,

    //D Register
    d: u8,

    //E Register
    e: u8,

    //H Registe
    h: u8,

    //L Register
    l: u8,
}

impl Registers {
    fn new() -> Self {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
        }
    }

    ///Get register pair BC
    fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    ///Store value in register pair BC
    fn set_bc(&mut self, data: u16) {
        self.b = ((data & 0xFF00) >> 8) as u8;
        self.c = (data & 0x00FF) as u8;
    }

    ///Get register pair DE
    fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    ///Store value in register pair DE
    fn set_de(&mut self, data: u16) {
        self.d = ((data & 0xFF00) >> 8) as u8;
        self.e = (data & 0x00FF) as u8;
    }

    ///Get register pair HL
    fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    ///Store value in register pair HL
    fn set_hl(&mut self, data: u16) {
        self.h = ((data & 0xFF00) >> 8) as u8;
        self.l = (data & 0x00FF) as u8;
    }
}

///Struct that represents the gameboy cpu
pub struct Cpu {
    memory: [u8; 65536],

    ///Flags
    f: Flags,

    //Registers
    registers: Registers,

    ///Stack pointer
    sp: u16,

    ///Program counter
    pc: u16,

    ///Current opcode
    opcode: u8,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    ///Create a new instance of the gameboy cpu
    pub fn new() -> Self {
        Cpu {
            memory: [0; 65536],
            registers: Registers::new(),
            f: Flags::new(),
            sp: 0,
            pc: 0,
            opcode: 0,
        }
    }

    fn emulate_cycle(&mut self, mmu: &mut Mmu) {
        self.fetch(mmu);

        match self.opcode {
            //NOP
            0x00 => self.pc += 1,

            //LD BC, u16
            0x01 => {
                //Grab u16 value
                let data = self.get_u16(mmu);

                //BC = u16
                self.registers.set_bc(data);

                //Increase program counter
                self.pc += 3;
            }

            //LD (BC), A
            0x02 => {
                //self.memory[self.registers.bc() as usize] = self.registers.a;
                mmu.write_mem(self.registers.bc(), self.registers.a);
                self.pc += 1;
            }

            //INC BC
            0x03 => {
                self.registers.set_bc(self.registers.bc().wrapping_add(1));
                self.pc += 1;
            }

            //INC B: Flags:Z0H
            0x04 => {
                inc_8bit(self, 'B');
                self.pc += 1;
            }

            //DEC B: Flags Z1H
            0x05 => {
                dec_8bit(self, 'B');
                self.pc += 1;
            }

            //LD B, u8
            0x06 => {
                //B = u8
                self.registers.b = mmu.read_mem(self.pc + 1);

                //Increase Program Counter
                self.pc += 2;
            }

            //RLCA
            0x07 => {
                rlca(self);

                //Clear Zero Flag
                self.f.zero_flag = 0;

                //Clear Sub Flag
                self.f.sub_flag = 0;

                //Clear Half Carry Flag
                self.f.half_carry_flag = 0;

                //Increase Program Counter
                self.pc += 1;
            }

            //LD (u16), SP
            0x08 => {
                //memory[u16] = SP
                let addr: u16 = ((self.pc + 1) as u16) << 8 | (self.pc + 2) as u16;

                //Lower byte of stack pointer
                let lower_sp: u8 = (self.sp & 0x00FF) as u8;

                //Higher byte of stack pointer
                let upper_sp: u8 = ((self.sp & 0xFF00) >> 8) as u8;

                //Write lower_sp to addr
                mmu.write_mem(addr, lower_sp);

                //Write lower_sp to addr+1
                mmu.write_mem(addr + 1, upper_sp);

                //Increase Program Counter
                self.pc += 3;
            }

            //ADD HL, BC
            0x09 => {
                add_rr_hl(self, "DE");
                //Increase Program Counter
                self.pc += 1;
            }

            //LD A, (BC)
            0x0A => {
                let addr: u16 = self.registers.bc();
                self.registers.a = mmu.read_mem(addr);
                self.pc += 1;
            }

            //DEC BC
            0x0B => {
                //BC = BC - 1
                self.registers.set_bc(self.registers.bc().wrapping_sub(1));

                //Increase Program Counter
                self.pc += 1;
            }

            //INC C
            0x0C => {
                inc_8bit(self, 'C');
                //Increase Program Counter
                self.pc += 1;
            }

            //DEC C
            0x0D => {
                dec_8bit(self, 'C');
                //Increase Program Counter
                self.pc += 1;
            }

            //LD C, u8
            0x0E => {
                //C = u8
                let value: u8 = mmu.read_mem(self.pc + 1);
                self.registers.c = value;

                //Increase Program Counter
                self.pc += 2;
            }

            //RRCA
            0x0F => {
                //Rotate
                rrca(self);

                //Clear Zero Flag
                self.f.zero_flag = 0;

                //Clear Sub Flag
                self.f.sub_flag = 0;

                //Clear Half Carry Flag
                self.f.half_carry_flag = 0;

                //Increase Program Counter
                self.pc += 1;
            }

            //STOP
            0x10 => {}

            // LD DE, u16
            0x11 => {
                //DE = u16
                let u16_value = self.get_u16(mmu);
                self.registers.set_de(u16_value);

                //Increase Program Counter
                self.pc += 3;
            }

            //LD (DE) = A
            0x12 => {
                //memory[DE] = A
                let addr: u16 = self.registers.de();
                mmu.write_mem(addr, self.registers.a);
                self.pc += 1;
            }

            //INC DE
            0x13 => {
                self.registers.set_de(self.registers.de().wrapping_add(1));
                self.pc += 1;
            }

            //INC D
            0x14 => {
                inc_8bit(self, 'D');
                //Increase Program counter
                self.pc += 1;
            }

            //DEC D
            0x15 => {
                dec_8bit(self, 'D');
                //Inrease Program Counter
                self.pc += 1;
            }

            //LD D, u8
            0x16 => {
                //D = u8
                let value: u8 = mmu.read_mem(self.pc + 1);
                self.registers.d = value;

                //Increase Program Counter
                self.pc += 2;
            }

            //RLA
            0x17 => {
                //Rotate
                rla(self);

                //Clear Zero Flag
                self.f.zero_flag = 0;

                //Clear Sub Flag
                self.f.sub_flag = 0;

                //Clear Half Carry Flag
                self.f.half_carry_flag = 0;

                //Increase Program Counter
                self.pc += 1;
            }

            //JR i8
            0x18 => {
                let value = mmu.read_mem(self.pc + 1);
                jr(self, value);
            }

            //ADD HL, DE
            0x19 => {
                add_rr_hl(self, "DE");
                self.pc += 1;
            }

            //LD A, (DE)
            0x1A => {
                self.registers.a = mmu.read_mem(self.registers.de());
                self.pc += 1;
            }

            //DEC DE
            0x1B => {
                dec_16bit(self, "DE");
                self.pc += 1;
            }

            //INC E
            0x1C => {
                inc_8bit(self, 'E');
                self.pc += 1;
            }

            //DEC E
            0x1D => {
                dec_8bit(self, 'E');
                self.pc += 1;
            }

            //LD E, u8
            0x1E => {
                self.registers.e = mmu.read_mem(self.pc + 1);
                self.pc += 2;
            }

            //RRA
            0x1F => {
                rra(self);

                self.f.zero_flag = 0;
                self.f.sub_flag = 0;
                self.f.half_carry_flag = 0;

                self.pc += 1;
            }

            //JR NZ, i8
            0x20 => {}
            _ => println!("NOT AN OPCODE"),
        }
    }

    fn fetch(&mut self, mmu: &Mmu) {
        self.opcode = mmu.read_mem(self.pc);
    }

    fn get_u16(&mut self, mmu: &Mmu) -> u16 {
        /*(self.memory[(self.pc + 1) as usize] as u16) << 8
        | (self.memory[(self.pc + 2) as usize]) as u16*/

        (mmu.read_mem(self.pc + 2) as u16) << 8 | mmu.read_mem(self.pc + 1) as u16
    }

    ///Updates Zero Flag
    ///
    /// Zero flag is set when operation results in 0
    fn update_zero_flag(&mut self, v: u8) {
        if v == 0 {
            self.f.zero_flag = 1;
        } else {
            self.f.zero_flag = 0;
        }
    }

    ///Updates Sub flag
    ///
    ///Sub flag is set if subtraction operation was done
    fn update_sub_flag(&mut self) {}

    ///Updates the half carry flag
    ///
    ///In 8 bit addition, half carry is set when there is a carry from bit 3 to bit 4
    fn update_half_carry_flag_sum_8bit(&mut self, register: u8, operand: u8) {
        self.f.half_carry_flag = ((register & 0xF) + (operand & 0xF) > 0xF) as u8;
    }

    ///Updates the half carry flag
    ///
    /// In 8 bit subtraction, half carry is set when lower byte of minuend is less than lower byte of subtrahend
    fn update_half_carry_flag_sub_8bit(&mut self, register: u8, operand: u8) {
        self.f.half_carry_flag = ((register & 0xF) < (operand & 0xF)) as u8;
    }

    ///Updates the half carry flag
    ///
    /// In 16 bit addition, half carry is set when there is a carry from bit 11 to bit 12
    fn update_half_carry_flag_sum_16bit(&mut self, register: u16, operand: u16) {}

    ///Updates the half carry flag
    ///
    /// In 16 bit subtraction, half carry is set when lower 3 bytes of minuend is less then lower 3 bytes of subtrahend
    fn update_half_carry_flag_sub_16bit(&mut self, register: u16, operand: u16) {
        self.f.half_carry_flag = ((register & 0xFFF) < (operand & 0xFFF)) as u8;
    }

    /// Updates Carry flag
    /// Carry flag is set when operation results in overflow
    fn update_carry_flag(&mut self, register: u8, operand: u8) {
        let mut res: u8 = 0;

        match register.checked_add(operand) {
            Some(_v) => res = 1,
            None => res = 0,
        }

        self.f.carry_flag = res;
    }

    /// Updates Carry Flag
    ///
    /// Carry flag is set when operation results in overflow
    fn update_carry_flag_16bit(&mut self, register: u16, operand: u16) {
        let mut res: u8 = 0;
        match register.checked_add(operand) {
            Some(_v) => res = 1,
            None => res = 0,
        }

        self.f.carry_flag = res;
    }
}

#[cfg(test)]
mod tests;
