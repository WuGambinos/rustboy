use super::Interconnect;
use crate::constants::*;

/// Different types of interrupts
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InterruptType {
    VBlank,
    LcdStat,
    Timer,
    Serial,
    Joypad,
}

pub fn is_interrupt_enabled(interconnect: &mut Interconnect, index: usize) -> bool {
    let IE = interconnect.read_mem(INTERRUPT_ENABLE);

    (IE & (1 << index)) > 0
}

pub fn is_interrupt_requested(interconnect: &mut Interconnect, index: usize) -> bool {
    let IF = interconnect.read_mem(INTERRUPT_FLAG);
    (IF & (1 << index)) > 0
}

pub fn request_interrupt(interconnect: &mut Interconnect, interrupt: InterruptType) {

    let mut IR = interconnect.read_mem(INTERRUPT_FLAG);

    let bit = INTERRUPTS.iter().position(|&i| i == interrupt).unwrap();

    IR |= (1 << bit);

    interconnect.write_mem(INTERRUPT_FLAG, IR);
}

pub fn get_interrupt(interconnect: &mut Interconnect) -> Option<InterruptType> {
    for i in 0..INTERRUPTS.len() {
        if is_interrupt_enabled(interconnect, i) && is_interrupt_requested(interconnect, i) {
            return Some(INTERRUPTS[i]);
        }
    }

    None
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
