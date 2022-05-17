pub mod instructions;
use crate::Mmu;

use self::instructions::*;

///Struct that represents flags of the Gameboy CPU

#[derive(Debug)]
pub struct Flags {
    data: u8,
}

impl Flags {
    fn new() -> Self {
        Flags { data: 0x80 }
    }

    fn zero_flag(&self) -> u8 {
        (self.data >> 7) & 1
    }

    fn sub_flag(&self) -> u8 {
        (self.data >> 6) & 1
    }

    fn half_carry_flag(&self) -> u8 {
        (self.data >> 5) & 1
    }

    fn carry_flag(&self) -> u8 {
        (self.data >> 4) & 1
    }

    ///Set Zero Flag
    fn set_zero_flag(&mut self) {
        self.data |= 1 << 7;
    }

    ///Clear Zero Flag
    fn clear_zero_flag(&mut self) {
        self.data &= !(1 << 7);
    }

    ///Set Sub Flag
    fn set_sub_flag(&mut self) {
        self.data |= 1 << 6;
    }

    ///Clear Sub Flag
    fn clear_sub_flag(&mut self) {
        self.data &= !(1 << 6);
    }

    ///Set Half Carry Flag
    fn set_half_carry_flag(&mut self) {
        self.data |= 1 << 5;
    }

    /// Clear Half Carry Flag
    fn clear_half_carry_flag(&mut self) {
        self.data &= !(1 << 5)
    }

    ///Set Carry Flag
    fn set_carry_flag(&mut self) {
        self.data |= 1 << 4;
    }

    ///Clear Carry Flag
    fn clear_carry_flag(&mut self) {
        self.data &= !(1 << 4);
    }

    ///Updates Carry flag
    ///
    /// Carry flag is set when operation results in overflow
    fn update_carry_flag_sum_8bit(&mut self, register: u8, operand: u8) {
        let mut res: u16 = (register as u16) + (operand as u16);

        if res > 0xFF {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }
    }

    fn update_carry_flag_sum_16bit(&mut self, register: u16, operand: u16) {
        let res: u32 = (register as u32) + (operand as u32);

        if res > 0xFFFF {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }
    }

    fn update_carry_flag_sub_8bit(&mut self, register: u8, operand: u8) {
        if register < operand {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }
    }

    fn update_carry_flag_sub_16bit(&mut self, register: u16, operand: u16) {
        if register < operand {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }
    }

    /// Updates the half carry flag when there is an addition
    ///
    ///In 8bit addition, half carry is set when there is a carry from bit 3 to bit
    fn update_half_carry_flag_sum_8bit(&mut self, register: u8, operand: u8) {
        if ((register & 0xF) + (operand & 0xF)) > 0xF {
            self.set_half_carry_flag();
        } else {
            self.clear_half_carry_flag();
        }
    }

    fn update_half_carry_flag_sum_16bit(&mut self, register: u16, operand: u16) {}

    //Updates the half carry flag where there is a subtraction
    fn update_half_carry_flag_sub_8bit(&mut self, register: u8, operand: u8) {
        if (register & 0xF) < (operand & 0xF) {
            self.set_half_carry_flag();
        } else {
            self.clear_half_carry_flag();
        }
    }

    pub fn update_half_carry_flag_sub_16bit(&mut self, register: u16, operand: u16) {
        if (register & 0xFFF) < (operand & 0xFFF) {
            self.set_half_carry_flag();
        } else {
            self.clear_half_carry_flag();
        }
    }

    /// Updates the zero flag
    ///
    /// Zero flag is set when operation results in 0
    fn update_zero_flag(&mut self, v: u8) {
        if v == 0 {
            self.set_zero_flag();
        } else {
            self.clear_zero_flag();
        }
    }
}

///Struct that represents registers for the Gameboy CPU

#[derive(Debug)]
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

    //F Register (FLAGS)
    f: Flags,
}

impl Registers {
    fn new() -> Self {
        Registers {
            a: 0x11,
            b: 0x00,
            c: 0x00,
            d: 0xFF,
            e: 0x56,
            h: 0x00,
            l: 0x00,
            f: Flags::new(),
        }
    }

    ///Get register pair BC
    fn bc(&self) -> u16 {
        //(self.b as u16) << 8 | self.c as u16
        u16::from_be_bytes([self.b, self.c])
    }

    ///Store value in register pair BC
    fn set_bc(&mut self, data: u16) {
        let [b, c] = data.to_be_bytes();
        self.b = b;
        self.c = c;
    }

    ///Get register pair DE
    fn de(&self) -> u16 {
        //(self.d as u16) << 8 | self.e as u16
        u16::from_be_bytes([self.d, self.e])
    }

    ///Store value in register pair DE
    fn set_de(&mut self, data: u16) {
        let [d, e] = data.to_be_bytes();
        self.d = d;
        self.e = e;
    }

    ///Get register pair HL
    fn hl(&self) -> u16 {
        // (self.h as u16) << 8 | self.l as u16
        u16::from_be_bytes([self.h, self.l])
    }

    ///Store value in register pair HL
    fn set_hl(&mut self, data: u16) {
        let [h, l] = data.to_be_bytes();
        self.h = h;
        self.l = l;
    }

    ///Get Register Pair AF
    fn af(&self) -> u16 {
        u16::from_be_bytes([self.a, self.f.data])
    }

    ///Store value in register pair AF
    fn set_af(&mut self, data: u16) {
        let [a, f] = data.to_be_bytes();
        self.a = a;
        self.f.data = f;
    }
}

///Struct that represents the gameboy cpu
#[derive(Debug)]
pub struct Cpu {
    //Registers
    registers: Registers,

    ///Stack pointer
    sp: u16,

    ///Program counter
    pub pc: u16,

