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

pub struct Cpu {

    memory: [u8; 65536],

    ///Accumulator
    a: u8,

    ///Flags
    f: Flags,

    ///B register
    b: u8,

    ///C register
    c: u8,

    ///D register
    d: u8,

    ///E register
    e: u8,

    ///H register
    h: u8,

    ///L register
    l: u8,

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
            a: 0,
            f: Flags::new(),
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            opcode: 0,
        }

    }

    fn emulate_cycle(&self) {

    }

    fn load_program(&mut self, rom: &[u8]) {

    }

    fn load_boot(&mut self, rom: &[u8]) {

    }


}