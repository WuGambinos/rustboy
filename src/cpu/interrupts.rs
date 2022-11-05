use super::Interconnect;

/// Different types of interrupts
pub enum InterruptType {
    VBlank,
    LcdStat,
    Timer,
    Serial,
    Joypad,
}

/// Trigger interrupt depending on type passed 
pub fn interrupt_request(interconnect: &mut Interconnect, interrput_type: InterruptType) -> u8 {
    match interrput_type {
        InterruptType::VBlank => {
            //Interrupt Flag
            let mut value = interconnect.read_mem(0xFF0F);
            value |= 1 << 0;
            interconnect.write_mem(0xFF0F, value);
            0
        }
        InterruptType::LcdStat => {
            //Interrupt Flag
            let mut value = interconnect.read_mem(0xFF0F);
            value |= 1 << 1;
            interconnect.write_mem(0xFF0F, value);
            1 
        }
        InterruptType::Timer => {
            //Interrupt Flag
            let mut value = interconnect.read_mem(0xFF0F);
            value |= 1 << 2;
            interconnect.write_mem(0xFF0F, value);
            2
        }
        InterruptType::Serial => {
            //Interrupt Flag
            let mut value = interconnect.read_mem(0xFF0F);
            value |= 1 << 3;
            interconnect.write_mem(0xFF0F, value);
            3
        }
        InterruptType::Joypad => {
            //Interrupt Flag
            let mut value = interconnect.read_mem(0xFF0F);
            value |= 1 << 4;
            interconnect.write_mem(0xFF0F, value);
            4
        }
    }
}
