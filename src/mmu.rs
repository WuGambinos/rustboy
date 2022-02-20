struct MMU {
    boot: [u8; 255],

    memory: [u8; 0xFF00],
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

impl MMU {
    fn new() -> Self {
        MMU {
            boot: [255; 0],
            memory: [0; 0xFF00],
            interrupt_en: 0,
        }
    }
}
