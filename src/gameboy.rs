use crate::interconnect::Interconnect;
use crate::{Cpu, Timer};

use crate::Mmu;

use crate::cpu::instructions::*;

pub struct GameBoy {
    pub cpu: Cpu,
    pub mmu: Mmu,
    pub timer: Timer,
    pub interconnect: Interconnect,
}

impl GameBoy {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            mmu: Mmu::new(),
            timer: Timer::new(),
            interconnect: Interconnect::new(),
        }
    }

    pub fn execute_instruction(&mut self) {
        if self.cpu.ime_to_be_enabled {
            self.cpu.ime = true;
        }

        //Handle Interrupts
        self.cpu.handle_interrupt(&mut self.mmu);

        self.cpu.last_cycle = self.cpu.timer.internal_ticks;

        self.cpu.fetch(&&self.mmu);

        match self.cpu.opcode {
            //NOP
            0x00 => {
                self.cpu.pc += 1;
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD BC, u16
            0x01 => {
                //Grab u16 value
                let data = self.cpu.get_u16(&self.mmu);

                //BC = u16
                self.cpu.registers.set_bc(data);

                //Increase program counter
                self.cpu.pc += 3;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(3);
            }

            //LD (BC), A
            0x02 => {
                self.mmu
                    .write_mem(self.cpu.registers.bc(), self.cpu.registers.a);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //INC BC
            0x03 => {
                self.cpu
                    .registers
                    .set_bc(self.cpu.registers.bc().wrapping_add(1));
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //INC B
            0x04 => {
                inc_8bit(&mut self.cpu.registers.f, &mut self.cpu.registers.b);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //DEC B
            0x05 => {
                dec_8bit(&mut self.cpu.registers.f, &mut self.cpu.registers.b);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD B, u8
            0x06 => {
                //B = u8
                self.cpu.registers.b = self.read_mem(self.cpu.pc + 1);

                //Increase Program Counter
                self.cpu.pc += 2;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //RLCA
            0x07 => {
                rlca(&mut self.cpu);

                //Increase Program Counter
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD (u16), SP
            0x08 => {
                //memory[u16] = SP
                let addr: u16 = self.cpu.get_u16(&self.mmu);

                //Lower byte of stack pointer
                let lower_sp: u8 = (self.cpu.sp & 0x00FF) as u8;

                //Upper byte of stack pointer
                let upper_sp: u8 = ((self.cpu.sp & 0xFF00) >> 8) as u8;

                //Write lower_sp to addr
                self.write_mem(addr, lower_sp);

                //Write upper_sp to addr+1
                self.write_mem(addr + 1, upper_sp);

                //Increase Program Counter
                self.cpu.pc += 3;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(5);
            }

            //ADD HL, BC
            0x09 => {
                add_rr_hl(&mut self.cpu, "BC");

                //Increase Program Counter
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //LD A, (BC)
            0x0A => {
                let addr: u16 = self.cpu.registers.bc();
                self.cpu.registers.a = self.read_mem(addr);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //DEC BC
            0x0B => {
                //BC = BC - 1
                self.cpu
                    .registers
                    .set_bc(self.cpu.registers.bc().wrapping_sub(1));

                //Increase Program Counter
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //INC C
            0x0C => {
                inc_8bit(&mut self.cpu.registers.f, &mut self.cpu.registers.c);
                //Increase Program Counter
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //DEC C
            0x0D => {
                dec_8bit(&mut self.cpu.registers.f, &mut self.cpu.registers.c);
                //Increase Program Counter
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD C, u8
            0x0E => {
                //C = u8
                let u8_value: u8 = self.read_mem(self.cpu.pc + 1);
                self.cpu.registers.c = u8_value;

                //Increase Program Counter
                self.cpu.pc += 2;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //RRCA
            0x0F => {
                //Rotate
                rrca(&mut self.cpu);
                //Increase Program Counter
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //STOP
            0x10 => {
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            // LD DE, u16
            0x11 => {
                //DE = u16
                let u16_value = self.cpu.get_u16(&self.mmu);
                self.cpu.registers.set_de(u16_value);

                //Increase Program Counter
                self.cpu.pc += 3;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(3);
            }

            //LD (DE) = A
            0x12 => {
                //memory[DE] = A
                let addr: u16 = self.cpu.registers.de();
                self.write_mem(addr, self.cpu.registers.a);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //INC DE
            0x13 => {
                self.cpu
                    .registers
                    .set_de(self.cpu.registers.de().wrapping_add(1));
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //INC D
            0x14 => {
                inc_8bit(&mut self.cpu.registers.f, &mut self.cpu.registers.d);
                //Increase Program counter
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //DEC D
            0x15 => {
                dec_8bit(&mut self.cpu.registers.f, &mut self.cpu.registers.d);
                //Increase Program Counter
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD D, u8
            0x16 => {
                //D = u8
                let u8_value: u8 = self.read_mem(self.cpu.pc + 1);
                self.cpu.registers.d = u8_value;

                //Increase Program Counter
                self.cpu.pc += 2;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //RLA
            0x17 => {
                //Rotate
                rla(&mut self.cpu);

                //Increase Program Counter
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //JR i8
            0x18 => {
                let u8_value = self.read_mem(self.cpu.pc + 1);
                jr(&mut self.cpu, u8_value);

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(3);
            }

            //ADD HL, DE
            0x19 => {
                add_rr_hl(&mut self.cpu, "DE");
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //LD A, (DE)
            0x1A => {
                self.cpu.registers.a = self.read_mem(self.cpu.registers.de());
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //DEC DE
            0x1B => {
                dec_16bit(&mut self.cpu, "DE");
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //INC E
            0x1C => {
                inc_8bit(&mut self.cpu.registers.f, &mut self.cpu.registers.e);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //DEC E
            0x1D => {
                dec_8bit(&mut self.cpu.registers.f, &mut self.cpu.registers.e);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD E, u8
            0x1E => {
                let u8_value = self.read_mem(self.cpu.pc + 1);
                self.cpu.registers.e = u8_value;
                self.cpu.pc += 2;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //RRA
            0x1F => {
                //Rotate
                rra(&mut self.cpu);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //JR NZ, i8
            0x20 => {
                let u8_value = self.read_mem(self.cpu.pc + 1);
                jr_nz(&mut self.cpu, u8_value);
            }

            //LD HL, u16
            0x21 => {
                let u16_value = self.cpu.get_u16(&self.mmu);
                self.cpu.registers.set_hl(u16_value);
                self.cpu.pc += 3;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(3);
            }

            //LD (HL+), A
            0x22 => {
                //memory[HL] = A
                self.mmu
                    .write_mem(self.cpu.registers.hl(), self.cpu.registers.a);

                //HL++
                self.cpu
                    .registers
                    .set_hl(self.cpu.registers.hl().wrapping_add(1));
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //INC HL
            0x23 => {
                //HL++
                inc_16bit(&mut self.cpu, "HL");
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //INC H
            0x24 => {
                //H++
                inc_8bit(&mut self.cpu.registers.f, &mut self.cpu.registers.h);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //DEC H
            0x25 => {
                //L++
                dec_8bit(&mut self.cpu.registers.f, &mut self.cpu.registers.h);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD H, u8
            0x26 => {
                let u8_value = self.read_mem(self.cpu.pc + 1);

                //H = u8
                self.cpu.registers.h = u8_value;
                self.cpu.pc += 2;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //DAA
            0x27 => {
                daa(&mut self.cpu);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //JR Z, i8
            0x28 => {
                let u8_value = self.read_mem(self.cpu.pc + 1);
                jr_z(&mut self.cpu, u8_value);
            }

            //ADD HL, HL
            0x29 => {
                //HL = HL + HL
                add_rr_hl(&mut self.cpu, "HL");
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //LD A, (HL+)
            0x2A => {
                //A = memory[HL]
                self.cpu.registers.a = self.read_mem(self.cpu.registers.hl());

                //HL++
                self.cpu
                    .registers
                    .set_hl(self.cpu.registers.hl().wrapping_add(1));
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //DEC HL
            0x2B => {
                //HL--
                dec_16bit(&mut self.cpu, "HL");
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //INC L
            0x2C => {
                //L++
                inc_8bit(&mut self.cpu.registers.f, &mut self.cpu.registers.l);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //DEC L
            0x2D => {
                //L--
                dec_8bit(&mut self.cpu.registers.f, &mut self.cpu.registers.l);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD L, u8
            0x2E => {
                let u8_value = self.read_mem(self.cpu.pc + 1);
                //L = u8
                self.cpu.registers.l = u8_value;
                self.cpu.pc += 2;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //CPL
            0x2F => {
                //A = A xor FF
                self.cpu.registers.a ^= 0xFF;
                self.cpu.registers.f.set_sub_flag();
                self.cpu.registers.f.set_half_carry_flag();
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //JR NC, i8
            0x30 => {
                let u8_value = self.read_mem(self.cpu.pc + 1);
                jr_nc(&mut self.cpu, u8_value);
            }

            //LD SP, u16
            0x31 => {
                let u16_value = self.cpu.get_u16(&self.mmu);
                //SP = u16
                self.cpu.sp = u16_value;
                self.cpu.pc += 3;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(3);
            }

            //LD (HL--), A
            0x32 => {
                //memory[HL] = A
                self.mmu
                    .write_mem(self.cpu.registers.hl(), self.cpu.registers.a);

                //HL--
                self.cpu
                    .registers
                    .set_hl(self.cpu.registers.hl().wrapping_sub(1));

                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //INC SP
            0x33 => {
                //SP++
                inc_16bit(&mut self.cpu, "SP");
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //INC (HL)
            0x34 => {
                //memory[HL]++
                inc_mem(&mut self.cpu, &mut self.mmu);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(3);
            }

            //DEC (HL)
            0x35 => {
                //memory[HL]--
                dec_mem(&mut self.cpu, &mut self.mmu);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(3);
            }

            //LD (HL), u8
            0x36 => {
                let u8_value = self.read_mem(self.cpu.pc + 1);
                //memory[HL] = u8
                self.write_mem(self.cpu.registers.hl(), u8_value);

                self.cpu.pc += 2;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(3);
            }

            //Set Carry Flag(SCF)
            0x37 => {
                self.cpu.registers.f.set_carry_flag();
                self.cpu.registers.f.clear_sub_flag();
                self.cpu.registers.f.clear_half_carry_flag();
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //JR C, i8
            0x38 => {
                let u8_value = self.read_mem(self.cpu.pc + 1);
                jr_c(&mut self.cpu, u8_value);
            }

            //ADD HL, SP
            0x39 => {
                //HL = HL + SP
                add_rr_hl(&mut self.cpu, "SP");
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //LD A, (HL--)
            0x3A => {
                //u8 = memory[HL]
                let u8_value = self.read_mem(self.cpu.registers.hl());

                //A = memory[HL]
                self.cpu.registers.a = u8_value;

                //HL--
                self.cpu
                    .registers
                    .set_hl(self.cpu.registers.hl().wrapping_sub(1));

                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //DEC SP
            0x3B => {
                //SP--
                dec_16bit(&mut self.cpu, "SP");
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //INC A
            0x3C => {
                //A++
                inc_8bit(&mut self.cpu.registers.f, &mut self.cpu.registers.a);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //DEC A
            0x3D => {
                //A--
                dec_8bit(&mut self.cpu.registers.f, &mut self.cpu.registers.a);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD A, u8
            0x3E => {
                let u8_value = self.read_mem(self.cpu.pc + 1);
                //A = u8
                self.cpu.registers.a = u8_value;
                self.cpu.pc += 2;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //Carry = Carry xor 1
            0x3F => {
                let c = self.cpu.registers.f.carry_flag() ^ 1;
                if c == 1 {
                    self.cpu.registers.f.set_carry_flag();
                } else {
                    self.cpu.registers.f.clear_carry_flag();
                }
                self.cpu.registers.f.clear_half_carry_flag();
                self.cpu.registers.f.clear_sub_flag();
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD B, B
            0x40 => {
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD B, C
            0x41 => {
                ld_8bit(&mut self.cpu.registers.b, self.cpu.registers.c);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD B, D
            0x42 => {
                ld_8bit(&mut self.cpu.registers.b, self.cpu.registers.d);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD B, E
            0x43 => {
                ld_8bit(&mut self.cpu.registers.b, self.cpu.registers.e);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD B, H
            0x44 => {
                ld_8bit(&mut self.cpu.registers.b, self.cpu.registers.h);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD B, L
            0x45 => {
                ld_8bit(&mut self.cpu.registers.b, self.cpu.registers.l);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD B, (HL)
            0x46 => {
                let addr: u16 = self.cpu.registers.hl();
                let u8_value = self.read_mem(addr);
                ld_8bit(&mut self.cpu.registers.b, u8_value);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //LD B, A
            0x47 => {
                ld_8bit(&mut self.cpu.registers.b, self.cpu.registers.a);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD C, B
            0x48 => {
                ld_8bit(&mut self.cpu.registers.c, self.cpu.registers.b);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD C, C
            0x49 => {
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD C, D
            0x4A => {
                ld_8bit(&mut self.cpu.registers.c, self.cpu.registers.d);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD C, E
            0x4B => {
                ld_8bit(&mut self.cpu.registers.c, self.cpu.registers.e);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD C, H
            0x4C => {
                ld_8bit(&mut self.cpu.registers.c, self.cpu.registers.h);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD C, L
            0x4D => {
                ld_8bit(&mut self.cpu.registers.c, self.cpu.registers.l);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD C, (HL)
            0x4E => {
                let addr: u16 = self.cpu.registers.hl();
                let u8_value = self.read_mem(addr);
                ld_8bit(&mut self.cpu.registers.c, u8_value);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //LD C, A
            0x4F => {
                ld_8bit(&mut self.cpu.registers.c, self.cpu.registers.a);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD D, B
            0x50 => {
                ld_8bit(&mut self.cpu.registers.d, self.cpu.registers.b);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD D, C
            0x51 => {
                ld_8bit(&mut self.cpu.registers.d, self.cpu.registers.c);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD D, D
            0x52 => {
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD D, E
            0x53 => {
                ld_8bit(&mut self.cpu.registers.d, self.cpu.registers.e);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD D, H
            0x54 => {
                ld_8bit(&mut self.cpu.registers.d, self.cpu.registers.h);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD D, L
            0x55 => {
                ld_8bit(&mut self.cpu.registers.d, self.cpu.registers.l);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD D, (HL)
            0x56 => {
                let addr: u16 = self.cpu.registers.hl();
                let u8_value = self.read_mem(addr);
                ld_8bit(&mut self.cpu.registers.d, u8_value);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //LD D, A
            0x57 => {
                ld_8bit(&mut self.cpu.registers.d, self.cpu.registers.a);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD E, B
            0x58 => {
                ld_8bit(&mut self.cpu.registers.e, self.cpu.registers.b);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD E, C
            0x59 => {
                ld_8bit(&mut self.cpu.registers.e, self.cpu.registers.c);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD E, D
            0x5A => {
                ld_8bit(&mut self.cpu.registers.e, self.cpu.registers.d);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD E, E
            0x5B => {
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD E, H
            0x5C => {
                ld_8bit(&mut self.cpu.registers.e, self.cpu.registers.h);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD E, L
            0x5D => {
                ld_8bit(&mut self.cpu.registers.e, self.cpu.registers.l);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD E, (HL)
            0x5E => {
                let addr: u16 = self.cpu.registers.hl();
                let u8_value = self.read_mem(addr);
                ld_8bit(&mut self.cpu.registers.e, u8_value);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //LD E, A
            0x5F => {
                ld_8bit(&mut self.cpu.registers.e, self.cpu.registers.a);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD H, B
            0x60 => {
                ld_8bit(&mut self.cpu.registers.h, self.cpu.registers.b);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD H, C
            0x61 => {
                ld_8bit(&mut self.cpu.registers.h, self.cpu.registers.c);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD H, D
            0x62 => {
                ld_8bit(&mut self.cpu.registers.h, self.cpu.registers.d);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD H, E
            0x63 => {
                ld_8bit(&mut self.cpu.registers.h, self.cpu.registers.e);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD H, H
            0x64 => {
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD H, L
            0x65 => {
                ld_8bit(&mut self.cpu.registers.h, self.cpu.registers.l);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD H, (HL)
            0x66 => {
                let addr: u16 = self.cpu.registers.hl();
                let u8_value = self.read_mem(addr);
                ld_8bit(&mut self.cpu.registers.h, u8_value);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //LD H, A
            0x67 => {
                ld_8bit(&mut self.cpu.registers.h, self.cpu.registers.a);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD L, B
            0x68 => {
                ld_8bit(&mut self.cpu.registers.l, self.cpu.registers.b);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD L, C
            0x69 => {
                ld_8bit(&mut self.cpu.registers.l, self.cpu.registers.c);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD L, D
            0x6A => {
                ld_8bit(&mut self.cpu.registers.l, self.cpu.registers.d);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD L, E
            0x6B => {
                ld_8bit(&mut self.cpu.registers.l, self.cpu.registers.e);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD L, H
            0x6C => {
                ld_8bit(&mut self.cpu.registers.l, self.cpu.registers.h);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD L, L
            0x6D => {
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD L, (HL)
            0x6E => {
                let addr: u16 = self.cpu.registers.hl();
                let u8_value = self.read_mem(addr);
                ld_8bit(&mut self.cpu.registers.l, u8_value);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //LD L, A
            0x6F => {
                ld_8bit(&mut self.cpu.registers.l, self.cpu.registers.a);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD (HL), B
            0x70 => {
                self.mmu
                    .write_mem(self.cpu.registers.hl(), self.cpu.registers.b);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //LD (HL), C
            0x71 => {
                self.mmu
                    .write_mem(self.cpu.registers.hl(), self.cpu.registers.c);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //LD (HL), D
            0x72 => {
                self.mmu
                    .write_mem(self.cpu.registers.hl(), self.cpu.registers.d);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //LD (HL), E
            0x73 => {
                self.mmu
                    .write_mem(self.cpu.registers.hl(), self.cpu.registers.e);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //LD (HL), H
            0x74 => {
                self.mmu
                    .write_mem(self.cpu.registers.hl(), self.cpu.registers.h);
                self.cpu.pc += 1;

                //Increase Timer&mut
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //LD (HL), L
            0x75 => {
                self.mmu
                    .write_mem(self.cpu.registers.hl(), self.cpu.registers.l);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //HALT (NEED TO FINISH)
            0x76 => {
                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD (HL), A
            0x77 => {
                self.mmu
                    .write_mem(self.cpu.registers.hl(), self.cpu.registers.a);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //LD A, B
            0x78 => {
                ld_8bit(&mut self.cpu.registers.a, self.cpu.registers.b);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD A, C
            0x79 => {
                ld_8bit(&mut self.cpu.registers.a, self.cpu.registers.c);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD A, D
            0x7A => {
                ld_8bit(&mut self.cpu.registers.a, self.cpu.registers.d);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD A, E
            0x7B => {
                ld_8bit(&mut self.cpu.registers.a, self.cpu.registers.e);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD A, H
            0x7C => {
                ld_8bit(&mut self.cpu.registers.a, self.cpu.registers.h);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD A, L
            0x7D => {
                ld_8bit(&mut self.cpu.registers.a, self.cpu.registers.l);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD A, (HL)
            0x7E => {
                let addr: u16 = self.cpu.registers.hl();
                let u8_value = self.read_mem(addr);
                ld_8bit(&mut self.cpu.registers.a, u8_value);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //LD A, A
            0x7F => {
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //ADD A, B
            0x80 => {
                let reg: u8 = self.cpu.registers.b;
                add_a_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //ADD A, C
            0x81 => {
                let reg: u8 = self.cpu.registers.c;
                add_a_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //ADD A, D
            0x82 => {
                let reg: u8 = self.cpu.registers.d;
                add_a_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //ADD A, E
            0x83 => {
                let reg: u8 = self.cpu.registers.e;
                add_a_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //ADD A, H
            0x84 => {
                let reg: u8 = self.cpu.registers.h;
                add_a_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //ADD A, L
            0x85 => {
                let reg: u8 = self.cpu.registers.l;
                add_a_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //ADD A, (HL)
            0x86 => {
                let addr: u16 = self.cpu.registers.hl();
                let u8_value = self.read_mem(addr);
                add_a_r(&mut self.cpu, u8_value);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //ADD A, A
            0x87 => {
                let reg: u8 = self.cpu.registers.a;
                add_a_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //ADC A, B
            0x88 => {
                let reg: u8 = self.cpu.registers.b;
                adc_a_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //ADC A, C
            0x89 => {
                let reg: u8 = self.cpu.registers.c;
                adc_a_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //ADC A, D
            0x8A => {
                let reg: u8 = self.cpu.registers.d;
                adc_a_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //ADC A, E
            0x8B => {
                let reg: u8 = self.cpu.registers.e;
                adc_a_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //ADC A, H
            0x8C => {
                let reg: u8 = self.cpu.registers.h;
                adc_a_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //ADC A, L
            0x8D => {
                let reg: u8 = self.cpu.registers.l;
                adc_a_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //ADC A, (HL)
            0x8E => {
                let addr: u16 = self.cpu.registers.hl();
                let u8_value = self.read_mem(addr);
                adc_a_r(&mut self.cpu, u8_value);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //ADC A, A
            0x8F => {
                let reg: u8 = self.cpu.registers.a;
                adc_a_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //SUB A, B
            0x90 => {
                let reg: u8 = self.cpu.registers.b;
                sub_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //SUB A, C
            0x91 => {
                let reg: u8 = self.cpu.registers.c;
                sub_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //SUB A, D
            0x92 => {
                let reg: u8 = self.cpu.registers.d;
                sub_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //SUB A, E
            0x93 => {
                let reg: u8 = self.cpu.registers.e;
                sub_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //SUB A, H
            0x94 => {
                let reg: u8 = self.cpu.registers.h;
                sub_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //SUB A, L
            0x95 => {
                let reg: u8 = self.cpu.registers.l;
                sub_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //SUB A, (HL)
            0x96 => {
                let addr: u16 = self.cpu.registers.hl();
                let u8_value = self.read_mem(addr);
                sub_r_r(&mut self.cpu, u8_value);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //SUB  A, A
            0x97 => {
                let reg: u8 = self.cpu.registers.a;
                sub_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //SBC A, B
            0x98 => {
                let reg: u8 = self.cpu.registers.b;
                sbc_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //SBC A, C
            0x99 => {
                let reg: u8 = self.cpu.registers.c;
                sbc_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //SBC A, D
            0x9A => {
                let reg: u8 = self.cpu.registers.d;
                sbc_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //SBC A, E
            0x9B => {
                let reg: u8 = self.cpu.registers.e;
                sbc_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //SBC A, H
            0x9C => {
                let reg: u8 = self.cpu.registers.h;
                sbc_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //SBC A, L
            0x9D => {
                let reg: u8 = self.cpu.registers.l;
                sbc_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //SBC A, (HL)
            0x9E => {
                let addr: u16 = self.cpu.registers.hl();
                let u8_value = self.read_mem(addr);
                sbc_r_r(&mut self.cpu, u8_value);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //SBC A, A
            0x9F => {
                let reg: u8 = self.cpu.registers.a;
                sbc_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //AND A, B
            0xA0 => {
                let reg: u8 = self.cpu.registers.b;
                and_r_r(&mut self.cpu, reg);
                self.cpu.pc = self.cpu.pc.wrapping_add(1);

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //AND A, C
            0xA1 => {
                let reg: u8 = self.cpu.registers.c;
                and_r_r(&mut self.cpu, reg);
                self.cpu.pc = self.cpu.pc.wrapping_add(1);

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //AND A, D
            0xA2 => {
                let reg: u8 = self.cpu.registers.d;
                and_r_r(&mut self.cpu, reg);
                self.cpu.pc = self.cpu.pc.wrapping_add(1);

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //AND A, E
            0xA3 => {
                let reg: u8 = self.cpu.registers.e;
                and_r_r(&mut self.cpu, reg);
                self.cpu.pc = self.cpu.pc.wrapping_add(1);

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //AND A, H
            0xA4 => {
                let reg: u8 = self.cpu.registers.h;
                and_r_r(&mut self.cpu, reg);
                self.cpu.pc = self.cpu.pc.wrapping_add(1);

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //AND A, L
            0xA5 => {
                let reg: u8 = self.cpu.registers.l;
                and_r_r(&mut self.cpu, reg);
                self.cpu.pc = self.cpu.pc.wrapping_add(1);

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //AND A, (HL)
            0xA6 => {
                let addr: u16 = self.cpu.registers.hl();
                let u8_value = self.read_mem(addr);
                and_r_r(&mut self.cpu, u8_value);
                self.cpu.pc = self.cpu.pc.wrapping_add(1);

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //AND A, A
            0xA7 => {
                let reg: u8 = self.cpu.registers.a;
                and_r_r(&mut self.cpu, reg);
                self.cpu.pc = self.cpu.pc.wrapping_add(1);

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //XOR A, B
            0xA8 => {
                let reg: u8 = self.cpu.registers.b;
                xor_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //XOR A, C
            0xA9 => {
                let reg: u8 = self.cpu.registers.c;
                xor_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //XOR A, D
            0xAA => {
                let reg: u8 = self.cpu.registers.d;
                xor_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //XOR A, E
            0xAB => {
                let reg: u8 = self.cpu.registers.e;
                xor_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //XOR A, H
            0xAC => {
                let reg: u8 = self.cpu.registers.h;
                xor_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //XOR A, L
            0xAD => {
                let reg: u8 = self.cpu.registers.l;
                xor_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //XOR A, (HL)
            0xAE => {
                let addr: u16 = self.cpu.registers.hl();
                let u8_value = self.read_mem(addr);
                xor_r_r(&mut self.cpu, u8_value);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //XOR A, A
            0xAF => {
                let reg: u8 = self.cpu.registers.a;
                xor_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //OR A, B
            0xB0 => {
                let reg: u8 = self.cpu.registers.b;
                or_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //OR A, C
            0xB1 => {
                let reg: u8 = self.cpu.registers.c;
                or_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //OR A, D
            0xB2 => {
                let reg: u8 = self.cpu.registers.d;
                or_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //OR A, E
            0xB3 => {
                let reg: u8 = self.cpu.registers.e;
                or_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //OR A, H
            0xB4 => {
                let reg: u8 = self.cpu.registers.h;
                or_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //OR A, L
            0xB5 => {
                let reg: u8 = self.cpu.registers.l;
                or_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //OR A, (HL)
            0xB6 => {
                let addr: u16 = self.cpu.registers.hl();
                let u8_value = self.read_mem(addr);
                or_r_r(&mut self.cpu, u8_value);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //OR A, A
            0xB7 => {
                let reg: u8 = self.cpu.registers.a;
                or_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //CP A, B
            0xB8 => {
                let reg: u8 = self.cpu.registers.b;
                cp_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //CP A, C
            0xB9 => {
                let reg: u8 = self.cpu.registers.c;
                cp_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //CP A, D
            0xBA => {
                let reg: u8 = self.cpu.registers.d;
                cp_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //CP A, E
            0xBB => {
                let reg: u8 = self.cpu.registers.e;
                cp_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //CP A, H
            0xBC => {
                let reg: u8 = self.cpu.registers.h;
                cp_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //CP A, L
            0xBD => {
                let reg: u8 = self.cpu.registers.l;
                cp_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //CP A, (HL)
            0xBE => {
                let addr: u16 = self.cpu.registers.hl();
                let u8_value = self.read_mem(addr);
                cp_r_r(&mut self.cpu, u8_value);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //CP A, A
            0xBF => {
                let reg: u8 = self.cpu.registers.a;
                cp_r_r(&mut self.cpu, reg);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //RET NZ
            0xC0 => {
                ret_nz(&mut self.cpu, &self.mmu);
            }

            //POP BC
            0xC1 => {
                pop_rr(
                    &self.mmu,
                    &mut self.cpu.registers.b,
                    &mut self.cpu.registers.c,
                    &mut self.cpu.sp,
                );
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(3);
            }

            //JP NZ, u16
            0xC2 => {
                let u16_value = self.cpu.get_u16(&self.mmu);
                jp_nz(&mut self.cpu, u16_value);
            }

            //JP u16
            0xC3 => {
                let u16_value = self.cpu.get_u16(&self.mmu);
                jp(&mut self.cpu, u16_value);

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(4);
            }

            //CALL NZ, u16
            0xC4 => {
                let u16_value: u16 = self.cpu.get_u16(&self.mmu);
                call_nz(&mut self.cpu, &mut self.mmu, u16_value);
            }

            //PUSH BC
            0xC5 => {
                push_rr(
                    &mut self.mmu,
                    self.cpu.registers.b,
                    self.cpu.registers.c,
                    &mut self.cpu.sp,
                );
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(4);
            }

            //ADD A, u8
            0xC6 => {
                let addr = self.cpu.pc + 1;
                let u8_value = self.read_mem(addr);
                add_a_r(&mut self.cpu, u8_value);
                self.cpu.pc += 2;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //RST 0x00(CAll to n)
            0xC7 => {
                rst(&mut self.cpu, &mut self.mmu, 0x00);

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(4);
            }

            //RET Z
            0xC8 => {
                ret_z(&mut self.cpu, &self.mmu);
            }

            //RET
            0xC9 => {
                ret(&mut self.cpu, &self.mmu);
                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(4);
            }

            //JP Z, u16
            0xCA => {
                let u16_value = self.cpu.get_u16(&self.mmu);
                jp_z(&mut self.cpu, u16_value);
            }

            //PREFIX CB
            0xCB => {
                let addr: u16 = self.cpu.pc + 1;

                //Opcode
                let op = &self.read_mem(addr);
                match op {
                    //RLC B
                    0x00 => {
                        rlc(&mut self.cpu.registers.f, &mut self.cpu.registers.b);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RLC C
                    0x01 => {
                        rlc(&mut self.cpu.registers.f, &mut self.cpu.registers.c);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RLC D
                    0x02 => {
                        rlc(&mut self.cpu.registers.f, &mut self.cpu.registers.d);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RLC E
                    0x03 => {
                        rlc(&mut self.cpu.registers.f, &mut self.cpu.registers.e);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RLC H
                    0x04 => {
                        rlc(&mut self.cpu.registers.f, &mut self.cpu.registers.h);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RLC L
                    0x05 => {
                        rlc(&mut self.cpu.registers.f, &mut self.cpu.registers.l);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RLC (HL)
                    0x06 => {
                        let addr = self.cpu.registers.hl();
                        rlc_hl(&mut self.cpu.registers.f, &mut self.mmu, addr);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //RLC A
                    0x07 => {
                        rlc(&mut self.cpu.registers.f, &mut self.cpu.registers.a);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RRC B
                    0x08 => {
                        rrc(&mut self.cpu.registers.f, &mut self.cpu.registers.b);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RRC C
                    0x09 => {
                        rrc(&mut self.cpu.registers.f, &mut self.cpu.registers.c);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RRC D
                    0x0A => {
                        rrc(&mut self.cpu.registers.f, &mut self.cpu.registers.d);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RRC E
                    0x0B => {
                        rrc(&mut self.cpu.registers.f, &mut self.cpu.registers.e);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RRC H
                    0x0C => {
                        rrc(&mut self.cpu.registers.f, &mut self.cpu.registers.h);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RRC L
                    0x0D => {
                        rrc(&mut self.cpu.registers.f, &mut self.cpu.registers.l);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RRC (HL)
                    0x0E => {
                        let addr = self.cpu.registers.hl();
                        rrc_hl(&mut self.cpu.registers.f, &mut self.mmu, addr);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //RRC A
                    0x0F => {
                        rrc(&mut self.cpu.registers.f, &mut self.cpu.registers.a);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RL B
                    0x10 => {
                        rl(&mut self.cpu.registers.f, &mut self.cpu.registers.b);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RL C
                    0x11 => {
                        rl(&mut self.cpu.registers.f, &mut self.cpu.registers.c);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RL D
                    0x12 => {
                        rl(&mut self.cpu.registers.f, &mut self.cpu.registers.d);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RL E
                    0x13 => {
                        rl(&mut self.cpu.registers.f, &mut self.cpu.registers.e);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RL H
                    0x14 => {
                        rl(&mut self.cpu.registers.f, &mut self.cpu.registers.h);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RL L
                    0x15 => {
                        rl(&mut self.cpu.registers.f, &mut self.cpu.registers.l);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RL (HL)
                    0x16 => {
                        let addr = self.cpu.registers.hl();
                        rl_hl(&mut self.cpu.registers.f, &mut self.mmu, addr);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //RL A
                    0x17 => {
                        rl(&mut self.cpu.registers.f, &mut self.cpu.registers.a);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RR B
                    0x18 => {
                        rr(&mut self.cpu.registers.f, &mut self.cpu.registers.b);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RR C
                    0x19 => {
                        rr(&mut self.cpu.registers.f, &mut self.cpu.registers.c);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RR D
                    0x1A => {
                        rr(&mut self.cpu.registers.f, &mut self.cpu.registers.d);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RR E
                    0x1B => {
                        rr(&mut self.cpu.registers.f, &mut self.cpu.registers.e);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RR H
                    0x1C => {
                        rr(&mut self.cpu.registers.f, &mut self.cpu.registers.h);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RR L
                    0x1D => {
                        rr(&mut self.cpu.registers.f, &mut self.cpu.registers.l);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RR (HL)
                    0x1E => {
                        let addr = self.cpu.registers.hl();
                        rr_hl(&mut self.cpu.registers.f, &mut self.mmu, addr);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //RR A
                    0x1F => {
                        rr(&mut self.cpu.registers.f, &mut self.cpu.registers.a);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SLA B
                    0x20 => {
                        sla(&mut self.cpu.registers.f, &mut self.cpu.registers.b);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SLA C
                    0x21 => {
                        sla(&mut self.cpu.registers.f, &mut self.cpu.registers.c);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SLA D
                    0x22 => {
                        sla(&mut self.cpu.registers.f, &mut self.cpu.registers.d);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SLA E
                    0x23 => {
                        sla(&mut self.cpu.registers.f, &mut self.cpu.registers.e);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SLA H
                    0x24 => {
                        sla(&mut self.cpu.registers.f, &mut self.cpu.registers.h);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SLA L
                    0x25 => {
                        sla(&mut self.cpu.registers.f, &mut self.cpu.registers.l);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SLA (HL)
                    0x26 => {
                        let addr = self.cpu.registers.hl();
                        sla_hl(&mut self.cpu.registers.f, &mut self.mmu, addr);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //SLA A
                    0x27 => {
                        sla(&mut self.cpu.registers.f, &mut self.cpu.registers.a);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SRA B
                    0x28 => {
                        sra(&mut self.cpu.registers.f, &mut self.cpu.registers.b);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SRA C
                    0x29 => {
                        sra(&mut self.cpu.registers.f, &mut self.cpu.registers.c);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SRA D
                    0x2A => {
                        sra(&mut self.cpu.registers.f, &mut self.cpu.registers.d);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SRA E
                    0x2B => {
                        sra(&mut self.cpu.registers.f, &mut self.cpu.registers.e);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SRA H
                    0x2C => {
                        sra(&mut self.cpu.registers.f, &mut self.cpu.registers.h);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SRA L
                    0x2D => {
                        sra(&mut self.cpu.registers.f, &mut self.cpu.registers.l);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SRA (HL)
                    0x2E => {
                        let addr = self.cpu.registers.hl();
                        sra_hl(&mut self.cpu.registers.f, &mut self.mmu, addr);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //SRA A
                    0x2F => {
                        sra(&mut self.cpu.registers.f, &mut self.cpu.registers.a);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SWAP B
                    0x30 => {
                        swap(&mut self.cpu.registers.f, &mut self.cpu.registers.b);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SWAP C
                    0x31 => {
                        swap(&mut self.cpu.registers.f, &mut self.cpu.registers.c);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SWAP D
                    0x32 => {
                        swap(&mut self.cpu.registers.f, &mut self.cpu.registers.d);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SWAP E
                    0x33 => {
                        swap(&mut self.cpu.registers.f, &mut self.cpu.registers.e);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SWAP H
                    0x34 => {
                        swap(&mut self.cpu.registers.f, &mut self.cpu.registers.h);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SWAP L
                    0x35 => {
                        swap(&mut self.cpu.registers.f, &mut self.cpu.registers.l);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SWAP (HL)
                    0x36 => {
                        let addr = self.cpu.registers.hl();
                        swap_hl(&mut self.cpu.registers.f, &mut self.mmu, addr);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //SWAP A
                    0x37 => {
                        swap(&mut self.cpu.registers.f, &mut self.cpu.registers.a);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SRL B
                    0x38 => {
                        srl(&mut self.cpu.registers.f, &mut self.cpu.registers.b);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SRL C
                    0x39 => {
                        srl(&mut self.cpu.registers.f, &mut self.cpu.registers.c);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SRL D
                    0x3A => {
                        srl(&mut self.cpu.registers.f, &mut self.cpu.registers.d);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SRL E
                    0x3B => {
                        srl(&mut self.cpu.registers.f, &mut self.cpu.registers.e);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SRL H
                    0x3C => {
                        srl(&mut self.cpu.registers.f, &mut self.cpu.registers.h);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SRL L
                    0x3D => {
                        srl(&mut self.cpu.registers.f, &mut self.cpu.registers.l);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SRL (HL)
                    0x3E => {
                        let addr = self.cpu.registers.hl();
                        srl_hl(&mut self.cpu.registers.f, &mut self.mmu, addr);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //SRL A
                    0x3F => {
                        srl(&mut self.cpu.registers.f, &mut self.cpu.registers.a);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 0, B
                    0x40 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.b, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 0, C
                    0x41 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.c, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 0, D
                    0x42 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.d, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 0, E
                    0x43 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.e, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 0, H
                    0x44 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.h, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 0, L
                    0x45 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.l, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 0, (HL)
                    0x46 => {
                        let addr = self.cpu.registers.hl();
                        bit_n_hl(&mut self.cpu.registers.f, &mut self.mmu, addr, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(3);
                    }

                    //BIT 0, A
                    0x47 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.a, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 1, B
                    0x48 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.b, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 1, C
                    0x49 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.c, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 1, D
                    0x4A => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.d, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 1, E
                    0x4B => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.e, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 1, H
                    0x4C => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.h, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 1, L
                    0x4D => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.l, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 1, (HL)
                    0x4E => {
                        let addr = self.cpu.registers.hl();
                        bit_n_hl(&mut self.cpu.registers.f, &mut self.mmu, addr, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(3);
                    }

                    //BIT 1, A
                    0x4F => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.a, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 2, B
                    0x50 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.b, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 2, C
                    0x51 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.c, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 2, D
                    0x52 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.d, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 2, E
                    0x53 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.e, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 2, H
                    0x54 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.h, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 2, L
                    0x55 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.l, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 2, (HL)
                    0x56 => {
                        let addr = self.cpu.registers.hl();
                        bit_n_hl(&mut self.cpu.registers.f, &mut self.mmu, addr, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(3);
                    }

                    //BIT 2, A
                    0x57 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.a, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 3, B
                    0x58 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.b, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 3, C
                    0x59 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.c, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 3, D
                    0x5A => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.d, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 3, E
                    0x5B => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.e, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 3, H
                    0x5C => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.h, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 3, L
                    0x5D => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.l, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 3, (HL)
                    0x5E => {
                        let addr = self.cpu.registers.hl();
                        bit_n_hl(&mut self.cpu.registers.f, &mut self.mmu, addr, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(3);
                    }

                    //BIT 3, A
                    0x5F => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.a, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 4, B
                    0x60 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.b, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 4, C
                    0x61 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.c, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 4, D
                    0x62 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.d, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 4, E
                    0x63 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.e, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 4, H
                    0x64 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.h, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 4, L
                    0x65 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.l, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 4, (HL)
                    0x66 => {
                        let addr = self.cpu.registers.hl();
                        bit_n_hl(&mut self.cpu.registers.f, &mut self.mmu, addr, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(3);
                    }

                    //BIT 4, A
                    0x67 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.a, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 5, B
                    0x68 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.b, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 5, C
                    0x69 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.c, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 5, D
                    0x6A => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.d, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 5, E
                    0x6B => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.e, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 5, H
                    0x6C => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.h, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 5, L
                    0x6D => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.l, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 5, (HL)
                    0x6E => {
                        let addr = self.cpu.registers.hl();
                        bit_n_hl(&mut self.cpu.registers.f, &mut self.mmu, addr, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(3);
                    }

                    //BIT 5, A
                    0x6F => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.a, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 6, B
                    0x70 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.b, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 6, C
                    0x71 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.c, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 6, D
                    0x72 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.d, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 6, E
                    0x73 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.e, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 6, H
                    0x74 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.h, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 6, L
                    0x75 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.l, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 6, (HL)
                    0x76 => {
                        let addr = self.cpu.registers.hl();
                        bit_n_hl(&mut self.cpu.registers.f, &mut self.mmu, addr, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(3);
                    }

                    //BIT 6, A
                    0x77 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.a, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 7, B
                    0x78 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.b, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 7, C
                    0x79 => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.c, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 7, D
                    0x7A => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.d, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 7, E
                    0x7B => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.e, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 7, H
                    0x7C => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.h, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 7, L
                    0x7D => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.l, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //BIT 7, (HL)
                    0x7E => {
                        let addr = self.cpu.registers.hl();
                        bit_n_hl(&mut self.cpu.registers.f, &mut self.mmu, addr, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(3);
                    }

                    //BIT 7, A
                    0x7F => {
                        bit_n_r(&mut self.cpu.registers.f, &mut self.cpu.registers.a, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 0, B
                    0x80 => {
                        res_n_r(&mut self.cpu.registers.b, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 0, C
                    0x81 => {
                        res_n_r(&mut self.cpu.registers.c, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 0, D
                    0x82 => {
                        res_n_r(&mut self.cpu.registers.d, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 0, E
                    0x83 => {
                        res_n_r(&mut self.cpu.registers.e, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 0, H
                    0x84 => {
                        res_n_r(&mut self.cpu.registers.h, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 0, L
                    0x85 => {
                        res_n_r(&mut self.cpu.registers.l, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 0, (HL)
                    0x86 => {
                        let addr = self.cpu.registers.hl();
                        res_n_hl(&mut self.mmu, addr, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //RES 0, A
                    0x87 => {
                        res_n_r(&mut self.cpu.registers.a, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 1, B
                    0x88 => {
                        res_n_r(&mut self.cpu.registers.b, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 1, C
                    0x89 => {
                        res_n_r(&mut self.cpu.registers.c, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 1, D
                    0x8A => {
                        res_n_r(&mut self.cpu.registers.d, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 1, E
                    0x8B => {
                        res_n_r(&mut self.cpu.registers.e, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 1, H
                    0x8C => {
                        res_n_r(&mut self.cpu.registers.h, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 1, L
                    0x8D => {
                        res_n_r(&mut self.cpu.registers.l, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 1, (HL)
                    0x8E => {
                        let addr = self.cpu.registers.hl();
                        res_n_hl(&mut self.mmu, addr, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //RES 1, A
                    0x8F => {
                        res_n_r(&mut self.cpu.registers.a, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 2, B
                    0x90 => {
                        res_n_r(&mut self.cpu.registers.b, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 2, C
                    0x91 => {
                        res_n_r(&mut self.cpu.registers.c, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 2, D
                    0x92 => {
                        res_n_r(&mut self.cpu.registers.d, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 2, E
                    0x93 => {
                        res_n_r(&mut self.cpu.registers.e, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 2, H
                    0x94 => {
                        res_n_r(&mut self.cpu.registers.h, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 2, L
                    0x95 => {
                        res_n_r(&mut self.cpu.registers.l, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 2, (HL)
                    0x96 => {
                        let addr = self.cpu.registers.hl();
                        res_n_hl(&mut self.mmu, addr, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //RES 2, A
                    0x97 => {
                        res_n_r(&mut self.cpu.registers.a, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 3, B
                    0x98 => {
                        res_n_r(&mut self.cpu.registers.b, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 3, C
                    0x99 => {
                        res_n_r(&mut self.cpu.registers.c, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 3, D
                    0x9A => {
                        res_n_r(&mut self.cpu.registers.d, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 3, E
                    0x9B => {
                        res_n_r(&mut self.cpu.registers.e, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 3, H
                    0x9C => {
                        res_n_r(&mut self.cpu.registers.h, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 3, L
                    0x9D => {
                        res_n_r(&mut self.cpu.registers.l, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 3, (HL)
                    0x9E => {
                        let addr = self.cpu.registers.hl();
                        res_n_hl(&mut self.mmu, addr, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //RES 3, A
                    0x9F => {
                        res_n_r(&mut self.cpu.registers.a, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 4, B
                    0xA0 => {
                        res_n_r(&mut self.cpu.registers.b, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 4, C
                    0xA1 => {
                        res_n_r(&mut self.cpu.registers.c, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 4, D
                    0xA2 => {
                        res_n_r(&mut self.cpu.registers.d, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 4, E
                    0xA3 => {
                        res_n_r(&mut self.cpu.registers.e, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 4, H
                    0xA4 => {
                        res_n_r(&mut self.cpu.registers.h, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 4, L
                    0xA5 => {
                        res_n_r(&mut self.cpu.registers.l, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 4, (HL)
                    0xA6 => {
                        let addr = self.cpu.registers.hl();
                        res_n_hl(&mut self.mmu, addr, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //RES 4, A
                    0xA7 => {
                        res_n_r(&mut self.cpu.registers.a, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 5, B
                    0xA8 => {
                        res_n_r(&mut self.cpu.registers.b, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 5, C
                    0xA9 => {
                        res_n_r(&mut self.cpu.registers.c, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 5, D
                    0xAA => {
                        res_n_r(&mut self.cpu.registers.d, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 5, E
                    0xAB => {
                        res_n_r(&mut self.cpu.registers.e, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 5, H
                    0xAC => {
                        res_n_r(&mut self.cpu.registers.h, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 5, L
                    0xAD => {
                        res_n_r(&mut self.cpu.registers.l, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 5, (HL)
                    0xAE => {
                        let addr = self.cpu.registers.hl();
                        res_n_hl(&mut self.mmu, addr, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //RES 5, A
                    0xAF => {
                        res_n_r(&mut self.cpu.registers.a, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 6, B
                    0xB0 => {
                        res_n_r(&mut self.cpu.registers.b, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 6, C
                    0xB1 => {
                        res_n_r(&mut self.cpu.registers.c, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 6, D
                    0xB2 => {
                        res_n_r(&mut self.cpu.registers.d, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 6, E
                    0xB3 => {
                        res_n_r(&mut self.cpu.registers.e, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 6, H
                    0xB4 => {
                        res_n_r(&mut self.cpu.registers.h, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 6, L
                    0xB5 => {
                        res_n_r(&mut self.cpu.registers.l, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 6, (HL)
                    0xB6 => {
                        let addr = self.cpu.registers.hl();
                        res_n_hl(&mut self.mmu, addr, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //RES 6, A
                    0xB7 => {
                        res_n_r(&mut self.cpu.registers.a, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 7, B
                    0xB8 => {
                        res_n_r(&mut self.cpu.registers.b, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 7, C
                    0xB9 => {
                        res_n_r(&mut self.cpu.registers.c, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 7, D
                    0xBA => {
                        res_n_r(&mut self.cpu.registers.d, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 7, E
                    0xBB => {
                        res_n_r(&mut self.cpu.registers.e, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 7, H
                    0xBC => {
                        res_n_r(&mut self.cpu.registers.h, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 7, L
                    0xBD => {
                        res_n_r(&mut self.cpu.registers.l, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //RES 7, (HL)
                    0xBE => {
                        let addr = self.cpu.registers.hl();
                        res_n_hl(&mut self.mmu, addr, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //RES 7, A
                    0xBF => {
                        res_n_r(&mut self.cpu.registers.a, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 0, B
                    0xC0 => {
                        set_n_r(&mut self.cpu.registers.b, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 0, C
                    0xC1 => {
                        set_n_r(&mut self.cpu.registers.c, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 0, D
                    0xC2 => {
                        set_n_r(&mut self.cpu.registers.d, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 0, E
                    0xC3 => {
                        set_n_r(&mut self.cpu.registers.e, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 0, H
                    0xC4 => {
                        set_n_r(&mut self.cpu.registers.h, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 0, L
                    0xC5 => {
                        set_n_r(&mut self.cpu.registers.l, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 0, (HL)
                    0xC6 => {
                        let addr = self.cpu.registers.hl();
                        set_n_hl(&mut self.mmu, addr, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //SET 0, A
                    0xC7 => {
                        set_n_r(&mut self.cpu.registers.a, 0);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 1, B
                    0xC8 => {
                        set_n_r(&mut self.cpu.registers.b, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 1, C
                    0xC9 => {
                        set_n_r(&mut self.cpu.registers.c, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 1, D
                    0xCA => {
                        set_n_r(&mut self.cpu.registers.d, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 1, E
                    0xCB => {
                        set_n_r(&mut self.cpu.registers.e, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 1, H
                    0xCC => {
                        set_n_r(&mut self.cpu.registers.h, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 1, L
                    0xCD => {
                        set_n_r(&mut self.cpu.registers.l, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 1, (HL)
                    0xCE => {
                        let addr = self.cpu.registers.hl();
                        set_n_hl(&mut self.mmu, addr, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //SET 1, A
                    0xCF => {
                        set_n_r(&mut self.cpu.registers.a, 1);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 2, B
                    0xD0 => {
                        set_n_r(&mut self.cpu.registers.b, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 2, C
                    0xD1 => {
                        set_n_r(&mut self.cpu.registers.c, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 2, D
                    0xD2 => {
                        set_n_r(&mut self.cpu.registers.d, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 2, E
                    0xD3 => {
                        set_n_r(&mut self.cpu.registers.e, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 2, H
                    0xD4 => {
                        set_n_r(&mut self.cpu.registers.h, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 2, L
                    0xD5 => {
                        set_n_r(&mut self.cpu.registers.l, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 2, (HL)
                    0xD6 => {
                        let addr = self.cpu.registers.hl();
                        set_n_hl(&mut self.mmu, addr, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //SET 2, A
                    0xD7 => {
                        set_n_r(&mut self.cpu.registers.a, 2);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 3, B
                    0xD8 => {
                        set_n_r(&mut self.cpu.registers.b, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 3, C
                    0xD9 => {
                        set_n_r(&mut self.cpu.registers.c, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 3, D
                    0xDA => {
                        set_n_r(&mut self.cpu.registers.d, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 3, E
                    0xDB => {
                        set_n_r(&mut self.cpu.registers.e, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 3, H
                    0xDC => {
                        set_n_r(&mut self.cpu.registers.h, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 3, L
                    0xDD => {
                        set_n_r(&mut self.cpu.registers.l, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 3, (HL)
                    0xDE => {
                        let addr = self.cpu.registers.hl();
                        set_n_hl(&mut self.mmu, addr, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //SET 3, A
                    0xDF => {
                        set_n_r(&mut self.cpu.registers.a, 3);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 4, B
                    0xE0 => {
                        set_n_r(&mut self.cpu.registers.b, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 4, C
                    0xE1 => {
                        set_n_r(&mut self.cpu.registers.c, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 4, D
                    0xE2 => {
                        set_n_r(&mut self.cpu.registers.d, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 4, E
                    0xE3 => {
                        set_n_r(&mut self.cpu.registers.e, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 4, H
                    0xE4 => {
                        set_n_r(&mut self.cpu.registers.h, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 4, L
                    0xE5 => {
                        set_n_r(&mut self.cpu.registers.l, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 4, (HL)
                    0xE6 => {
                        let addr = self.cpu.registers.hl();
                        set_n_hl(&mut self.mmu, addr, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //SET 4, A
                    0xE7 => {
                        set_n_r(&mut self.cpu.registers.a, 4);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 5, B
                    0xE8 => {
                        set_n_r(&mut self.cpu.registers.b, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 5, C
                    0xE9 => {
                        set_n_r(&mut self.cpu.registers.c, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 5, D
                    0xEA => {
                        set_n_r(&mut self.cpu.registers.d, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 5, E
                    0xEB => {
                        set_n_r(&mut self.cpu.registers.e, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 5, H
                    0xEC => {
                        set_n_r(&mut self.cpu.registers.h, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 5, L
                    0xED => {
                        set_n_r(&mut self.cpu.registers.l, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 5, (HL)
                    0xEE => {
                        let addr = self.cpu.registers.hl();
                        set_n_hl(&mut self.mmu, addr, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //SET 5, A
                    0xEF => {
                        set_n_r(&mut self.cpu.registers.a, 5);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 6, B
                    0xF0 => {
                        set_n_r(&mut self.cpu.registers.b, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 6, C
                    0xF1 => {
                        set_n_r(&mut self.cpu.registers.c, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 6, D
                    0xF2 => {
                        set_n_r(&mut self.cpu.registers.d, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 6, E
                    0xF3 => {
                        set_n_r(&mut self.cpu.registers.e, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 6, H
                    0xF4 => {
                        set_n_r(&mut self.cpu.registers.h, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 6, L
                    0xF5 => {
                        set_n_r(&mut self.cpu.registers.l, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 6, (HL)
                    0xF6 => {
                        let addr = self.cpu.registers.hl();
                        set_n_hl(&mut self.mmu, addr, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //SET 6, A
                    0xF7 => {
                        set_n_r(&mut self.cpu.registers.a, 6);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 7, B
                    0xF8 => {
                        set_n_r(&mut self.cpu.registers.b, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 7, C
                    0xF9 => {
                        set_n_r(&mut self.cpu.registers.c, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 7, D
                    0xFA => {
                        set_n_r(&mut self.cpu.registers.d, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 7, E
                    0xFB => {
                        set_n_r(&mut self.cpu.registers.e, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 7, H
                    0xFC => {
                        set_n_r(&mut self.cpu.registers.h, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 7, L
                    0xFD => {
                        set_n_r(&mut self.cpu.registers.l, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }

                    //SET 7, (HL)
                    0xFE => {
                        let addr = self.cpu.registers.hl();
                        set_n_hl(&mut self.mmu, addr, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(4);
                    }

                    //SET 7, A
                    0xFF => {
                        set_n_r(&mut self.cpu.registers.a, 7);
                        self.cpu.pc += 2;

                        //Increase Timer
                        self.cpu.timer.internal_ticks =
                            self.cpu.timer.internal_ticks.wrapping_add(2);
                    }
                }
            }

            //CALL Z, u16
            0xCC => {
                let u16_value = self.cpu.get_u16(&self.mmu);
                call_z(&mut self.cpu, &mut self.mmu, u16_value);
            }

            //CALL u16
            0xCD => {
                let u16_value = self.cpu.get_u16(&self.mmu);
                call(&mut self.cpu, &mut self.mmu, u16_value);

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(6);
            }

            //ADC A, u8
            0xCE => {
                let operand = self.read_mem(self.cpu.pc + 1);
                adc_a_r(&mut self.cpu, operand);
                self.cpu.pc += 2;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //RST 0x08
            0xCF => {
                rst(&mut self.cpu, &mut self.mmu, 0x08);

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //RET NC
            0xD0 => {
                ret_nc(&mut self.cpu, &self.mmu);
            }

            //POP DE
            0xD1 => {
                pop_rr(
                    &self.mmu,
                    &mut self.cpu.registers.d,
                    &mut self.cpu.registers.e,
                    &mut self.cpu.sp,
                );
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(3);
            }

            //JP NC, u16
            0xD2 => {
                let u16_value = self.cpu.get_u16(&self.mmu);
                jp_nc(&mut self.cpu, u16_value);
            }

            //Invalid Opcode
            0xD3 => {}

            //CALL NC, u16
            0xD4 => {
                let u16_value = self.cpu.get_u16(&self.mmu);
                call_nc(&mut self.cpu, &mut self.mmu, u16_value);
            }

            //PUSH DE
            0xD5 => {
                push_rr(
                    &mut self.mmu,
                    self.cpu.registers.d,
                    self.cpu.registers.e,
                    &mut self.cpu.sp,
                );
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(4);
            }

            //SUB A, u8
            0xD6 => {
                let u8_value = self.read_mem(self.cpu.pc + 1);
                sub_r_r(&mut self.cpu, u8_value);
                self.cpu.pc += 2;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //RST 0x10
            0xD7 => {
                rst(&mut self.cpu, &mut self.mmu, 0x10);

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(4);
            }

            //RET C
            0xD8 => ret_c(&mut self.cpu, &self.mmu),

            //RETI (NEED TO FIX)
            0xD9 => {
                ret(&mut self.cpu, &self.mmu);
                ei(&mut self.cpu);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(4);
            }

            //JP C, u16
            0xDA => {
                let u16_value = self.cpu.get_u16(&self.mmu);
                jp_c(&mut self.cpu, u16_value);
            }

            //Invalid Opcode
            0xDB => {}

            //CALL C, u16
            0xDC => {
                let u16_value = self.cpu.get_u16(&self.mmu);
                call_c(&mut self.cpu, &mut self.mmu, u16_value);
            }

            //Invalid Opcode
            0xDD => {}

            //SBC A, u8
            0xDE => {
                let u8_value = self.read_mem(self.cpu.pc + 1);
                sbc_r_r(&mut self.cpu, u8_value);
                self.cpu.pc += 2;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //RST 0x18
            0xDF => {
                rst(&mut self.cpu, &mut self.mmu, 0x18);

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(4);
            }

            //LD (0xFF00 + u8), A
            0xE0 => {
                let u8_value: u8 = self.read_mem(self.cpu.pc + 1);
                ld_io_from_a(&mut self.cpu, &mut self.mmu, u8_value);
                self.cpu.pc += 2;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(3);
            }

            //POP HL
            0xE1 => {
                pop_rr(
                    &self.mmu,
                    &mut self.cpu.registers.h,
                    &mut self.cpu.registers.l,
                    &mut self.cpu.sp,
                );
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(3);
            }

            //LD (0xFF00 + C), A
            0xE2 => {
                ld_io_c_from_a(&mut self.cpu, &mut self.mmu);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //Invalid Opcode
            0xE3 => {}

            //Invalid Opcode
            0xE4 => {}

            //PUSH HL
            0xE5 => {
                push_rr(
                    &mut self.mmu,
                    self.cpu.registers.h,
                    self.cpu.registers.l,
                    &mut self.cpu.sp,
                );
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(4);
            }

            //AND A, u8
            0xE6 => {
                let u8_value = self.read_mem(self.cpu.pc + 1);
                and_r_r(&mut self.cpu, u8_value);
                self.cpu.pc += 2;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //RST 0x20
            0xE7 => {
                rst(&mut self.cpu, &mut self.mmu, 0x20);

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(4);
            }

            //ADD SP, i8
            0xE8 => {
                //i8
                let i8_value = self.read_mem(self.cpu.pc + 1) as i8;

                //SP + i8
                let c = self.cpu.sp.wrapping_add(i8_value as u16);

                //Calculate Half Carry
                let half_carry = (c & 0x0F) < (self.cpu.sp & 0x0F);

                //Calculate Carry
                let carry = (c & 0xFF) < (self.cpu.sp & 0xFF);

                //Clear Sub Flag
                self.cpu.registers.f.clear_sub_flag();

                //Clear Zero Flag
                self.cpu.registers.f.clear_zero_flag();

                //Update Half Carry
                if half_carry {
                    self.cpu.registers.f.set_half_carry_flag();
                } else {
                    self.cpu.registers.f.clear_half_carry_flag();
                }

                //Update Carry
                if carry {
                    self.cpu.registers.f.set_carry_flag();
                } else {
                    self.cpu.registers.f.clear_carry_flag();
                }

                //SP = SP + i8
                self.cpu.sp = c;

                self.cpu.pc += 2;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(4);
            }

            //JP HL
            0xE9 => {
                let nn: u16 = self.cpu.registers.hl();
                jp(&mut self.cpu, nn);

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //LD (u16), A
            0xEA => {
                let u16_value = self.cpu.get_u16(&self.mmu);
                self.write_mem(u16_value, self.cpu.registers.a);
                self.cpu.pc += 3;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(4);
            }

            //Invalid Opcode
            0xEB => {}

            //Invalid Opcode
            0xEC => {}

            //Invalid Opcode
            0xED => {}

            //XOR A, u8
            0xEE => {
                let u8_value = self.read_mem(self.cpu.pc + 1);
                xor_r_r(&mut self.cpu, u8_value);
                self.cpu.pc += 2;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //RST 0x28
            0xEF => {
                rst(&mut self.cpu, &mut self.mmu, 0x28);
                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(4);
            }

            //LD A, (FF00+u8)
            0xF0 => {
                let u8_value: u8 = self.read_mem(self.cpu.pc + 1);
                ld_a_from_io(&mut self.cpu, &self.mmu, u8_value);
                self.cpu.pc += 2;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(3);
            }

            //POP AF
            0xF1 => {
                pop_rr(
                    &self.mmu,
                    &mut self.cpu.registers.a,
                    &mut self.cpu.registers.f.data,
                    &mut self.cpu.sp,
                );

                //Clear Lower Nibble of F register
                self.cpu.registers.f.data &= 0xF0;
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(3);
            }

            //LD A, (FF00 + C)
            0xF2 => {
                ld_a_from_io_c(&mut self.cpu, &self.mmu);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //DI
            0xF3 => {
                di(&mut self.cpu);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //Invalid Opcode
            0xF4 => {}

            //PUSH AF
            0xF5 => {
                push_rr(
                    &mut self.mmu,
                    self.cpu.registers.a,
                    self.cpu.registers.f.data,
                    &mut self.cpu.sp,
                );
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(4);
            }

            //OR A, u8
            0xF6 => {
                let u8_value = self.read_mem(self.cpu.pc + 1);
                or_r_r(&mut self.cpu, u8_value);
                self.cpu.pc += 2;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //RST 0x30
            0xF7 => {
                rst(&mut self.cpu, &mut self.mmu, 0x30);

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(4);
            }

            //LD HL, SP+i8
            0xF8 => {
                //i8
                let i8_value = self.read_mem(self.cpu.pc + 1) as i8;

                //SP + i8
                let c: u16 = self.cpu.sp.wrapping_add(i8_value as u16);

                //Calculate Half Carry
                let half_carry = (c & 0x0F) < (self.cpu.sp & 0x0F);

                //Calculate Carry
                let carry = (c & 0xFF) < (self.cpu.sp & 0xFF);

                //Clear Sub Flag
                self.cpu.registers.f.clear_sub_flag();

                //Clear Zero Flag
                self.cpu.registers.f.clear_zero_flag();

                //Update Half Carry
                if half_carry {
                    self.cpu.registers.f.set_half_carry_flag();
                } else {
                    self.cpu.registers.f.clear_half_carry_flag();
                }

                //Update Carry
                if carry {
                    self.cpu.registers.f.set_carry_flag();
                } else {
                    self.cpu.registers.f.clear_carry_flag();
                }

                //HL = SP + i8
                self.cpu.registers.set_hl(c);

                self.cpu.pc += 2;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(3);
            }

            //LD SP, HL
            0xF9 => {
                self.cpu.sp = self.cpu.registers.hl();
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //LD A, (u16)
            0xFA => {
                let addr = self.cpu.get_u16(&&self.mmu);
                let u8_value = self.read_mem(addr);
                ld_8bit(&mut self.cpu.registers.a, u8_value);
                self.cpu.pc += 3;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(4);
            }

            //EI
            0xFB => {
                ei(&mut self.cpu);
                self.cpu.pc += 1;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(1);
            }

            //Invalid Opcode
            0xFC => {}

            //Invalid Opcode
            0xFD => {}

            //CP A, u8
            0xFE => {
                let u8_value = self.read_mem(self.cpu.pc + 1);
                cp_r_r(&mut self.cpu, u8_value);
                self.cpu.pc += 2;

                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            }

            //RST 0x38
            0xFF => {
                rst(&mut self.cpu, &mut self.mmu, 0x38);
                //Increase Timer
                self.cpu.timer.internal_ticks = self.cpu.timer.internal_ticks.wrapping_add(2);
            } //_ => println!("NOT AN OPCODE"),
        }
    }

    pub fn write_mem(&mut self, addr: u16, value: u8) {
        if addr >= 0xFF04 && addr <= 0xFF07 {
            match addr {
                0xFF04 => self.timer.div = 0,

                0xFF05 => self.timer.tima = value,

                0xFF06 => self.timer.tma = value,

                0xFF07 => self.timer.tac = value,

                _ => (),
            }
        } else {
            self.mmu.memory[addr as usize] = value;
        }
    }

    pub fn read_mem(&self, addr: u16) -> u8 {
        if addr >= 0xFF04 && addr <= 0xFF07 {
            match addr {
                0xFF04 => return self.timer.div as u8,

                0xFF05 => return self.timer.tima,

                0xFF06 => return self.timer.tma,

                0xFF07 => return self.timer.tac,

                _ => return 0,
            }
        } else {
            self.mmu.memory[addr as usize]
        }
    }

    pub fn emu_cycles(&mut self, n: u32) {
        for _ in 0..n {
            self.cpu.timer.internal_ticks += 1;
            self.cpu.timer.do_cycle(n);
        }
    }
}
