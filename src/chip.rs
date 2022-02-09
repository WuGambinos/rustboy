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
    fn bc(&self) -> u16  {
        (self.b as u16) << 8 | self.c as u16
    }

    ///Store value in register pair BC
    fn set_bc(&mut self, data: u16) {
        self.b = ((data & 0xFF00) >> 8) as u8;
        self.c = ((data & 0x00FF)) as u8;
    }

    ///Get register pair DE
    fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    ///Store value in register pair DE
    fn set_de(&mut self, data: u16) {
        self.d = ((data & 0xFF00) >> 8) as u8;
        self.e = ((data & 0x00FF)) as u8;
    }

    ///Get register pair HL
    fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    ///Store value in register pair HL
    fn set_hl(&mut self, data: u16) {
        self.h = ((data & 0xFF00) >> 8) as u8;
        self.l = ((data & 0x00FF)) as u8;
    }
}



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

impl Cpu {
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

    }

    fn load_program(&mut self, rom: &[u8]) {

    }

    fn load_boot(&mut self, rom: &[u8]) {

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
        let mut regs = Registers::new();
        let data: u16 = 0xFE67;
        regs.set_bc(data);
        assert_eq!(regs.bc(), data);

    }

}