/// Memory Mapped Unit
#[derive(Debug)]
pub struct Mmu {
    //pub memory: [u8; 0x10000],
    /// ROM Bank
    pub rom_bank: [u8; 0x8000],

    /// IO Registers
    pub io: [u8; 0x80],

    /// High RAM (HRAM)
    pub hram: [u8; 0x7F],

    /// External RAM
    pub external_ram: [u8; 0x2000],

    /// Work RAM
    pub work_ram: [u8; 0x2000],

    pub interrupt_enable: u8,
    /*
    //Video Ram
    video_ram: [u8; 8192],

    //External RAM
    external_ram: [u8; 8192],
    //Work RAM
    work_ram_b0: [u8; 4096],
    work_ram_b1: [u8; 4096],

    //ECHO RAM(Mirror of C000 - DDFF)
    echo_ram: [u8; 7680],

    //Sprite Attribute Table(OAM)
    sprite_attribute_table: [u8; 160],

    //I/O Registers
    io: [u8; 128],

    //High Ram
    high_ram: [u8; 127],*/
}

impl Default for Mmu {
    fn default() -> Self {
        Self::new()
    }
}

impl Mmu {
    /// Constructor
    pub fn new() -> Self {
        Mmu {
            //memory: [0; 0x10000],
            rom_bank: [0; 0x8000],
            io: [0; 0x80],
            hram: [0; 0x7F],
            external_ram: [0; 0x2000],
            work_ram: [0; 0x2000],
            interrupt_enable: 0,
        }
    }

    /*  pub fn write_mem(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    pub fn read_mem(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
    */
}
