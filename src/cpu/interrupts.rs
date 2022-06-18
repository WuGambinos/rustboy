use crate::interconnect::Interconnect;

pub enum InterruptType {
    VBlank,
    LcdStat,
    Timer,
    Serial,
    Joypad,
}

pub fn interrupt_request(interconnect: &mut Interconnect, interrput_type: InterruptType) {
    match interrput_type {
        InterruptType::VBlank => {
            //Interrupt Flag
            let mut value = interconnect.read_mem(0xFF0F);
            value |= 1 << 0;
            interconnect.write_mem(0xFF0F, value);
        }
        InterruptType::LcdStat => {
            //Interrupt Flag
            let mut value = interconnect.read_mem(0xFF0F);
            value |= 1 << 1;
            interconnect.write_mem(0xFF0F, value);
        }
        InterruptType::Timer => {
            //Interrupt Flag
            let mut value = interconnect.read_mem(0xFF0F);
            value |= 1 << 2;
            interconnect.write_mem(0xFF0F, value);
        }
        InterruptType::Serial => {
            //Interrupt Flag
            let mut value = interconnect.read_mem(0xFF0F);
            value |= 1 << 3;
            interconnect.write_mem(0xFF0F, value);
        }
        InterruptType::Joypad => {
            //Interrupt Flag
            let mut value = interconnect.read_mem(0xFF0F);
            value |= 1 << 4;
            interconnect.write_mem(0xFF0F, value);
        }
    }
}
