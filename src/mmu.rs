use raylib::models::RaylibMesh;

#[derive(Debug)]
pub struct MMU {
    memory: [u8; 0xFFFF],
    //Interrupt Enable Register
    interrupt_en: u8,
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

impl Default for MMU {
    fn default() -> Self {
        Self::new()
    }
}

impl MMU {
    pub fn new() -> Self {
        MMU {
            memory: [0; 0xFFFF],
            interrupt_en: 0,
        }
    }

    pub fn write_mem(&mut self, value: u8, addr: u16) {
        self.memory[addr as usize] = value;
    }

    pub fn read_mem(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
}
