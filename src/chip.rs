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

    fn ld_bc_16(&mut self) {


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
    fn register_test() {
        let mut b: Register = Register::new();
        let mut c: Register = Register::new();

        let mut bc: RegisterPair = RegisterPair::new(b, c);

        b.data = 1;
        c.data = 1;

        assert_eq!(b.data, c.data);

        //assert_eq!(bc.data, 257);

    }

}