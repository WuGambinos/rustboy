use std::io::Read;

use crate::{mmu, MMU};

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

    fn emulate_cycle(&mut self, mmu: &mut MMU) {
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
                self.inc_8bit('B');
                self.pc += 1;
            }

            //DEC B: Flags Z1H
            0x05 => {
                self.dec_8bit('B');
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
                self.rlca();

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

            //ADD HL, BC NEED TO FIX THSIS
            0x09 => {
                //Clear Sub flag
                self.f.sub_flag = 0;

                //Update Half Carry

                //Update Carry
                self.update_carry_flag_16bit(self.registers.hl(), self.registers.bc());

                //HL = HL + BC
                self.registers
                    .set_hl(self.registers.hl().wrapping_add(self.registers.bc()));

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
                self.inc_8bit('C');
                //Increase Program Counter
                self.pc += 1;
            }

            //DEC C
            0x0D => {
                self.dec_8bit('C');
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
                self.rrca();

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
                self.inc_8bit('D');
                //Increase Program counter
                self.pc += 1;
            }

            //DEC D
            0x15 => {
                self.dec_8bit('D');
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
                self.rla();
                //Clear Zero Flag
                self.f.zero_flag = 0;

                //Clear Sub Flag
                self.f.sub_flag = 0;

                //Clear Half Carry Flag
                self.f.half_carry_flag = 0;

                //Increase Program Counter
                self.pc += 1;
            }
            _ => println!("NOT AN OPCODE"),
        }
    }

    fn fetch(&mut self, mmu: &MMU) {
        self.opcode = mmu.read_mem(self.pc);
    }

    fn get_u16(&mut self, mmu: &MMU) -> u16 {
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

    /*************************************************************************
     * INSTRUCTIONS
     *************************************************************************/

    ///Rotate Left Circular Accumulator
    ///
    /// 7th bit of Accumulator is copied into carry and into the 0th bit of A
    fn rlca(&mut self) {
        let lmb: u8 = self.registers.a & 0x80;

        //Rotate Accumulator to left
        self.registers.a <<= 1;

        //Store previous 7th bit in 0th position
        self.registers.a |= (1 << 0) & lmb;

        //Store original 7th bit in carry
        self.f.carry_flag = lmb;
    }

    ///Rotate Right Circular Accumulator
    ///
    /// 0th Bit of Accumulator is copied into the carry and into 7th bit of Accumulator
    fn rrca(&mut self) {
        let rmb: u8 = self.registers.a & 0x01;

        //Rotate Accumulator to right
        self.registers.a >>= 1;

        //Store previous 0th bit in 7th bit of A
        self.registers.a |= (1 << 7) & rmb;

        //Store original 0th bit in carry
        self.f.carry_flag = rmb;
    }

    /// Rotate Left Accumulator
    ///
    /// 7th bit is moved into carry, and the carry is moved into the 0th bit
    fn rla(&mut self) {
        let lmb: u8 = self.registers.a & 0x80;

        //Rotate Accumulator Left
        self.registers.a <<= 1;

        //Store carry into 0th bit of Accumulator
        self.registers.a |= (1 << 0) & (self.f.carry_flag);

        //Move 7th bit into carry
        self.f.carry_flag = lmb;
    }

    /// Rotate Right Accumulator
    ///
    /// 0th bit of A is moved into the carry, and the carry is moved into the 7th bit of A
    fn rra(&mut self) {
        let rmb: u8 = self.registers.a & 0x01;

        //Rotate Accumulator to right
        self.registers.a >>= 1;

        //Store carry in 7th bit of A
        self.registers.a |= (1 << 7) & self.f.carry_flag;

        //Store original 0th bit in carry
        self.f.carry_flag = rmb;
    }

    fn inc_8bit(&mut self, register: char) {
        match register {
            'A' => {
                //Update Half Carry
                self.update_half_carry_flag_sum_8bit(self.registers.a, 1);

                //A = A + 1
                self.registers.a = self.registers.a.wrapping_add(1);

                //Update Zero Flag
                self.update_zero_flag(self.registers.a);

                //Clear Sub Flag
                self.f.sub_flag = 0;
            }
            'B' => {
                //Update Half Carry
                self.update_half_carry_flag_sum_8bit(self.registers.b, 1);

                //B = B + 1
                self.registers.b = self.registers.b.wrapping_add(1);

                //Update Zero Flag
                self.update_zero_flag(self.registers.b);

                //Clear Sub Flag
                self.f.sub_flag = 0;
            }
            'C' => {
                //Update Half Carry
                self.update_half_carry_flag_sum_8bit(self.registers.c, 1);

                //C = C + 1
                self.registers.c = self.registers.c.wrapping_add(1);

                //Update Zero Flag
                self.update_zero_flag(self.registers.c);

                //Clear Sub Flag
                self.f.sub_flag = 0;
            }
            'D' => {
                //Update Half Carry
                self.update_half_carry_flag_sum_8bit(self.registers.d, 1);

                //D = D + 1
                self.registers.d = self.registers.d.wrapping_add(1);

                //Update Zero Flag
                self.update_zero_flag(self.registers.d);

                //Clear Sub Flag
                self.f.sub_flag = 0;
            }
            'E' => {
                //Update Half Carry
                self.update_half_carry_flag_sum_8bit(self.registers.e, 1);

                //E = E + 1
                self.registers.e = self.registers.e.wrapping_add(1);

                //Update Zero Flag
                self.update_zero_flag(self.registers.e);

                //Clear Sub Flag
                self.f.sub_flag = 0;
            }
            'H' => {
                //Update Half Carry
                self.update_half_carry_flag_sum_8bit(self.registers.h, 1);

                //H = H + 1
                self.registers.h = self.registers.h.wrapping_add(1);

                //Update Zero Flag
                self.update_zero_flag(self.registers.h);

                //Clear Sub Flag
                self.f.sub_flag = 0;
            }
            'L' => {
                //Update Half Carry
                self.update_half_carry_flag_sum_8bit(self.registers.l, 1);

                //L = L + 1
                self.registers.l = self.registers.l.wrapping_add(1);

                //Updafte Zero Flag
                self.update_zero_flag(self.registers.l);

                //Clear Sub Flag
                self.f.sub_flag = 0;
            }
            _ => println!("NOT A REGISTER!"),
        }
    }

    fn dec_8bit(&mut self, register: char) {
        match register {
            'A' => {
                self.update_half_carry_flag_sub_8bit(self.registers.a, 1);
                self.registers.a = self.registers.a.wrapping_sub(1);
                self.update_zero_flag(self.registers.a);
                self.f.sub_flag = 1;
            }
            'B' => {
                self.update_half_carry_flag_sub_8bit(self.registers.b, 1);
                self.registers.b = self.registers.b.wrapping_sub(1);
                self.update_zero_flag(self.registers.b);
                self.f.sub_flag = 1;
            }
            'C' => {
                self.update_half_carry_flag_sub_8bit(self.registers.c, 1);
                self.registers.c = self.registers.c.wrapping_sub(1);
                self.update_zero_flag(self.registers.c);
                self.f.sub_flag = 1;
            }
            'D' => {
                self.update_half_carry_flag_sub_8bit(self.registers.d, 1);
                self.registers.d = self.registers.d.wrapping_sub(1);
                self.update_zero_flag(self.registers.d);
                self.f.sub_flag = 1;
            }
            'E' => {
                self.update_half_carry_flag_sub_8bit(self.registers.e, 1);
                self.registers.e = self.registers.e.wrapping_sub(1);
                self.update_zero_flag(self.registers.e);
                self.f.sub_flag = 1;
            }
            'H' => {
                self.update_half_carry_flag_sub_8bit(self.registers.h, 1);
                self.registers.h = self.registers.h.wrapping_sub(1);
                self.update_zero_flag(self.registers.h);
                self.f.sub_flag = 1;
            }
            'L' => {
                self.update_half_carry_flag_sub_8bit(self.registers.l, 1);
                self.registers.l = self.registers.l.wrapping_sub(1);
                self.update_zero_flag(self.registers.l);
                self.f.sub_flag = 1;
            }
            _ => println!("NOT A REGISTER"),
        }
    }

    fn inc_16bit(&mut self, register: String) {}
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn internal() {
        assert_eq!(4, 4);
    }

    #[test]
    fn ld_bc_u16() {
        let mut cpu = Cpu::new();
        let mut mmu = MMU::new();
        cpu.pc = 0;

        mmu.write_mem(0, 0x01);
        mmu.write_mem(1, 0xFA);
        mmu.write_mem(2, 0xDC);

        /*cpu.memory[0] = 0x01;
        cpu.memory[1] = 0xFA;
        cpu.memory[2] = 0xDC;*/

        //FADC

        cpu.emulate_cycle(&mut mmu);

        assert_eq!(cpu.registers.bc(), 0xDCFA);
    }

    /*************************************************************************
     * 8-bit Arithmetic Tests
     *************************************************************************/

    #[test]
    fn inc_b() {
        let mut cpu = Cpu::new();

        cpu.registers.b = 0x01;

        cpu.inc_8bit('B');

        //cpu.registers.b = cpu.registers.b.wrapping_add(1);

        assert_eq!(cpu.registers.b, 0x02);
    }

    #[test]
    fn inc_c() {
        let mut cpu = Cpu::new();

        cpu.registers.c = 0x01;

        cpu.inc_8bit('C');

        assert_eq!(cpu.registers.c, 0x02);
    }

    #[test]
    fn inc_d() {
        let mut cpu = Cpu::new();

        cpu.registers.d = 0x05;
        cpu.inc_8bit('D');

        assert_eq!(cpu.registers.d, 0x06);
    }

    #[test]
    fn inc_e() {
        let mut cpu = Cpu::new();

        cpu.registers.e = 0x05;
        cpu.inc_8bit('E');

        assert_eq!(cpu.registers.e, 0x06);
    }

    #[test]
    fn inc_h() {
        let mut cpu = Cpu::new();

        cpu.registers.h = 0x05;
        cpu.inc_8bit('H');

        assert_eq!(cpu.registers.h, 0x06);
    }

    #[test]
    fn inc_l() {
        let mut cpu = Cpu::new();

        cpu.registers.l = 0x05;
        cpu.inc_8bit('L');

        assert_eq!(cpu.registers.l, 0x06);
    }

    #[test]
    fn dec_b() {
        let mut cpu = Cpu::new();

        cpu.registers.b = 0x02;
        cpu.dec_8bit('B');

        assert_eq!(cpu.registers.b, 0x01);
    }

    #[test]
    fn dec_c() {
        let mut cpu = Cpu::new();

        cpu.registers.c = 0x05;
        cpu.dec_8bit('C');

        assert_eq!(cpu.registers.c, 0x04);
    }

    #[test]
    fn dec_d() {
        let mut cpu = Cpu::new();

        cpu.registers.d = 0x03;
        cpu.dec_8bit('D');

        assert_eq!(cpu.registers.d, 0x02);
    }

    #[test]
    fn dec_e() {
        let mut cpu = Cpu::new();

        cpu.registers.e = 0x01;
        cpu.dec_8bit('E');

        assert_eq!(cpu.registers.e, 0x00);
        assert_eq!(cpu.f.zero_flag, 1);
    }

    #[test]
    fn dec_h() {
        let mut cpu = Cpu::new();

        cpu.registers.h = 0x00;
        cpu.dec_8bit('H');

        assert_eq!(cpu.registers.h, 0xFF);
    }

    #[test]
    fn dec_l() {
        let mut cpu = Cpu::new();

        cpu.registers.l = 0x05;
        cpu.dec_8bit('L');

        assert_eq!(cpu.registers.l, 0x04);
    }

    #[test]
    fn inc_8bit_overflow() {
        let mut cpu = Cpu::new();

        cpu.registers.b = 0xFF;
        cpu.inc_8bit('B');

        assert_eq!(cpu.registers.l, 0x00);
        assert_eq!(cpu.f.zero_flag, 0x01);
        assert_eq!(cpu.f.sub_flag, 0x00);
        assert_eq!(cpu.f.half_carry_flag, 0x01);
    }

    /*************************************************************************
     * 16-bit Arithmetic Tests
     *************************************************************************/

    fn inc_bc() {
        let mut cpu = Cpu::new();

        cpu.registers.set_bc(0x00FF);
    }

    #[test]
    fn half_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0x09;

        let operand: u8 = 0x0A;

        cpu.f.half_carry_flag = ((cpu.registers.b & 0xF) + (operand & 0xF) > 0xF) as u8;

        //cpu.inc_8bit_register('B');

        assert_eq!(cpu.f.half_carry_flag, 1);
    }

    #[test]
    fn carry() {
        let mut cpu = Cpu::new();

        cpu.registers.set_bc(0xFFFF);
        cpu.registers.set_hl(0x0001);

        cpu.update_carry_flag_16bit(cpu.registers.bc(), cpu.registers.hl());

        //assert_eq!(cpu.f.carry_flag, 1);
        assert_eq!(cpu.registers.bc(), 0xFFFF);
        assert_eq!(cpu.registers.hl(), 0x0001);
    }
}
