#[derive(Debug)]
pub struct Mmu {
    pub memory: [u8; 0x10000],

    //Interrupt Enable Register
    interrupt_en: u8,

    //Interrupt Flag Register,
    interrupt_flag: u8,
    /*rom_bank: [u8; 16384],
    extra_rom_bank: [u8; 16384],

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
    pub fn new() -> Self {
        Mmu {
            memory: [0; 0x10000],
            interrupt_en: 0,
            interrupt_flag: 0,
        }
    }

    pub fn write_mem(&mut self, addr: u16, value: u8) {
        if addr >= 0xFF04 && addr <= 0xFF07 {
        } else {
            self.memory[addr as usize] = value;
        }
    }

    pub fn read_mem(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }


}
