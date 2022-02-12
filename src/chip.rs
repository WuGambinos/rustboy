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
    sp: u8,

    ///Program counter
    pc: u8,

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

    fn emulate_cycle(&mut self) {
        self.fetch();

        match self.opcode {
            //NOP
            0x00 => self.pc += 1,

            //LD BC, u16
            0x01 => {
                //Grab u16 value
                let data = self.get_u16();

                //BC = u16
                self.registers.set_bc(data);

                //Increase program counter
                self.pc += 3;
            }

            //LD (BC), A
            0x02 => self.memory[self.registers.bc() as usize] = self.registers.a,

            //INC BC
            0x03 => self.registers.set_bc(self.registers.bc().wrapping_add(1)),

            //0x04 INC B: Flags:Z0H
            //0x04 => self.registers.b = self.registers.b.wrapping_add(1),
            0x04 => {
                //self.inc_8bit_register(&self.registers.b);
                self.pc += 1
            }

            _ => println!("NOT AN OPCODE"),
        }
    }

    fn fetch(&mut self) {
        self.opcode = self.memory[self.pc as usize]
    }

    fn get_u16(&mut self) -> u16 {
        (self.memory[(self.pc + 1) as usize] as u16) << 8
            | (self.memory[(self.pc + 2) as usize]) as u16
    }

    fn load_program(&mut self, rom: &[u8]) {}

    fn load_boot(&mut self, rom: &[u8]) {}

    /*************************************************************************
     * INSTRUCTIONS
     *************************************************************************/

    fn inc_8bit_register(&mut self, reg: char) {
        match reg {
            'A' => {
                let res: Option<u8> = self.registers.a.checked_add(1);
                self.registers.a = match res {
                    Some(v) => v,

                    None => self.registers.a.wrapping_add(1),
                }
            }

            'B' => {
                let res: Option<u8> = self.registers.b.checked_add(1);
                self.registers.b = match res {
                    Some(v) => v,

                    None => self.registers.b.wrapping_add(1),
                }
            }

            'C' => {
                let res: Option<u8> = self.registers.c.checked_add(1);
                self.registers.c = match res {
                    Some(v) => v,
                    None => self.registers.c.wrapping_add(1),
                }
            }

            'D' => {
                let res: Option<u8> = self.registers.d.checked_add(1);
                self.registers.d = match res {
                    Some(v) => v,
                    None => self.registers.d.wrapping_add(1),
                }
            }

            'E' => {
                let res: Option<u8> = self.registers.e.checked_add(1);
                self.registers.e = match res {
                    Some(v) => v,
                    None => self.registers.e.wrapping_add(1),
                }
            }
            'H' => {
                let res: Option<u8> = self.registers.h.checked_add(1);
                self.registers.h = match res {
                    Some(v) => v,
                    None => self.registers.h.wrapping_add(1),
                }
            }
            'L' => {
                let res: Option<u8> = self.registers.l.checked_add(1);
                self.registers.l = match res {
                    Some(v) => v,
                    None => self.registers.l.wrapping_add(1),
                }
            }
            _ => (),
        };
    }
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

        cpu.memory[0] = 0x01;
        cpu.memory[1] = 0xFA;
        cpu.memory[2] = 0xDC;

        cpu.emulate_cycle();

        assert_eq!(cpu.registers.bc(), 0xFADC);
    }

    #[test]
    fn inc_bc() {
        let mut cpu = Cpu::new();

        cpu.registers.set_bc(0xFFFF);
        cpu.registers.set_bc(cpu.registers.bc().wrapping_add(1));

        assert_eq!(cpu.registers.bc(), 0);
    }
}
