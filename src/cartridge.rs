#[derive(Debug)]
pub enum CartridgeType {
    RomOnly = 0x00,
    Mbc1 = 0x01,
    Mbc1WithRam = 0x02,
    Mbc1WithRamAndBattery = 0x03,
    Mbc2 = 0x05,
    Mbc2WithBattery = 0x06,
    RomWithRam1 = 0x08,
    RomWithRamAndBattery1 = 0x09,
    Mmm01 = 0x0B,
    Mmm01WithRam = 0x0C,
    Mmm01WithRamAndBattery = 0x0D,
    Mbc3WithTimerAndBattery = 0x0F,
    Mbc3WithTimerRamAndBattery2 = 0x10,
    Mbc3 = 0x11,
    Mbc3WithRam2 = 0x12,
    Mbc3WithRamAndBattery2 = 0x13,
    Mbc5 = 0x19,
    Mbc5WithRam = 0x1A,
    Mbc5WithRamAndBattery = 0x1B,
    Mbc5WithRumble = 0x1C,
    Mbc5WithRumbleAndRam = 0x1D,
    Mbc5WithRumbleRamAndBattery = 0x1E,
    Mbc6 = 0x20,
    Mbc7WithSensorRumbleRamAndBattery = 0x22,
    PocketCamera = 0xFC,
    BandaiTama5 = 0xFD,
    HuC3 = 0xFE,
    HuC1WithRamAndBattery = 0xFF,
}

fn u8_to_cartridge_type(value: u8) -> Option<CartridgeType> {
    match value {
        0x00 => Some(CartridgeType::RomOnly),
        0x01 => Some(CartridgeType::Mbc1),
        0x02 => Some(CartridgeType::Mbc1WithRam),
        0x03 => Some(CartridgeType::Mbc1WithRamAndBattery),
        0x05 => Some(CartridgeType::Mbc2),
        0x06 => Some(CartridgeType::Mbc2WithBattery),
        0x08 => Some(CartridgeType::RomWithRam1),
        0x09 => Some(CartridgeType::RomWithRamAndBattery1),
        0x0B => Some(CartridgeType::Mmm01),
        0x0C => Some(CartridgeType::Mmm01WithRam),
        0x0D => Some(CartridgeType::Mmm01WithRamAndBattery),
        0x0F => Some(CartridgeType::Mbc3WithTimerAndBattery),
        0x10 => Some(CartridgeType::Mbc3WithTimerRamAndBattery2),
        0x11 => Some(CartridgeType::Mbc3),
        0x12 => Some(CartridgeType::Mbc3WithRam2),
        0x13 => Some(CartridgeType::Mbc3WithRamAndBattery2),
        0x19 => Some(CartridgeType::Mbc5),
        0x1A => Some(CartridgeType::Mbc5WithRam),
        0x1B => Some(CartridgeType::Mbc5WithRamAndBattery),
        0x1C => Some(CartridgeType::Mbc5WithRumble),
        0x1D => Some(CartridgeType::Mbc5WithRumbleAndRam),
        0x1E => Some(CartridgeType::Mbc5WithRumbleRamAndBattery),
        0x20 => Some(CartridgeType::Mbc6),
        0x22 => Some(CartridgeType::Mbc7WithSensorRumbleRamAndBattery),
        0xFC => Some(CartridgeType::PocketCamera),
        0xFD => Some(CartridgeType::BandaiTama5),
        0xFE => Some(CartridgeType::HuC3),
        0xFF => Some(CartridgeType::HuC1WithRamAndBattery),
        _ => None,
    }
}

#[derive(Debug)]
pub struct Cartridge {
    pub cartridge_info: CartridgeInfo,
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
}

impl Cartridge {
    pub fn new() -> Cartridge {
        Cartridge {
            cartridge_info: CartridgeInfo::new(),
            rom: Vec::new(),
            ram: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct CartridgeInfo {
    pub logo: [u8; 48],
    pub title: String,
    pub cartridge_type: CartridgeType,
    pub rom_size: u32,
    pub ram_size: u8,
}

impl CartridgeInfo {
    pub fn new() -> CartridgeInfo {
        CartridgeInfo {
            logo: [0; 48],
            title: String::new(),
            cartridge_type: CartridgeType::RomOnly,
            rom_size: 0,
            ram_size: 0,
        }
    }
    pub fn calc_header_checksum(&self, rom: Vec<u8>) -> u8 {
        let mut checksum: u8 = 0;
        for address in 0x134..=0x14C {
            checksum = checksum.wrapping_sub(rom[address as usize]).wrapping_sub(1);
        }

        checksum
    }

    pub fn cartridge_type(&mut self, rom: Vec<u8>) {
        self.cartridge_type = u8_to_cartridge_type(rom[0x147]).unwrap();
    }

    pub fn rom_size(&mut self, rom: Vec<u8>) {
        self.rom_size = 32 * (1 << rom[0x148]);
    }

    pub fn title(&mut self, rom: Vec<u8>) {
        for i in 0..16 {
            let ch: char = rom[0x134 + i].to_ascii_uppercase() as char;

            if ch != '\0' {
                self.title.push(ch);
            }
        }
        println!("TITLE: {}", self.title.as_str());
    }
}

impl std::fmt::Display for Cartridge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let header_checksum = self.cartridge_info.calc_header_checksum(self.rom.to_vec());
        write!(
            f,
            " CHECKSUM: {:#X} {}\n CARTRIDGE TYPE: {:?} \n ROM_SIZE: {} KiB\n TITLE: {:#?}",
            header_checksum,
            (header_checksum & 0xFF) != 0,
            self.cartridge_info.cartridge_type,
            self.cartridge_info.rom_size,
            self.cartridge_info.title,
        )
    }
}
