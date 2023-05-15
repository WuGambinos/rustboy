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
    let int_enable = interconnect.read_mem(INTERRUPT_ENABLE);

    (int_enable & (1 << index)) > 0
}

pub fn is_interrupt_requested(interconnect: &mut Interconnect, index: usize) -> bool {
    let int_flag = interconnect.read_mem(INTERRUPT_FLAG);
    (int_flag & (1 << index)) > 0
}

pub fn request_interrupt(interconnect: &mut Interconnect, interrupt: InterruptType) {
    let mut int_request = interconnect.read_mem(INTERRUPT_FLAG);

    let bit = INTERRUPTS.iter().position(|&i| i == interrupt).unwrap();

    int_request |= 1 << bit;

    interconnect.write_mem(INTERRUPT_FLAG, int_request);
}

pub fn get_interrupt(interconnect: &mut Interconnect) -> Option<InterruptType> {
    for (i, interrupt) in INTERRUPTS.iter().enumerate() {
        if is_interrupt_enabled(interconnect, i) && is_interrupt_requested(interconnect, i) {
            return Some(*interrupt);
        }
    }
    None
}