    ///Current opcode
    pub(crate) opcode: u8,
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
            registers: Registers::new(),
            sp: 0xFFFE,
            pc: 0,
            opcode: 0,
        }
    }

    pub fn emulate_cycle(&mut self, mmu: &mut Mmu) {
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
                inc_8bit(&mut self.registers.f, &mut self.registers.b);
                self.pc += 1;
            }

            //DEC B: Flags Z1H
            0x05 => {
                dec_8bit(&mut self.registers.f, &mut self.registers.b);
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

                //Increase Program Counter
                self.pc += 1;
            }

            //LD (u16), SP
            0x08 => {
                //memory[u16] = SP
                let addr: u16 =
                    u16::from_be_bytes([mmu.read_mem(self.pc + 1), mmu.read_mem(self.pc + 2)]);

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
                inc_8bit(&mut self.registers.f, &mut self.registers.c);
                //Increase Program Counter
                self.pc += 1;
            }

            //DEC C
            0x0D => {
                dec_8bit(&mut self.registers.f, &mut self.registers.c);
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
                //Increase Program Counter
                self.pc += 1;
            }

            //STOP
            0x10 => {
                std::process::exit(0);
            }

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
                inc_8bit(&mut self.registers.f, &mut self.registers.d);
                //Increase Program counter
                self.pc += 1;
            }

            //DEC D
            0x15 => {
                dec_8bit(&mut self.registers.f, &mut self.registers.d);
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
                inc_8bit(&mut self.registers.f, &mut self.registers.e);
                self.pc += 1;
            }

            //DEC E
            0x1D => {
                dec_8bit(&mut self.registers.f, &mut self.registers.e);
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
                self.pc += 1;
            }

            //JR NZ, i8
            0x20 => {
                let value = mmu.read_mem(self.pc + 1);
                jr_nz(self, value);
            }

            //LD HL, u16
            0x21 => {
                let value = self.get_u16(mmu);
                self.registers.set_hl(value);
                self.pc += 3;
            }

            //LD (HL+), A
            0x22 => {
                mmu.write_mem(self.registers.hl(), self.registers.a);
                self.registers.set_hl(self.registers.hl().wrapping_add(1));
                self.pc += 1;
            }

            //INC HL
            0x23 => {
                inc_16bit(self, "HL");
                self.pc += 1;
            }

            //INC H
            0x24 => {
                inc_8bit(&mut self.registers.f, &mut self.registers.h);
                self.pc += 1;
            }

            //DEC H
            0x25 => {
                dec_8bit(&mut self.registers.f, &mut self.registers.h);
                self.pc += 1;
            }

            //LD H, u8
            0x26 => {
                self.registers.h = mmu.read_mem(self.pc + 1);
                self.pc += 2;
            }

            //DAA MAY need to check
            0x27 => {
                daa(self);
                //Temproary Fix
                if self.registers.a == 0 {
                    self.registers.f.set_zero_flag();
                } else {
                    self.registers.f.clear_carry_flag();
                }
                self.registers.f.clear_half_carry_flag();
            }

            //JR Z, i8
            0x28 => {
                let value = mmu.read_mem(self.pc + 1);
                jr_z(self, value);
            }

            //ADD HL, HL
            0x29 => {
                add_rr_hl(self, "HL");
                self.pc += 1;
            }

            //LD A, (HL+)
            0x2A => {
                self.registers.a = mmu.read_mem(self.registers.hl());
                self.registers.set_hl(self.registers.hl().wrapping_add(1));
                self.pc += 1;
            }

            //DEC HL
            0x2B => {
                dec_16bit(self, "HL");
                self.pc += 1;
            }

            //INC L
            0x2C => {
                inc_8bit(&mut self.registers.f, &mut self.registers.l);
                self.pc += 1;
            }

            //DEC L
            0x2D => {
                dec_8bit(&mut self.registers.f, &mut self.registers.l);
                self.pc += 1;
            }

            //LD L, u8
            0x2E => {
                let value = mmu.read_mem(self.pc + 1);
                self.registers.l = value;
                self.pc += 2;
            }

            //CPL
            0x2F => {
                self.registers.f.set_sub_flag();
                self.registers.f.set_half_carry_flag();
                //A = A xor FF
                self.registers.a ^= 0xFF;
                self.pc += 1;
            }

            //JR NC, i8
            0x30 => {
                let value = mmu.read_mem(self.pc + 1);
                jr_nc(self, value);
            }

            //LD SP, u16
            0x31 => {
                self.sp = self.get_u16(mmu);
                self.pc += 3;
            }

            //LD (HL--), A
            0x32 => {
                //mmu[HL] = A
                mmu.write_mem(self.registers.hl(), self.registers.a);

                //HL--
                self.registers.set_hl(self.registers.hl().wrapping_sub(1));

                self.pc += 1;
            }

            //INC SP
            0x33 => {
                inc_16bit(self, "SP");
                self.pc += 1;
            }

            //INC (HL)
            0x34 => {
                inc_mem(self, mmu);
                self.pc += 1;
            }

            //DEC (HL)
            0x35 => {
                dec_mem(self, mmu);
                self.pc += 1;
            }

            //LD (HL), u8
            0x36 => {
                let value = mmu.read_mem(self.pc + 1);

                //mmu[HL] = u8
                mmu.write_mem(self.registers.hl(), value);

                self.pc += 2;
            }

            //Set Carry Flag(SCF)
            0x37 => {
                self.registers.f.set_carry_flag();
                self.pc += 1;
            }

            //JR C, i8
            0x38 => {
                let value = mmu.read_mem(self.pc + 1);
                jr_c(self, value);
            }

            //ADD HL, SP
            0x39 => {
                add_rr_hl(self, "SP");
                self.pc += 1;
            }

            //LD A, (HL--)
            0x3A => {
                //value = mem[HL]
                let value = mmu.read_mem(self.registers.hl());

                //A = mem[HL]
                self.registers.a = value;

                //HL--
                self.registers.set_hl(self.registers.hl().wrapping_sub(1));

                self.pc += 1;
            }

            //DEC SP
            0x3B => {
                dec_16bit(self, "SP");
                self.pc += 1;
            }

            //INC A
            0x3C => {
                inc_8bit(&mut self.registers.f, &mut self.registers.a);
                self.pc += 1;
            }

            //DEC A
            0x3D => {
                dec_8bit(&mut self.registers.f, &mut self.registers.a);
                self.pc += 1;
            }

            //LD A, u8
            0x3E => {
                self.registers.a = mmu.read_mem(self.pc + 1);
                self.pc += 2;
            }

            //Clear Carry Flag(CCF)
            0x3F => {
                self.registers.f.clear_carry_flag();
                self.pc += 1;
            }

            //LD B, B
            0x40 => {
                self.pc += 1;
            }

            //LD B, C
            0x41 => {
                ld_8bit(&mut self.registers.b, self.registers.c);
                self.pc += 1;
            }

            //LD B, D
            0x42 => {
                ld_8bit(&mut self.registers.b, self.registers.d);
                self.pc += 1;
            }

            //LD B, E
            0x43 => {
                ld_8bit(&mut self.registers.b, self.registers.e);
                self.pc += 1;
            }

            //LD B, H
            0x44 => {
                ld_8bit(&mut self.registers.b, self.registers.h);
                self.pc += 1;
            }

            //LD B, L
            0x45 => {
                ld_8bit(&mut self.registers.b, self.registers.l);
                self.pc += 1;
            }

            //LD B, (HL)
            0x46 => {
                ld_8bit(&mut self.registers.b, mmu.read_mem(self.pc + 1));
                self.pc += 1;
            }

            //LD B, A
            0x47 => {
                ld_8bit(&mut self.registers.b, self.registers.a);
                self.pc += 1;
            }

            //LD C, B
            0x48 => {
                ld_8bit(&mut self.registers.c, self.registers.b);
                self.pc += 1;
            }

            //LD C, C
            0x49 => {
                self.pc += 1;
            }

            //LD C, D
            0x4A => {
                ld_8bit(&mut self.registers.c, self.registers.d);
                self.pc += 1;
            }

            //LD C, E
            0x4B => {
                ld_8bit(&mut self.registers.c, self.registers.e);
                self.pc += 1;
            }

            //LD C, H
            0x4C => {
                ld_8bit(&mut self.registers.c, self.registers.h);
                self.pc += 1;
            }

            //LD C, L
            0x4D => {
                ld_8bit(&mut self.registers.c, self.registers.l);
                self.pc += 1;
            }

            //LD C, (HL)
            0x4E => {
                ld_8bit(&mut self.registers.c, mmu.read_mem(self.pc + 1));
                self.pc += 1;
            }

            //LD C, A
            0x4F => {
                ld_8bit(&mut self.registers.c, self.registers.a);
                self.pc += 1;
            }

            //LD D, B
            0x50 => {
                ld_8bit(&mut self.registers.d, self.registers.b);
                self.pc += 1;
            }

            //LD D, C
            0x51 => {
                ld_8bit(&mut self.registers.d, self.registers.c);
                self.pc += 1;
            }

            //LD D, D
            0x52 => {
                self.pc += 1;
            }

            //LD D, E
            0x53 => {
                ld_8bit(&mut self.registers.d, self.registers.e);
                self.pc += 1;
            }

            //LD D, H
            0x54 => {
                ld_8bit(&mut self.registers.d, self.registers.h);
                self.pc += 1;
            }

            //LD D, L
            0x55 => {
                ld_8bit(&mut self.registers.d, self.registers.l);
                self.pc += 1;
            }

            //LD D, (HL)
            0x56 => {
                ld_8bit(&mut self.registers.d, mmu.read_mem(self.pc + 1));
                self.pc += 1;
            }

            //LD D, A
            0x57 => {
                ld_8bit(&mut self.registers.d, self.registers.a);
                self.pc += 1;
            }

            //LD E, B
            0x58 => {
                ld_8bit(&mut self.registers.e, self.registers.b);
                self.pc += 1;
            }

            //LD E, C
            0x59 => {
                ld_8bit(&mut self.registers.e, self.registers.c);
                self.pc += 1;
            }

            //LD E, D
            0x5A => {
                ld_8bit(&mut self.registers.e, self.registers.d);
                self.pc += 1;
            }

            //LD E, E
            0x5B => {
                self.pc += 1;
            }

            //LD E, H
            0x5C => {
                ld_8bit(&mut self.registers.e, self.registers.h);
                self.pc += 1;
            }

            //LD E, L
            0x5D => {
                ld_8bit(&mut self.registers.e, self.registers.l);
                self.pc += 1;
            }

            //LD E, (HL)
            0x5E => {
                ld_8bit(&mut self.registers.e, mmu.read_mem(self.pc + 1));
                self.pc += 1;
            }

            //LD E, A
            0x5F => {
                ld_8bit(&mut self.registers.e, self.registers.a);
                self.pc += 1;
            }

            //LD H, B
            0x60 => {
                ld_8bit(&mut self.registers.h, self.registers.b);
                self.pc += 1;
            }

            //LD H, C
            0x61 => {
                ld_8bit(&mut self.registers.h, self.registers.c);
                self.pc += 1;
            }

            //LD H, D
            0x62 => {
                ld_8bit(&mut self.registers.h, self.registers.d);
                self.pc += 1;
            }

            //LD H, E
            0x63 => {
                ld_8bit(&mut self.registers.h, self.registers.e);
                self.pc += 1;
            }

            //LD H, H
            0x64 => {
                self.pc += 1;
            }

            //LD H, L
            0x65 => {
                ld_8bit(&mut self.registers.h, self.registers.l);
                self.pc += 1;
            }

            //LD H, (HL)
            0x66 => {
                ld_8bit(&mut self.registers.h, mmu.read_mem(self.pc + 1));
                self.pc += 1;
            }

            //LD H, A
            0x67 => {
                ld_8bit(&mut self.registers.h, self.registers.a);
                self.pc += 1;
            }

            //LD L, B
            0x68 => {
                ld_8bit(&mut self.registers.l, self.registers.b);
                self.pc += 1;
            }

            //LD L, C
            0x69 => {
                ld_8bit(&mut self.registers.l, self.registers.c);
                self.pc += 1;
            }

            //LD L, D
            0x6A => {
                ld_8bit(&mut self.registers.l, self.registers.d);
                self.pc += 1;
            }

            //LD L, E
            0x6B => {
                ld_8bit(&mut self.registers.l, self.registers.e);
                self.pc += 1;
            }

            //LD L, H
            0x6C => {
                ld_8bit(&mut self.registers.l, self.registers.h);
                self.pc += 1;
            }

            //LD L, L
            0x6D => {
                self.pc += 1;
            }

            //LD L, (HL)
            0x6E => {
                ld_8bit(&mut self.registers.l, mmu.read_mem(self.pc + 1));
                self.pc += 1;
            }

            //LD L, A
            0x6F => {
                ld_8bit(&mut self.registers.l, self.registers.a);
                self.pc += 1;
            }

            //LD (HL), B
            0x70 => {
                mmu.write_mem(self.registers.hl(), self.registers.b);
                self.pc += 1;
            }

            //LD (HL), C
            0x71 => {
                mmu.write_mem(self.registers.hl(), self.registers.c);
                self.pc += 1;
            }

            //LD (HL), D
            0x72 => {
                mmu.write_mem(self.registers.hl(), self.registers.d);
                self.pc += 1;
            }

            //LD (HL), E
            0x73 => {
                mmu.write_mem(self.registers.hl(), self.registers.e);
                self.pc += 1;
            }

            //LD (HL), H
            0x74 => {
                mmu.write_mem(self.registers.hl(), self.registers.h);
                self.pc += 1;
            }

            //LD (HL), L
            0x75 => {
                mmu.write_mem(self.registers.hl(), self.registers.l);
                self.pc += 1;
            }

            //HALT (NEED TO FINISH)
            0x76 => {}

            //LD (HL), A
            0x77 => {
                mmu.write_mem(self.registers.hl(), self.registers.a);
                self.pc += 1;
            }

            //LD A, B
            0x78 => {
                ld_8bit(&mut self.registers.a, self.registers.b);
                self.pc += 1;
            }

            //LD A, C
            0x79 => {
                ld_8bit(&mut self.registers.a, self.registers.c);
                self.pc += 1;
            }

            //LD A, D
            0x7A => {
                ld_8bit(&mut self.registers.a, self.registers.d);
                self.pc += 1;
            }

            //LD A, E
            0x7B => {
                ld_8bit(&mut self.registers.a, self.registers.e);
                self.pc += 1;
            }

            //LD A, H
            0x7C => {
                ld_8bit(&mut self.registers.a, self.registers.h);
                self.pc += 1;
            }

            //LD A, L
            0x7D => {
                ld_8bit(&mut self.registers.a, self.registers.l);
                self.pc += 1;
            }

            //LD A, (HL)
            0x7E => {
                let addr = self.registers.hl();
                ld_8bit(&mut self.registers.a, mmu.read_mem(addr));
                self.pc += 1;
            }

            //LD A, A
            0x7F => {
                self.pc += 1;
            }

            //ADD A, B
            0x80 => {
                add_a_r(self, self.registers.b);
                self.pc += 1;
            }

            //ADD A, C
            0x81 => {
                add_a_r(self, self.registers.c);
                self.pc += 1;
            }

            //ADD A, D
            0x82 => {
                add_a_r(self, self.registers.d);
                self.pc += 1;
            }

            //ADD A, E
            0x83 => {
                add_a_r(self, self.registers.e);
                self.pc += 1;
            }

            //ADD A, H
            0x84 => {
                add_a_r(self, self.registers.e);
                self.pc += 1;
            }

            //ADD A, L
            0x85 => {
                add_a_r(self, self.registers.l);
                self.pc += 1;
            }

            //ADD A, (HL)
            0x86 => {
                let addr: u16 = self.registers.hl();
                add_a_r(self, mmu.read_mem(addr));
                self.pc += 1;
            }

            //ADD A, A
            0x87 => {
                add_a_r(self, self.registers.a);
                self.pc += 1;
            }

            //ADC A, B
            0x88 => {
                adc_a_r(self, self.registers.b);
                self.pc += 1;
            }

            //ADC A, C
            0x89 => {
                adc_a_r(self, self.registers.c);
                self.pc += 1;
            }

            //ADC A, D
            0x8A => {
                adc_a_r(self, self.registers.d);
                self.pc += 1;
            }

            //ADC A, E
            0x8B => {
                adc_a_r(self, self.registers.e);
                self.pc += 1;
            }

            //ADC A, H
            0x8C => {
                adc_a_r(self, self.registers.h);
                self.pc += 1;
            }

            //ADC A, L
            0x8D => {
                adc_a_r(self, self.registers.l);
                self.pc += 1;
            }

            //ADC A, (HL)
            0x8E => {
                let addr = self.registers.hl();
                adc_a_r(self, mmu.read_mem(addr));
                self.pc += 1;
            }

            //ADC A, A
            0x8F => {
                adc_a_r(self, self.registers.a);
                self.pc += 1;
            }

            //SUB A, B
            0x90 => {
                sub_r_r(self, self.registers.b);
                self.pc += 1;
            }

            //SUB A, C
            0x91 => {
                sub_r_r(self, self.registers.c);
                self.pc += 1;
            }

            //SUB A, D
            0x92 => {
                sub_r_r(self, self.registers.d);
                self.pc += 1;
            }

            //SUB A, E
            0x93 => {
                sub_r_r(self, self.registers.e);
                self.pc += 1;
            }

            //SUB A, H
            0x94 => {
                sub_r_r(self, self.registers.h);
                self.pc += 1;
            }

            //SUB A, L
            0x95 => {
                sub_r_r(self, self.registers.l);
                self.pc += 1;
            }

            //SUB A, (HL)
            0x96 => {
                let addr: u16 = self.registers.hl();
                sub_r_r(self, mmu.read_mem(addr));
                self.pc += 1;
            }

            //SUB  A, A
            0x97 => {
                sub_r_r(self, self.registers.a);
                self.pc += 1;
            }

            //SBC A, B
            0x98 => {
                sbc_r_r(self, self.registers.b);
                self.pc += 1;
            }

            //SBC A, C
            0x99 => {
                sbc_r_r(self, self.registers.c);
                self.pc += 1;
            }

            //SBC A, D
            0x9A => {
                sbc_r_r(self, self.registers.d);
                self.pc += 1;
            }

            //SBC A, E
            0x9B => {
                sbc_r_r(self, self.registers.e);
                self.pc += 1;
            }

            //SBC A, H
            0x9C => {
                sbc_r_r(self, self.registers.h);
                self.pc += 1;
            }

            //SBC A, L
            0x9D => {
                sbc_r_r(self, self.registers.l);
                self.pc += 1;
            }

            //SBC A, (HL)
            0x9E => {
                let addr: u16 = self.registers.hl();
                sbc_r_r(self, mmu.read_mem(addr));
                self.pc += 1;
            }

            //SBC A, A
            0x9F => {
                sbc_r_r(self, self.registers.a);
                self.pc += 1;
            }

            //AND A, B
            0xA0 => {
                and_r_r(self, self.registers.b);
                self.pc = self.pc.wrapping_add(1);
            }

            //AND A, C
            0xA1 => {
                and_r_r(self, self.registers.c);
                self.pc = self.pc.wrapping_add(1);
            }

            //AND A, D
            0xA2 => {
                and_r_r(self, self.registers.d);
                self.pc = self.pc.wrapping_add(1);
            }

            //AND A, E
            0xA3 => {
                and_r_r(self, self.registers.e);
                self.pc = self.pc.wrapping_add(1);
            }

            //AND A, H
            0xA4 => {
                and_r_r(self, self.registers.h);
                self.pc = self.pc.wrapping_add(1);
            }

            //AND A. L
            0xA5 => {
                and_r_r(self, self.registers.l);
                self.pc = self.pc.wrapping_add(1);
            }

            //AND A, (HL)
            0xA6 => {
                let addr: u16 = self.registers.hl();
                and_r_r(self, mmu.read_mem(addr));
                self.pc = self.pc.wrapping_add(1);
            }

            //AND A, A
            0xA7 => {
                and_r_r(self, self.registers.a);
                self.pc = self.pc.wrapping_add(1);
            }

            //XOR A, B
            0xA8 => {
                xor_r_r(self, self.registers.b);
                self.pc += 1;
            }

            //XOR A, C
            0xA9 => {
                xor_r_r(self, self.registers.c);
                self.pc += 1
            }

            //XOR A, D
            0xAA => {
                xor_r_r(self, self.registers.d);
                self.pc += 1
            }

            //XOR A, E
            0xAB => {
                xor_r_r(self, self.registers.e);
                self.pc += 1
            }

            //XOR A, H
            0xAC => {
                xor_r_r(self, self.registers.h);
                self.pc += 1
            }

            //XOR A, L
            0xAD => {
                xor_r_r(self, self.registers.l);
                self.pc += 1
            }

            //XOR A, (HL)
            0xAE => {
                let addr: u16 = self.registers.hl();
                xor_r_r(self, mmu.read_mem(addr));
                self.pc += 1
            }

            //XOR A, A
            0xAF => {
                xor_r_r(self, self.registers.a);
                self.pc += 1
            }

            //OR A, B
            0xB0 => {
                or_r_r(self, self.registers.b);
                self.pc += 1;
            }

            //OR A, C
            0xB1 => {
                or_r_r(self, self.registers.c);
                self.pc += 1;
            }

            //OR A, D
            0xB2 => {
                or_r_r(self, self.registers.d);
                self.pc += 1;
            }

            //OR A, E
            0xB3 => {
                or_r_r(self, self.registers.e);
                self.pc += 1;
            }

            //OR A, H
            0xB4 => {
                or_r_r(self, self.registers.h);
                self.pc += 1;
            }

            //OR A, L
            0xB5 => {
                or_r_r(self, self.registers.l);
                self.pc += 1;
            }

            //OR A, (HL)
            0xB6 => {
                let addr: u16 = self.registers.hl();
                or_r_r(self, mmu.read_mem(addr));
                self.pc += 1;
            }

            //OR A, A
            0xB7 => {
                or_r_r(self, self.registers.a);
                self.pc += 1;
            }

            //CP A, B
            0xB8 => {
                cp_r_r(self, self.registers.b);
                self.pc += 1;
            }

            //CP A, C
            0xB9 => {
                cp_r_r(self, self.registers.c);
                self.pc += 1;
            }

            //CP A, D
            0xBA => {
                cp_r_r(self, self.registers.d);
                self.pc += 1;
            }

            //CP A, E
            0xBB => {
                cp_r_r(self, self.registers.e);
                self.pc += 1;
            }

            //CP A, H
            0xBC => {
                cp_r_r(self, self.registers.h);
                self.pc += 1;
            }

            //CP A, L
            0xBD => {
                cp_r_r(self, self.registers.l);
                self.pc += 1;
            }

            //CP A, (HL)
            0xBE => {
                let addr: u16 = self.registers.hl();
                cp_r_r(self, mmu.read_mem(addr));
                self.pc += 1;
            }

            //CP A, A
            0xBF => {
                cp_r_r(self, self.registers.a);
                self.pc += 1;
            }

            //RET NZ
            0xC0 => {
                ret_nz(self, mmu);
            }

            //POP BC
            0xC1 => {
                pop_rr(
                    mmu,
                    &mut self.registers.b,
                    &mut self.registers.c,
                    &mut self.sp,
                );
                self.pc += 1;
            }

            //JP NZ, u16
            0xC2 => {
                let nn: u16 =
                    u16::from_be_bytes([mmu.read_mem(self.pc + 2), mmu.read_mem(self.pc + 1)]);
                jp_nz(self, nn);
            }

            //JP u16
            0xC3 => {
                let nn: u16 =
                    u16::from_be_bytes([mmu.read_mem(self.pc + 2), mmu.read_mem(self.pc + 1)]);
                jp(self, nn)
            }

            //CALL NZ, u16
            0xC4 => {
                let nn: u16 =
                    u16::from_be_bytes([mmu.read_mem(self.pc + 2), mmu.read_mem(self.pc + 1)]);
                call_nz(self, mmu, nn);
            }

            //PUSH BC
            0xC5 => {
                push_rr(mmu, self.registers.b, self.registers.c, &mut self.sp);
                self.pc += 1;
            }

            //ADD A, u8
            0xC6 => {
                let addr = self.pc + 1;
                add_a_r(self, mmu.read_mem(addr));
                self.pc += 1;
            }

            //RST 0x00(CAll to n)
            0xC7 => rst(self, mmu, 0x00),
            //RET Z
            0xC8 => {
                ret_z(self, mmu);
            }

            //RET
            0xC9 => {
                ret(self, mmu);
            }

            //JP Z, u16
            0xCA => {
                let nn: u16 =
                    u16::from_be_bytes([mmu.read_mem(self.pc + 2), mmu.read_mem(self.pc + 1)]);
                jp_z(self, nn);
            }

            //PREFIX CB
            0xCB => {
                let addr: u16 = self.pc + 1;

                //Opcode
                let op = mmu.read_mem(addr);
                match op {
                    //RLC B
                    0x00 => {
                        rlc(&mut self.registers.f, &mut self.registers.b);
                        self.pc += 2;
                    }

                    //RLC C
                    0x01 => {
                        rlc(&mut self.registers.f, &mut self.registers.c);
                        self.pc += 2;
                    }

                    //RLC D
                    0x02 => {
                        rlc(&mut self.registers.f, &mut self.registers.d);
                        self.pc += 2;
                    }

                    //RLC E
                    0x03 => {
                        rlc(&mut self.registers.f, &mut self.registers.e);
                        self.pc += 2;
                    }

                    //RLC H
                    0x04 => {
                        rlc(&mut self.registers.f, &mut self.registers.h);
                        self.pc += 2;
                    }

                    //RLC L
                    0x05 => {
                        rlc(&mut self.registers.f, &mut self.registers.l);
                        self.pc += 2;
                    }

                    //RLC (HL)
                    0x06 => {
                        let addr = self.registers.hl();
                        rlc(&mut self.registers.f, &mut mmu.read_mem(addr));
                        self.pc += 2;
                    }

                    //RLC A
                    0x07 => {
                        rlc(&mut self.registers.f, &mut self.registers.a);
                        self.pc += 2;
                    }

                    //RRC B
                    0x08 => {
                        rrc(&mut self.registers.f, &mut self.registers.b);
                        self.pc += 2;
                    }

                    //RRC C
                    0x09 => {
                        rrc(&mut self.registers.f, &mut self.registers.c);
                        self.pc += 2;
                    }

                    //RRC D
                    0x0A => {
                        rrc(&mut self.registers.f, &mut self.registers.d);
                        self.pc += 2;
                    }

                    //RRC E
                    0x0B => {
                        rrc(&mut self.registers.f, &mut self.registers.e);
                        self.pc += 2;
                    }

                    //RRC H
                    0x0C => {
                        rrc(&mut self.registers.f, &mut self.registers.h);
                        self.pc += 2;
                    }

                    //RRC L
                    0x0D => {
                        rrc(&mut self.registers.f, &mut self.registers.l);
                        self.pc += 2;
                    }

                    //RRC (HL)
                    0x0E => {
                        let addr = self.registers.hl();
                        rrc(&mut self.registers.f, &mut mmu.read_mem(addr));
                        self.pc += 2;
                    }

                    //RRC A
                    0x0F => {
                        rrc(&mut self.registers.f, &mut self.registers.a);
                        self.pc += 2;
                    }

                    //RL B
                    0x10 => {
                        rl(&mut self.registers.f, &mut self.registers.b);
                        self.pc += 2;
                    }

                    //RL C
                    0x11 => {
                        rl(&mut self.registers.f, &mut self.registers.c);
                        self.pc += 2;
                    }

                    //RL D
                    0x12 => {
                        rl(&mut self.registers.f, &mut self.registers.d);
                        self.pc += 2;
                    }

                    //RL E
                    0x13 => {
                        rl(&mut self.registers.f, &mut self.registers.e);
                        self.pc += 2;
                    }

                    //RL H
                    0x14 => {
                        rl(&mut self.registers.f, &mut self.registers.h);
                        self.pc += 2;
                    }

                    //RL L
                    0x15 => {
                        rl(&mut self.registers.f, &mut self.registers.l);
                        self.pc += 2;
                    }

                    //RL (HL)
                    0x16 => {
                        let addr = self.registers.hl();
                        rl(&mut self.registers.f, &mut mmu.read_mem(addr));
                        self.pc += 2;
                    }

                    //RL A
                    0x17 => {
                        rl(&mut self.registers.f, &mut self.registers.a);
                        self.pc += 2;
                    }

                    //RR B
                    0x18 => {
                        rr(&mut self.registers.f, &mut self.registers.b);
                        self.pc += 2;
                    }

                    //RR C
                    0x19 => {
                        rr(&mut self.registers.f, &mut self.registers.c);
                        self.pc += 2;
                    }

                    //RR D
                    0x1A => {
                        rr(&mut self.registers.f, &mut self.registers.d);
                        self.pc += 2;
                    }

                    //RR E
                    0x1B => {
                        rr(&mut self.registers.f, &mut self.registers.e);
                        self.pc += 2;
                    }

                    //RR H
                    0x1C => {
                        rr(&mut self.registers.f, &mut self.registers.h);
                        self.pc += 2;
                    }

                    //RR L
                    0x1D => {
                        rr(&mut self.registers.f, &mut self.registers.l);
                        self.pc += 2;
                    }

                    //RR (HL)
                    0x1E => {
                        let addr = self.registers.hl();
                        rr(&mut self.registers.f, &mut mmu.read_mem(addr));
                        self.pc += 2;
                    }

                    //RR A
                    0x1F => {
                        rr(&mut self.registers.f, &mut self.registers.a);
                        self.pc += 2;
                    }

                    //SLA B
                    0x20 => {
                        sla(&mut self.registers.f, &mut self.registers.b);
                        self.pc += 2;
                    }

                    //SLA C
                    0x21 => {
                        sla(&mut self.registers.f, &mut self.registers.c);
                        self.pc += 2;
                    }

                    //SLA D
                    0x22 => {
                        sla(&mut self.registers.f, &mut self.registers.d);
                        self.pc += 2;
                    }

                    //SLA E
                    0x23 => {
                        sla(&mut self.registers.f, &mut self.registers.e);
                        self.pc += 2;
                    }

                    //SLA H
                    0x24 => {
                        sla(&mut self.registers.f, &mut self.registers.h);
                        self.pc += 2;
                    }

                    //SLA L
                    0x25 => {
                        sla(&mut self.registers.f, &mut self.registers.l);
                        self.pc += 2;
                    }

                    //SLA (HL)
                    0x26 => {
                        let addr = self.registers.hl();
                        sla(&mut self.registers.f, &mut mmu.read_mem(addr));
                        self.pc += 2;
                    }

                    //SLA A
                    0x27 => {
                        sla(&mut self.registers.f, &mut self.registers.a);
                        self.pc += 2;
                    }

                    //SRA B
                    0x28 => {
                        sra(&mut self.registers.f, &mut self.registers.b);
                        self.pc += 2;
                    }

                    //SRA C
                    0x29 => {
                        sra(&mut self.registers.f, &mut self.registers.c);
                        self.pc += 2;
                    }

                    //SRA D
                    0x2A => {
                        sra(&mut self.registers.f, &mut self.registers.d);
                        self.pc += 2;
                    }

                    //SRA E
                    0x2B => {
                        sra(&mut self.registers.f, &mut self.registers.e);
                        self.pc += 2;
                    }

                    //SRA H
                    0x2C => {
                        sra(&mut self.registers.f, &mut self.registers.h);
                        self.pc += 2;
                    }

                    //SRA L
                    0x2D => {
                        sra(&mut self.registers.f, &mut self.registers.l);
                        self.pc += 2;
                    }

                    //SRA (HL)
                    0x2E => {
                        let addr = self.registers.hl();
                        sra(&mut self.registers.f, &mut mmu.read_mem(addr));
                        self.pc += 2;
                    }

                    //SRA A
                    0x2F => {
                        sra(&mut self.registers.f, &mut self.registers.a);
                        self.pc += 2;
                    }

                    //SWAP B
                    0x30 => {
                        swap(&mut self.registers.f, &mut self.registers.b);
                        self.pc += 2;
                    }

                    //SWAP C
                    0x31 => {
                        swap(&mut self.registers.f, &mut self.registers.c);
                        self.pc += 2;
                    }

                    //SWAP D
                    0x32 => {
                        swap(&mut self.registers.f, &mut self.registers.d);
                        self.pc += 2;
                    }

                    //SWAP E
                    0x33 => {
                        swap(&mut self.registers.f, &mut self.registers.e);
                        self.pc += 2;
                    }

                    //SWAP H
                    0x34 => {
                        swap(&mut self.registers.f, &mut self.registers.h);
                        self.pc += 2;
                    }

                    //SWAP L
                    0x35 => {
                        swap(&mut self.registers.f, &mut self.registers.l);
                        self.pc += 2;
                    }

                    //SWAP (HL)
                    0x36 => {
                        let addr = self.registers.hl();
                        swap(&mut self.registers.f, &mut mmu.read_mem(addr));
                        self.pc += 2;
                    }

                    //SWAP A
                    0x37 => {
                        swap(&mut self.registers.f, &mut self.registers.a);
                        self.pc += 2;
                    }

                    //SRL B
                    0x38 => {
                        srl(&mut self.registers.f, &mut self.registers.b);
                        self.pc += 2;
                    }

                    //SRL C
                    0x39 => {
                        srl(&mut self.registers.f, &mut self.registers.c);
                        self.pc += 2;
                    }

                    //SRL D
                    0x3A => {
                        srl(&mut self.registers.f, &mut self.registers.d);
                        self.pc += 2;
                    }

                    //SRL E
                    0x3B => {
                        srl(&mut self.registers.f, &mut self.registers.e);
                        self.pc += 2;
                    }

                    //SRL H
                    0x3C => {
                        srl(&mut self.registers.f, &mut self.registers.h);
                        self.pc += 2;
                    }

                    //SRL L
                    0x3D => {
                        srl(&mut self.registers.f, &mut self.registers.l);
                        self.pc += 2;
                    }

                    //SRL (HL)
                    0x3E => {
                        let addr = self.registers.hl();
                        srl(&mut self.registers.f, &mut mmu.read_mem(addr));
                        self.pc += 2;
                    }

                    //SRL A
                    0x3F => {
                        srl(&mut self.registers.f, &mut self.registers.a);
                        self.pc += 2;
                    }

                    //BIT 0, B
                    0x40 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.b, 0);
                        self.pc += 2;
                    }

                    //BIT 0, C
                    0x41 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.c, 0);
                        self.pc += 2;
                    }

                    //BIT 0, D
                    0x42 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.d, 0);
                        self.pc += 2;
                    }

                    //BIT 0, E
                    0x43 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.e, 0);
                        self.pc += 2;
                    }

                    //BIT 0, H
                    0x44 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.h, 0);
                        self.pc += 2;
                    }

                    //BIT 0, L
                    0x45 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.l, 0);
                        self.pc += 2;
                    }

                    //BIT 0, (HL)
                    0x46 => {
                        let addr = self.registers.hl();
                        bit_n_hl(&mut self.registers.f, mmu, addr, 0);
                        self.pc += 2;
                    }

                    //BIT 0, A
                    0x47 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.a, 0);
                        self.pc += 2;
                    }

                    //BIT 1, B
                    0x48 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.b, 1);
                        self.pc += 2;
                    }

                    //BIT 1, C
                    0x49 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.c, 1);
                        self.pc += 2;
                    }

                    //BIT 1, D
                    0x4A => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.d, 1);
                        self.pc += 2;
                    }

                    //BIT 1, E
                    0x4B => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.e, 1);
                        self.pc += 2;
                    }

                    //BIT 1, H
                    0x4C => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.h, 1);
                        self.pc += 2;
                    }

                    //BIT 1, L
                    0x4D => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.l, 1);
                        self.pc += 2;
                    }

                    //BIT 1, (HL)
                    0x4E => {
                        let addr = self.registers.hl();
                        bit_n_hl(&mut self.registers.f, mmu, addr, 1);
                        self.pc += 2;
                    }

                    //BIT 1, A
                    0x4F => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.a, 1);
                        self.pc += 2;
                    }

                    //BIT 2, B
                    0x50 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.b, 2);
                        self.pc += 2;
                    }

                    //BIT 2, C
                    0x51 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.c, 2);
                        self.pc += 2;
                    }

                    //BIT 2, D
                    0x52 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.d, 2);
                        self.pc += 2;
                    }

                    //BIT 2, E
                    0x53 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.e, 2);
                        self.pc += 2;
                    }

                    //BIT 2, H
                    0x54 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.h, 2);
                        self.pc += 2;
                    }

                    //BIT 2, L
                    0x55 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.l, 2);
                        self.pc += 2;
                    }

                    //BIT 2, (HL)
                    0x56 => {
                        let addr = self.registers.hl();
                        bit_n_hl(&mut self.registers.f, mmu, addr, 2);
                        self.pc += 2;
                    }

                    //BIT 2, A
                    0x57 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.a, 2);
                        self.pc += 2;
                    }

                    //BIT 3, B
                    0x58 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.b, 3);
                        self.pc += 2;
                    }

                    //BIT 3, C
                    0x59 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.c, 3);
                        self.pc += 2;
                    }

                    //BIT 3, D
                    0x5A => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.d, 3);
                        self.pc += 2;
                    }

                    //BIT 3, E
                    0x5B => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.e, 3);
                        self.pc += 2;
                    }

                    //BIT 3, H
                    0x5C => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.h, 3);
                        self.pc += 2;
                    }

                    //BIT 3, L
                    0x5D => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.l, 3);
                        self.pc += 2;
                    }

                    //BIT 3, (HL)
                    0x5E => {
                        let addr = self.registers.hl();
                        bit_n_hl(&mut self.registers.f, mmu, addr, 3);
                        self.pc += 2;
                    }

                    //BIT 3, A
                    0x5F => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.a, 3);
                        self.pc += 2;
                    }

                    //BIT 4, B
                    0x60 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.b, 4);
                        self.pc += 2;
                    }

                    //BIT 4, C
                    0x61 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.c, 4);
                        self.pc += 2;
                    }

                    //BIT 4, D
                    0x62 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.d, 4);
                        self.pc += 2;
                    }

                    //BIT 4, E
                    0x63 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.e, 4);
                        self.pc += 2;
                    }

                    //BIT 4, H
                    0x64 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.h, 4);
                        self.pc += 2;
                    }

                    //BIT 4, L
                    0x65 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.l, 4);
                        self.pc += 2;
                    }

                    //BIT 4, (HL)
                    0x66 => {
                        let addr = self.registers.hl();
                        bit_n_hl(&mut self.registers.f, mmu, addr, 4);
                        self.pc += 2;
                    }

                    //BIT 4, A
                    0x67 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.a, 4);
                        self.pc += 2;
                    }

                    //BIT 5, B
                    0x68 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.b, 5);
                        self.pc += 2;
                    }

                    //BIT 5, C
                    0x69 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.c, 5);
                        self.pc += 2;
                    }

                    //BIT 5, D
                    0x6A => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.d, 5);
                        self.pc += 2;
                    }

                    //BIT 5, E
                    0x6B => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.e, 5);
                        self.pc += 2;
                    }

                    //BIT 5, H
                    0x6C => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.h, 5);
                        self.pc += 2;
                    }

                    //BIT 5, L
                    0x6D => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.l, 5);
                        self.pc += 2;
                    }

                    //BIT 5, (HL)
                    0x6E => {
                        let addr = self.registers.hl();
                        bit_n_hl(&mut self.registers.f, mmu, addr, 5);
                        self.pc += 2;
                    }

                    //BIT 5, A
                    0x6F => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.a, 5);
                        self.pc += 2;
                    }

                    //BIT 6, B
                    0x70 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.b, 6);
                        self.pc += 2;
                    }

                    //BIT 6, C
                    0x71 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.c, 6);
                        self.pc += 2;
                    }

                    //BIT 6, D
                    0x72 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.d, 6);
                        self.pc += 2;
                    }

                    //BIT 6, E
                    0x73 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.e, 6);
                        self.pc += 2;
                    }

                    //BIT 6, H
                    0x74 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.h, 6);
                        self.pc += 2;
                    }

                    //BIT 6, L
                    0x75 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.l, 6);
                        self.pc += 2;
                    }

                    //BIT 6, (HL)
                    0x76 => {
                        let addr = self.registers.hl();
                        bit_n_hl(&mut self.registers.f, mmu, addr, 6);
                        self.pc += 2;
                    }

                    //BIT 6, A
                    0x77 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.a, 6);
                        self.pc += 2;
                    }

                    //BIT 7, B
                    0x78 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.b, 7);
                        self.pc += 2;
                    }

                    //BIT 7, C
                    0x79 => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.c, 7);
                        self.pc += 2;
                    }

                    //BIT 7, D
                    0x7A => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.d, 7);
                        self.pc += 2;
                    }

                    //BIT 7, E
                    0x7B => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.e, 7);
                        self.pc += 2;
                    }

                    //BIT 7, H
                    0x7C => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.h, 7);
                        self.pc += 2;
                    }

                    //BIT 7, L
                    0x7D => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.l, 7);
                        self.pc += 2;
                    }

                    //BIT 7, (HL)
                    0x7E => {
                        let addr = self.registers.hl();
                        bit_n_hl(&mut self.registers.f, mmu, addr, 7);
                        self.pc += 2;
                    }

                    //BIT 7, A
                    0x7F => {
                        bit_n_r(&mut self.registers.f, &mut self.registers.a, 7);
                        self.pc += 2;
                    }

                    //RES 0, B
                    0x80 => {
                        res_n_r(&mut self.registers.b, 0);
                        self.pc += 2;
                    }

                    //RES 0, C
                    0x81 => {
                        res_n_r(&mut self.registers.c, 0);
                        self.pc += 2;
                    }

                    //RES 0, D
                    0x82 => {
                        res_n_r(&mut self.registers.d, 0);
                        self.pc += 2;
                    }

                    //RES 0, E
                    0x83 => {
                        res_n_r(&mut self.registers.e, 0);
                        self.pc += 2;
                    }

                    //RES 0, H
                    0x84 => {
                        res_n_r(&mut self.registers.h, 0);
                        self.pc += 2;
                    }

                    //RES 0, L
                    0x85 => {
                        res_n_r(&mut self.registers.l, 0);
                        self.pc += 2;
                    }

                    //RES 0, (HL)
                    0x86 => {
                        let addr = self.registers.hl();
                        res_n_hl(mmu, addr, 0);
                        self.pc += 2;
                    }

                    //RES 0, A
                    0x87 => {
                        res_n_r(&mut self.registers.a, 0);
                        self.pc += 2;
                    }

                    //RES 1, B
                    0x88 => {
                        res_n_r(&mut self.registers.b, 1);
                        self.pc += 2;
                    }

                    //RES 1, C
                    0x89 => {
                        res_n_r(&mut self.registers.c, 1);
                        self.pc += 2;
                    }

                    //RES 1, D
                    0x8A => {
                        res_n_r(&mut self.registers.d, 1);
                        self.pc += 2;
                    }

                    //RES 1, E
                    0x8B => {
                        res_n_r(&mut self.registers.e, 1);
                        self.pc += 2;
                    }

                    //RES 1, H
                    0x8C => {
                        res_n_r(&mut self.registers.h, 1);
                        self.pc += 2;
                    }

                    //RES 1, L
                    0x8D => {
                        res_n_r(&mut self.registers.l, 1);
                        self.pc += 2;
                    }

                    //RES 1, (HL)
                    0x8E => {
                        let addr = self.registers.hl();
                        res_n_hl(mmu, addr, 1);
                        self.pc += 2;
                    }

                    //RES 1, A
                    0x8F => {
                        res_n_r(&mut self.registers.a, 1);
                        self.pc += 2;
                    }

                    //RES 2, B
                    0x90 => {
                        res_n_r(&mut self.registers.b, 2);
                        self.pc += 2;
                    }

                    //RES 2, C
                    0x91 => {
                        res_n_r(&mut self.registers.c, 2);
                        self.pc += 2;
                    }

                    //RES 2, D
                    0x92 => {
                        res_n_r(&mut self.registers.d, 2);
                        self.pc += 2;
                    }

                    //RES 2, E
                    0x93 => {
                        res_n_r(&mut self.registers.e, 2);
                        self.pc += 2;
                    }

                    //RES 2, H
                    0x94 => {
                        res_n_r(&mut self.registers.h, 2);
                        self.pc += 2;
                    }

                    //RES 2, L
                    0x95 => {
                        res_n_r(&mut self.registers.l, 2);
                        self.pc += 2;
                    }

                    //RES 2, (HL)
                    0x96 => {
                        let addr = self.registers.hl();
                        res_n_hl(mmu, addr, 2);
                        self.pc += 2;
                    }

                    //RES 2, A
                    0x97 => {
                        res_n_r(&mut self.registers.a, 2);
                        self.pc += 2;
                    }

                    //RES 3, B
                    0x98 => {
                        res_n_r(&mut self.registers.b, 3);
                        self.pc += 2;
                    }

                    //RES 3, C
                    0x99 => {
                        res_n_r(&mut self.registers.c, 3);
                        self.pc += 2;
                    }

                    //RES 3, D
                    0x9A => {
                        res_n_r(&mut self.registers.d, 3);
                        self.pc += 2;
                    }

                    //RES 3, E
                    0x9B => {
                        res_n_r(&mut self.registers.e, 3);
                        self.pc += 2;
                    }

                    //RES 3, H
                    0x9C => {
                        res_n_r(&mut self.registers.h, 3);
                        self.pc += 2;
                    }

                    //RES 3, L
                    0x9D => {
                        res_n_r(&mut self.registers.l, 3);
                        self.pc += 2;
                    }

                    //RES 3, (HL)
                    0x9E => {
                        let addr = self.registers.hl();
                        res_n_hl(mmu, addr, 3);
                        self.pc += 2;
                    }

                    //RES 3, A
                    0x9F => {
                        res_n_r(&mut self.registers.a, 3);
                        self.pc += 2;
                    }

                    //RES 4, B
                    0xA0 => {
                        res_n_r(&mut self.registers.b, 4);
                        self.pc += 2;
                    }

                    //RES 4, C
                    0xA1 => {
                        res_n_r(&mut self.registers.c, 4);
                        self.pc += 2;
                    }

                    //RES 4, D
                    0xA2 => {
                        res_n_r(&mut self.registers.d, 4);
                        self.pc += 2;
                    }

                    //RES 4, E
                    0xA3 => {
                        res_n_r(&mut self.registers.e, 4);
                        self.pc += 2;
                    }

                    //RES 4, H
                    0xA4 => {
                        res_n_r(&mut self.registers.h, 4);
                        self.pc += 2;
                    }

                    //RES 4, L
                    0xA5 => {
                        res_n_r(&mut self.registers.l, 4);
                        self.pc += 2;
                    }

                    //RES 4, (HL)
                    0xA6 => {
                        let addr = self.registers.hl();
                        res_n_hl(mmu, addr, 4);
                        self.pc += 2;
                    }

                    //RES 4, A
                    0xA7 => {
                        res_n_r(&mut self.registers.a, 4);
                        self.pc += 2;
                    }

                    //RES 5, B
                    0xA8 => {
                        res_n_r(&mut self.registers.b, 5);
                        self.pc += 2;
                    }

                    //RES 5, C
                    0xA9 => {
                        res_n_r(&mut self.registers.c, 5);
                        self.pc += 2;
                    }

                    //RES 5, D
                    0xAA => {
                        res_n_r(&mut self.registers.d, 5);
                        self.pc += 2;
                    }

                    //RES 5, E
                    0xAB => {
                        res_n_r(&mut self.registers.e, 5);
                        self.pc += 2;
                    }

                    //RES 5, H
                    0xAC => {
                        res_n_r(&mut self.registers.h, 5);
                        self.pc += 2;
                    }

                    //RES 5, L
                    0xAD => {
                        res_n_r(&mut self.registers.l, 5);
                        self.pc += 2;
                    }

                    //RES 5, (HL)
                    0xAE => {
                        let addr = self.registers.hl();
                        res_n_hl(mmu, addr, 5);
                        self.pc += 2;
                    }

                    //RES 5, A
                    0xAF => {
                        res_n_r(&mut self.registers.a, 5);
                        self.pc += 2;
                    }

                    //RES 6, B
                    0xB0 => {
                        res_n_r(&mut self.registers.b, 6);
                        self.pc += 2;
                    }

                    //RES 6, C
                    0xB1 => {
                        res_n_r(&mut self.registers.c, 6);
                        self.pc += 2;
                    }

                    //RES 6, D
                    0xB2 => {
                        res_n_r(&mut self.registers.d, 6);
                        self.pc += 2;
                    }

                    //RES 6, E
                    0xB3 => {
                        res_n_r(&mut self.registers.e, 6);
                        self.pc += 2;
                    }

                    //RES 6, H
                    0xB4 => {
                        res_n_r(&mut self.registers.h, 6);
                        self.pc += 2;
                    }

                    //RES 6, L
                    0xB5 => {
                        res_n_r(&mut self.registers.l, 6);
                        self.pc += 2;
                    }

                    //RES 6, (HL)
                    0xB6 => {
                        let addr = self.registers.hl();
                        res_n_hl(mmu, addr, 6);
                        self.pc += 2;
                    }

                    //RES 6, A
                    0xB7 => {
                        res_n_r(&mut self.registers.a, 6);
                        self.pc += 2;
                    }

                    //RES 7, B
                    0xB8 => {
                        res_n_r(&mut self.registers.b, 7);
                        self.pc += 2;
                    }

                    //RES 7, C
                    0xB9 => {
                        res_n_r(&mut self.registers.c, 7);
                        self.pc += 2;
                    }

                    //RES 7, D
                    0xBA => {
                        res_n_r(&mut self.registers.d, 7);
                        self.pc += 2;
                    }

                    //RES 7, E
                    0xBB => {
                        res_n_r(&mut self.registers.e, 7);
                        self.pc += 2;
                    }

                    //RES 7, H
                    0xBC => {
                        res_n_r(&mut self.registers.h, 7);
                        self.pc += 2;
                    }

                    //RES 7, L
                    0xBD => {
                        res_n_r(&mut self.registers.l, 7);
                        self.pc += 2;
                    }

                    //RES 7, (HL)
                    0xBE => {
                        let addr = self.registers.hl();
                        res_n_hl(mmu, addr, 7);
                        self.pc += 2;
                    }

                    //RES 7, A
                    0xBF => {
                        res_n_r(&mut self.registers.a, 7);
                        self.pc += 2;
                    }

                    //SET 0, B
                    0xC0 => {
                        set_n_r(&mut self.registers.b, 0);
                        self.pc += 2;
                    }

                    //SET 0, C
                    0xC1 => {
                        set_n_r(&mut self.registers.c, 0);
                        self.pc += 2;
                    }

                    //SET 0, D
                    0xC2 => {
                        set_n_r(&mut self.registers.d, 0);
                        self.pc += 2;
                    }

                    //SET 0, E
                    0xC3 => {
                        set_n_r(&mut self.registers.e, 0);
                        self.pc += 2;
                    }

                    //SET 0, H
                    0xC4 => {
                        set_n_r(&mut self.registers.h, 0);
                        self.pc += 2;
                    }

                    //SET 0, L
                    0xC5 => {
                        set_n_r(&mut self.registers.l, 0);
                        self.pc += 2;
                    }

                    //SET 0, (HL)
                    0xC6 => {
                        let addr = self.registers.hl();
                        set_n_hl(mmu, addr, 0);
                        self.pc += 2;
                    }

                    //SET 0, A
                    0xC7 => {
                        set_n_r(&mut self.registers.a, 0);
                        self.pc += 2;
                    }

                    //SET 1, B
                    0xC8 => {
                        set_n_r(&mut self.registers.b, 1);
                        self.pc += 2;
                    }

                    //SET 1, C
                    0xC9 => {
                        set_n_r(&mut self.registers.c, 1);
                        self.pc += 2;
                    }

                    //SET 1, D
                    0xCA => {
                        set_n_r(&mut self.registers.d, 1);
                        self.pc += 2;
                    }

                    //SET 1, E
                    0xCB => {
                        set_n_r(&mut self.registers.e, 1);
                        self.pc += 2;
                    }

                    //SET 1, H
                    0xCC => {
                        set_n_r(&mut self.registers.h, 1);
                        self.pc += 2;
                    }

                    //SET 1, L
                    0xCD => {
                        set_n_r(&mut self.registers.l, 1);
                        self.pc += 2;
                    }

                    //SET 1, (HL)
                    0xCE => {
                        let addr = self.registers.hl();
                        set_n_hl(mmu, addr, 1);
                        self.pc += 2;
                    }

                    //SET 1, A
                    0xCF => {
                        set_n_r(&mut self.registers.a, 1);
                        self.pc += 2;
                    }

                    //SET 2, B
                    0xD0 => {
                        set_n_r(&mut self.registers.b, 2);
                        self.pc += 2;
                    }

                    //SET 2, C
                    0xD1 => {
                        set_n_r(&mut self.registers.c, 2);
                        self.pc += 2;
                    }

                    //SET 2, D
                    0xD2 => {
                        set_n_r(&mut self.registers.d, 2);
                        self.pc += 2;
                    }

                    //SET 2, E
                    0xD3 => {
                        set_n_r(&mut self.registers.e, 2);
                        self.pc += 2;
                    }

                    //SET 2, H
                    0xD4 => {
                        set_n_r(&mut self.registers.h, 2);
                        self.pc += 2;
                    }

                    //SET 2, L
                    0xD5 => {
                        set_n_r(&mut self.registers.l, 2);
                        self.pc += 2;
                    }

                    //SET 2, (HL)
                    0xD6 => {
                        let addr = self.registers.hl();
                        set_n_hl(mmu, addr, 2);
                        self.pc += 2;
                    }

                    //SET 2, A
                    0xD7 => {
                        set_n_r(&mut self.registers.a, 2);
                        self.pc += 2;
                    }

                    //SET 3, B
                    0xD8 => {
                        set_n_r(&mut self.registers.b, 3);
                        self.pc += 2;
                    }

                    //SET 3, C
                    0xD9 => {
                        set_n_r(&mut self.registers.c, 3);
                        self.pc += 2;
                    }

                    //SET 3, D
                    0xDA => {
                        set_n_r(&mut self.registers.d, 3);
                        self.pc += 2;
                    }

                    //SET 3, E
                    0xDB => {
                        set_n_r(&mut self.registers.e, 3);
                        self.pc += 2;
                    }

                    //SET 3, H
                    0xDC => {
                        set_n_r(&mut self.registers.h, 3);
                        self.pc += 2;
                    }

                    //SET 3, L
                    0xDD => {
                        set_n_r(&mut self.registers.c, 3);
                        self.pc += 2;
                    }

                    //SET 3, (HL)
                    0xDE => {
                        let addr = self.registers.hl();
                        set_n_hl(mmu, addr, 3);
                        self.pc += 2;
                    }

                    //SET 3, A
                    0xDF => {
                        set_n_r(&mut self.registers.a, 3);
                        self.pc += 2;
                    }

                    //SET 4, B
                    0xE0 => {
                        set_n_r(&mut self.registers.b, 4);
                        self.pc += 2;
                    }

                    //SET 4, C
                    0xE1 => {
                        set_n_r(&mut self.registers.c, 4);
                        self.pc += 2;
                    }

                    //SET 4, D
                    0xE2 => {
                        set_n_r(&mut self.registers.d, 4);
                        self.pc += 2;
                    }

                    //SET 4, E
                    0xE3 => {
                        set_n_r(&mut self.registers.e, 4);
                        self.pc += 2;
                    }

                    //SET 4, H
                    0xE4 => {
                        set_n_r(&mut self.registers.h, 4);
                        self.pc += 2;
                    }

                    //SET 4, L
                    0xE5 => {
                        set_n_r(&mut self.registers.l, 4);
                        self.pc += 2;
                    }

                    //SET 4, (HL)
                    0xE6 => {
                        let addr = self.registers.hl();
                        set_n_hl(mmu, addr, 4);
                        self.pc += 2;
                    }

                    //SET 4, A
                    0xE7 => {
                        set_n_r(&mut self.registers.a, 4);
                        self.pc += 2;
                    }

                    //SET 5, B
                    0xE8 => {
                        set_n_r(&mut self.registers.b, 5);
                        self.pc += 2;
                    }

                    //SET 5, C
                    0xE9 => {
                        set_n_r(&mut self.registers.c, 5);
                        self.pc += 2;
                    }

                    //SET 5, D
                    0xEA => {
                        set_n_r(&mut self.registers.d, 5);
                        self.pc += 2;
                    }

                    //SET 5, E
                    0xEB => {
                        set_n_r(&mut self.registers.e, 5);
                        self.pc += 2;
                    }

                    //SET 5, H
                    0xEC => {
                        set_n_r(&mut self.registers.h, 5);
                        self.pc += 2;
                    }

                    //SET 5, L
                    0xED => {
                        set_n_r(&mut self.registers.l, 5);
                        self.pc += 2;
                    }

                    //SET 5, (HL)
                    0xEE => {
                        let addr = self.registers.hl();
                        set_n_hl(mmu, addr, 5);
                        self.pc += 2;
                    }

                    //SET 5, A
                    0xEF => {
                        set_n_r(&mut self.registers.a, 5);
                        self.pc += 2;
                    }

                    //SET 6, B
                    0xF0 => {
                        set_n_r(&mut self.registers.b, 6);
                        self.pc += 2;
                    }

                    //SET 6, C
                    0xF1 => {
                        set_n_r(&mut self.registers.c, 6);
                        self.pc += 2;
                    }

                    //SET 6, D
                    0xF2 => {
                        set_n_r(&mut self.registers.d, 6);
                        self.pc += 2;
                    }

                    //SET 6, E
                    0xF3 => {
                        set_n_r(&mut self.registers.e, 6);
                        self.pc += 2;
                    }

                    //SET 6, H
                    0xF4 => {
                        set_n_r(&mut self.registers.h, 6);
                        self.pc += 2;
                    }

                    //SET 6, L
                    0xF5 => {
                        set_n_r(&mut self.registers.l, 6);
                        self.pc += 2;
                    }

                    //SET 6, (HL)
                    0xF6 => {
                        let addr = self.registers.hl();
                        set_n_hl(mmu, addr, 6);
                        self.pc += 2;
                    }

                    //SET 6, A
                    0xF7 => {
                        set_n_r(&mut self.registers.a, 6);
                        self.pc += 2;
                    }

                    //SET 7, B
                    0xF8 => {
                        set_n_r(&mut self.registers.b, 7);
                        self.pc += 2;
                    }

                    //SET 7, C
                    0xF9 => {
                        set_n_r(&mut self.registers.c, 7);
                        self.pc += 2;
                    }

                    //SET 7, D
                    0xFA => {
                        set_n_r(&mut self.registers.d, 7);
                        self.pc += 2;
                    }

                    //SET 7, E
                    0xFB => {
                        set_n_r(&mut self.registers.e, 7);
                        self.pc += 2;
                    }

                    //SET 7, H
                    0xFC => {
                        set_n_r(&mut self.registers.h, 7);
                        self.pc += 2;
                    }

                    //SET 7, L
                    0xFD => {
                        set_n_r(&mut self.registers.l, 7);
                        self.pc += 2;
                    }

                    //SET 7, (HL)
                    0xFE => {
                        let addr = self.registers.hl();
                        set_n_hl(mmu, addr, 7);
                        self.pc += 2;
                    }

                    //SET 7, A
                    0xFF => {
                        set_n_r(&mut self.registers.a, 7);
                        self.pc += 2;
                    }
                }
            }

            //CALL Z, u16
            0xCC => {
                let nn: u16 =
                    u16::from_be_bytes([mmu.read_mem(self.pc + 2), mmu.read_mem(self.pc + 1)]);
                call_z(self, mmu, nn);
            }

            //CALL u16
            0xCD => {
                let nn: u16 =
                    u16::from_be_bytes([mmu.read_mem(self.pc + 2), mmu.read_mem(self.pc + 1)]);
                call(self, mmu, nn);
            }

            //ADC A, u8
            0xCE => {
                let operand = mmu.read_mem(self.pc + 1);
                adc_a_r(self, operand);
                self.pc += 2;
            }

            //RST 0x08
            0xCF => rst(self, mmu, 0x08),

            //RET NC
            0xD0 => {
                ret_nc(self, mmu);
            }

            //POP DE
            0xD1 => {
                pop_rr(
                    mmu,
                    &mut self.registers.d,
                    &mut self.registers.e,
                    &mut self.sp,
                );
                self.pc += 1;
            }

            //JP NC, u16
            0xD2 => {
                let nn: u16 =
                    u16::from_be_bytes([mmu.read_mem(self.pc + 2), mmu.read_mem(self.pc + 1)]);
                jp_nc(self, nn);
            }

            //Invalid Opcode
            0xD3 => {}

            //CALL NC, u16
            0xD4 => {
                let nn: u16 =
                    u16::from_be_bytes([mmu.read_mem(self.pc + 2), mmu.read_mem(self.pc + 1)]);
                call_nc(self, mmu, nn);
            }

            //PUSH DE
            0xD5 => {
                push_rr(mmu, self.registers.d, self.registers.e, &mut self.sp);
                self.pc += 1;
            }

            //SUB A, u8
            0xD6 => {
                let operand = mmu.read_mem(self.pc + 1);
                sub_r_r(self, operand);
                self.pc += 2;
            }

            //RST 0x10
            0xD7 => rst(self, mmu, 0x10),

            //RET C
            0xD8 => ret_c(self, mmu),

            //RETI (NEED TO FIX)
            0xD9 => {}

            //JP C, u16
            0xDA => {
                let nn: u16 =
                    u16::from_be_bytes([mmu.read_mem(self.pc + 2), mmu.read_mem(self.pc + 1)]);

                jp_c(self, nn);
            }

            //Invalid Opcode
            0xDB => {}

            //CALL C, u16
            0xDC => {
                let nn: u16 =
                    u16::from_be_bytes([mmu.read_mem(self.pc + 2), mmu.read_mem(self.pc + 1)]);
                call_c(self, mmu, nn);
            }

            //Invalid Opcode
            0xDD => {}

            //SBC A, u8
            0xDE => {
                let operand = mmu.read_mem(self.pc + 1);
                sbc_r_r(self, operand);
                self.pc += 2;
            }

            //RST 0x18
            0xDF => rst(self, mmu, 0x18),

            //LD (0xFF00 + u8), A
            0xE0 => {
                let n: u8 = mmu.read_mem(self.pc + 1);
                ld_io_from_a(self, mmu, n);
                self.pc += 2;
            }

            //POP HL
            0xE1 => {
                pop_rr(
                    mmu,
                    &mut self.registers.h,
                    &mut self.registers.l,
                    &mut self.sp,
                );
                self.pc += 1;
            }

            //LD (0xFF00 + C), A
            0xE2 => {
                ld_io_c_from_a(self, mmu);
                self.pc += 2;
            }

            //Invalid Opcode
            0xE3 => {}

            //Invalid Opcode
            0xE4 => {}

            //PUSH HL
            0xE5 => {
                push_rr(mmu, self.registers.h, self.registers.l, &mut self.sp);
                self.pc += 1;
            }

            //AND A, u8
            0xE6 => {
                let n: u8 = mmu.read_mem(self.pc + 1);
                and_r_r(self, n);
                self.pc += 2;
            }

            //RST 0x20
            0xE7 => rst(self, mmu, 0x20),

            //ADD SP, i8  MAY NEED TO CHECK
            0xE8 => {
                let d: i8 = mmu.read_mem(self.pc + 1) as i8;
                self.sp = self.sp.wrapping_add(d as u16);
                self.pc += 2;
            }

            //JP HL
            0xE9 => {
                jp(self, self.registers.hl());
            }

            //LD (u16), A
            0xEA => {
                let nn: u16 =
                    u16::from_be_bytes([mmu.read_mem(self.pc + 2), mmu.read_mem(self.pc + 1)]);
                mmu.write_mem(nn, self.registers.a);
                self.pc += 3;
            }

            //Invalid Opcode
            0xEB => {}

            //Invalid Opcode
            0xEC => {}

            //Invalid Opcode
            0xED => {}

            //XOR A, u8
            0xEE => {
                let n: u8 = mmu.read_mem(self.pc + 1);
                xor_r_r(self, n);
                self.pc += 2;
            }

            //RST 0x28
            0xEF => {
                rst(self, mmu, 0x28);
            }

            //LD A, (FF00+u8)
            0xF0 => {
                let n: u8 = mmu.read_mem(self.pc + 1);
                ld_a_from_io(self, mmu, n);
                self.pc += 2;
            }

            //POP AF
            0xF1 => {
                pop_rr(
                    mmu,
                    &mut self.registers.a,
                    &mut self.registers.f.data,
                    &mut self.sp,
                );
                self.pc += 1;
            }

            //LD A, (FF00 + C)
            0xF2 => {
                ld_a_from_io_c(self, mmu);
                self.pc += 1;
            }

            //DI
            0xF3 => {
                self.pc += 1;
            }

            //Invalid Opcode
            0xF4 => {}

            //PUSH AF
            0xF5 => {
                push_rr(mmu, self.registers.a, self.registers.f.data, &mut self.sp);
                self.pc += 1;
            }

            //OR A, u8
            0xF6 => {
                let n: u8 = mmu.read_mem(self.pc + 1);
                or_r_r(self, n);
                self.pc += 2;
            }

            //RST 0x30
            0xF7 => rst(self, mmu, 0x30),

            //LD HL, SP+i8
            0xF8 => {}

            //LD SP, HL
            0xF9 => {
                self.sp = self.registers.hl();
                self.pc += 1;
            }

            //LD A, (u16)
            0xFA => {
                let n = mmu.read_mem(self.pc + 1);
                ld_8bit(&mut self.registers.a, n);
                self.pc += 3;
            }

            //EI
            0xFB => {
                self.pc += 1;
            }

            //Invalid Opcode
            0xFC => {}

            //Invalid Opcode
            0xFD => {}

            //CP A, u8
            0xFE => {
                let n: u8 = mmu.read_mem(self.pc + 1);
                cp_r_r(self, n);
                self.pc += 2;
            }

            //RST 0x38
            0xFF => rst(self, mmu, 0x38),
            //_ => println!("NOT AN OPCODE"),
        }
    }

    fn fetch(&mut self, mmu: &Mmu) {
        self.opcode = mmu.read_mem(self.pc);
    }

    fn get_u16(&mut self, mmu: &Mmu) -> u16 {
        // (mmu.read_mem(self.pc + 2) as u16) << 8 | mmu.read_mem(self.pc + 1) as u16
        u16::from_be_bytes([mmu.read_mem(self.pc + 2), mmu.read_mem(self.pc + 1)])
    }

    pub fn print_status(&self, mmu: &Mmu) {
        println!("PC: {:#X}", self.pc);
        println!("SP: {:#X}", self.sp);

        println!("MEM[SP+1]: {:#X}", mmu.read_mem(self.sp.wrapping_add(1)));
        println!("MEM[SP]: {:#X}", mmu.read_mem(self.sp));
        println!("MEM[SP-1]: {:#X}", mmu.read_mem(self.sp.wrapping_sub(1)));
        println!("MEM[SP-2]: {:#X}", mmu.read_mem(self.sp.wrapping_sub(2)));

        println!("MEM[0xDFEA]: {:#X}", mmu.read_mem(0xDFEA));
        println!("MEM[0xDFE9]: {:#X}", mmu.read_mem(0xDFE9));

        let reg = format!(
            "AF: {:#X}, BC: {:#X}, DE:{:#X}, HL: {:#X}",
            self.registers.af(),
            self.registers.bc(),
            self.registers.de(),
            self.registers.hl()
        );

        println!("{}", reg);

        println!("FLAG: {:#X}", self.registers.f.data);

        println!("OPCODE: {:#X}", self.opcode);
    }
}

#[cfg(test)]
mod tests;
