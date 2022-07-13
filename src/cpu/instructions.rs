use super::{Cpu, Flags, Interconnect};

/************************************************************************
 * 8-bit Arithmetic instructions
 * *********************************************************************/

/// Increment 8-bit register
///
/// Flags:
///
/// Zero: Dependent
///
/// Sub: Clear
///
/// Half Carry: Dependent
pub fn inc_8bit(flags: &mut Flags, register: &mut u8) {
    let mut value: u8 = *register;

    //Update Half Carry
    flags.update_half_carry_flag_sum_8bit(value, 1);

    //r = r + 1
    value = value.wrapping_add(1);

    //Update Zero Flag
    flags.update_zero_flag(value);

    //Clear Sub Flag
    flags.clear_sub_flag();

    //Store value back in register
    *register = value;
}


/// Decrement 8-bit register
///
/// Flags:
///
/// Zero: Dependent
///
/// Sub: Set
///
/// Half Carry: Dependent 
pub fn dec_8bit(flags: &mut Flags, register: &mut u8) {
    let mut value = *register;

    //Update Half Carry
    flags.update_half_carry_flag_sub_8bit(value, 1);

    //r = r - 1
    value = value.wrapping_sub(1);

    //Update Zero Flag
    flags.update_zero_flag(value);

    //Set Sub Flag
    flags.set_sub_flag();

    //Store new value back in register
    *register = value;
}

/// Increment value in memory using HL pointer
///
/// Flags:
///
/// Zero: Dependent
///
/// Sub Flag: Clear
///
/// Half Carry: Dependent
pub fn inc_mem(cpu: &mut Cpu, interconnect: &mut Interconnect) {
    let hl = cpu.registers.hl();

    // Grab value in memory
    let mut value = interconnect.read_mem(hl);

    // Increase Timer
    interconnect.emu_cycles(1);

    // Check for Half Carry
    cpu.registers.f.update_half_carry_flag_sum_8bit(value, 1);

    // Increment value
    value = value.wrapping_add(1);

    // Write new incremented value back into memory
    interconnect.write_mem(cpu.registers.hl(), value);

    // Increase Timer
    interconnect.emu_cycles(1);

    // Update Zero Flag
    cpu.registers.f.update_zero_flag(value);

    // Clear Sub Flag
    cpu.registers.f.clear_sub_flag();
}

/// Decrement value in memory using HL pointer
///
/// Flags:
///
/// Zero: Dependent
///
/// Sub: Set
///
/// Half Carry: Dependent
pub fn dec_mem(cpu: &mut Cpu, interconnect: &mut Interconnect) {
    // Grab value in memory
    let mut value: u8 = interconnect.read_mem(cpu.registers.hl());

    // Increase Timer
    interconnect.emu_cycles(1);

    // Check for Half Carry
    cpu.registers.f.update_half_carry_flag_sub_8bit(value, 1);

    // Decrement Value
    value = value.wrapping_sub(1);

    // Write new decremented value back into memory
    interconnect.write_mem(cpu.registers.hl(), value);

    // Increase Timer
    interconnect.emu_cycles(1);

    // Update Zero Flag
    cpu.registers.f.update_zero_flag(value);

    // Set sub flag
    cpu.registers.f.set_sub_flag();
}

///Adds Accumulator(register A) and another register together, storing result in the accumulator
///
/// a = r + a
///
/// Flags: Z0HC
///
/// Zero: Dependent
///
/// Sub: Clear
///
/// Half Carry: Dependent
///
/// Carry: Dependent
pub fn add_a_r(cpu: &mut Cpu, operand: u8) {
    let mut a: u8 = cpu.registers.a;
    // Update Half Carry
    cpu.registers.f.update_half_carry_flag_sum_8bit(a, operand);

    // Update Carry Flag
    cpu.registers.f.update_carry_flag_sum_8bit(a, operand);

    // a = r + a
    a = a.wrapping_add(operand);

    // Clear Sub Flag
    cpu.registers.f.clear_sub_flag();

    // Update Zero Flag
    cpu.registers.f.update_zero_flag(a);

    // Store new value in accumulator
    cpu.registers.a = a;
}

/// Adds Accumulator(register A), another register, and carry all together, storing result in the accumulator
///
/// a = a + r + c
///
/// Flags:
///
/// Zero: Dependent
///
/// Sub: Clear
///
/// Half Carry: Dependent
///
/// Carry: Dependent
pub fn adc_a_r(cpu: &mut Cpu, operand: u8) {
    // Accumulator
    let mut a: u8 = cpu.registers.a;

    // Result
    let c: u16 = (a as u16) + (operand as u16) + (cpu.registers.f.carry_flag() as u16);

    // Calculate Half Carry
    let half_carry: bool = ((a & 0x0F) + (operand & 0x0F) + cpu.registers.f.carry_flag()) > 0x0F;

    // Clear Sub Flag
    cpu.registers.f.clear_sub_flag();

    // Update Half Carry Flag
    if half_carry {
        cpu.registers.f.set_half_carry_flag();
    } else {
        cpu.registers.f.clear_half_carry_flag();
    }

    // Update Carry Flag
    if c > 0xFF {
        cpu.registers.f.set_carry_flag();
    } else {
        cpu.registers.f.clear_carry_flag();
    }

    // Update Zero Flag
    cpu.registers.f.update_zero_flag(c as u8);

    // Store new value in accumulator
    cpu.registers.a = c as u8;
}

/// Subtracts another register from the accumulator, storing the result in the accumulator
///
/// a = a - r
///
/// Flags:
///
/// Zero: Dependent
///
/// Sub: Set
///
/// Half Carry: Dependent
///
/// Carry: Dependent
pub fn sub_a_r(cpu: &mut Cpu, operand: u8) {
    let mut a: u8 = cpu.registers.a;

    // Update Half Carry
    cpu.registers.f.update_half_carry_flag_sub_8bit(a, operand);

    // Update Carry(Borrow) Flag
    cpu.registers.f.update_carry_flag_sub_8bit(a, operand);

    // a = a - r
    a = a.wrapping_sub(operand);

    // Set Sub flag
    cpu.registers.f.set_sub_flag();

    // Update Zero Flag
    cpu.registers.f.update_zero_flag(a);

    // Store new value in accumulator  
    cpu.registers.a = a;
}


/// Subtracts another register and carry from the accumulator, storing the result in the accumulator
///
/// a = a - r - c
///
/// Flags:
///
/// Zero: Dependent
///
/// Sub: Set
///
/// Half Carry: Dependent
///
/// Carry: Dependent
pub fn sbc_a_r(cpu: &mut Cpu, operand: u8) {
    // Accumulator
    let a = cpu.registers.a as i16;

    // Operand
    let b = operand as i16;

    // Result
    let c = a
        .wrapping_sub(b)
        .wrapping_sub(cpu.registers.f.carry_flag() as i16);

    // Calculate Half Carry
    let half_carry = ((a & 0x0F) - (b & 0x0F) - (cpu.registers.f.carry_flag() as i16)) < 0;

    // Set Sub Flag
    cpu.registers.f.set_sub_flag();

    // Update Half Carry Flag
    if half_carry {
        cpu.registers.f.set_half_carry_flag();
    } else {
        cpu.registers.f.clear_half_carry_flag();
    }

    // Update Carry(Borrow) Flag
    if c < 0 {
        cpu.registers.f.set_carry_flag();
    } else {
        cpu.registers.f.clear_carry_flag();
    }

    // Update Zero Flag
    cpu.registers.f.update_zero_flag(c as u8);

    // Store new value in accumulator
    cpu.registers.a = c as u8;
}

/// Stores the logical "and" of accumulator and register in accumluator 
///
/// a = a & r
///
/// Flags:
///
/// Zero: Dependent
///
/// Sub: Clear
///
/// Half Carry: Set
///
/// Carry: Clear
pub fn and_a_r(cpu: &mut Cpu, operand: u8) {
    let mut a: u8 = cpu.registers.a;

    // and
    a &= operand;

    // Update Zero Flag
    cpu.registers.f.update_zero_flag(a);

    // Clear Sub Flag
    cpu.registers.f.clear_sub_flag();

    // Set Half Carry
    cpu.registers.f.set_half_carry_flag();

    // Clear Carry Flag
    cpu.registers.f.clear_carry_flag();

    // Store new value in accumulator
    cpu.registers.a = a;
}

/// Stores logical xor of accumulator and register in accumulator
///
/// a = a ^ r
///
/// Flags:
///
/// Zero: Dependent
///
/// Sub: Clear
///
/// Half Carry: Clear
///
/// Carry: Clear
pub fn xor_a_r(cpu: &mut Cpu, operand: u8) {
    let mut a: u8 = cpu.registers.a;

    // xor
    a ^= operand;

    // Update Zero Flag
    cpu.registers.f.update_zero_flag(a);

    // Clear Sub Flag
    cpu.registers.f.clear_sub_flag();

    // Clear Half Carry
    cpu.registers.f.clear_half_carry_flag();

    // Clear Carry
    cpu.registers.f.clear_carry_flag();

    // Set actual accumualtor equal to resulting value
    cpu.registers.a = a;
}

/// Stores logical or of accumulator and register in accumulator
///
/// a = a | r
///
/// Flags:
///
/// Zero: Dependent
///
/// Sub: Clear
///
/// Half Carry: Clear
///
/// Carry: Clear
///
pub fn or_a_r(cpu: &mut Cpu, operand: u8) {
    let mut a: u8 = cpu.registers.a;

    //or
    a |= operand;

    //Update Zero Flag
    cpu.registers.f.update_zero_flag(a);

    //Clear Sub Flag
    cpu.registers.f.clear_sub_flag();

    //Clear Half Carry
    cpu.registers.f.clear_half_carry_flag();

    //Clear Carry Flag
    cpu.registers.f.clear_carry_flag();

    //Set actual accumualtor equal to resulting value
    cpu.registers.a = a;
}

pub fn cp_a_r(cpu: &mut Cpu, operand: u8) {
    let a: u8 = cpu.registers.a;

    let res = a.wrapping_sub(operand);

    //Update Zero Flag
    cpu.registers.f.update_zero_flag(res);

    //Set Sub Flag
    cpu.registers.f.set_sub_flag();

    //Update Half Carry
    cpu.registers.f.update_half_carry_flag_sub_8bit(a, operand);

    //Update Carry(Borrow) Flag
    cpu.registers.f.update_carry_flag_sub_8bit(a, operand);
}

pub fn daa(cpu: &mut Cpu) {
    if cpu.registers.f.sub_flag() == 0 {
        if cpu.registers.f.carry_flag() == 1 || cpu.registers.a > 0x99 {
            cpu.registers.a = cpu.registers.a.wrapping_add(0x60);
            cpu.registers.f.set_carry_flag();
        }

        if cpu.registers.f.half_carry_flag() == 1 || (cpu.registers.a & 0x0F) > 0x09 {
            cpu.registers.a = cpu.registers.a.wrapping_add(0x6);
        }
    } else {
        if cpu.registers.f.carry_flag() == 1 {
            cpu.registers.a = cpu.registers.a.wrapping_sub(0x60);
        }

        if cpu.registers.f.half_carry_flag() == 1 {
            cpu.registers.a = cpu.registers.a.wrapping_sub(0x6);
        }
    }

    //Clear Half Carry
    cpu.registers.f.clear_half_carry_flag();

    //Update Zero Flag
    cpu.registers.f.update_zero_flag(cpu.registers.a);
}

/************************************************************************
 * 8-bit Rotate instructions
 * *********************************************************************/

///Rotate Left Circular Accumulator
///
/// 7th bit of Accumulator is copied into carry and into the 0th bit of A
pub fn rlca(cpu: &mut Cpu) {
    let lmb: u8 = (cpu.registers.a & 0x80) >> 7;

    //Rotate Accumulator to left
    cpu.registers.a <<= 1;

    //Store previous 7th bit in 0th position
    cpu.registers.a |= (1 << 0) & lmb;

    //Store original 7th bit in carry
    if lmb == 0 {
        cpu.registers.f.clear_carry_flag();
    } else {
        cpu.registers.f.set_carry_flag();
    }

    //Clear Zero Flag
    cpu.registers.f.clear_zero_flag();

    //Clear Sub Flag
    cpu.registers.f.clear_sub_flag();

    //Clear Half Carry
    cpu.registers.f.clear_half_carry_flag();
}

///Rotate Right Circular Accumulator
///
/// 0th Bit of Accumulator is copied into the carry and into 7th bit of Accumulator
pub fn rrca(cpu: &mut Cpu) {
    let rmb: u8 = cpu.registers.a & 0x01;

    //Rotate Accumulator to right
    cpu.registers.a >>= 1;

    //Store previous 0th bit in 7th bit of A
    cpu.registers.a |= (1 << 7) & (rmb << 7);

    //Store original 0th bit in carry
    if rmb == 0 {
        cpu.registers.f.clear_carry_flag();
    } else {
        cpu.registers.f.set_carry_flag();
    }

    //Clear Zero Flag
    cpu.registers.f.clear_zero_flag();

    //Clear Sub Flag
    cpu.registers.f.clear_sub_flag();

    //Clear Half Carry
    cpu.registers.f.clear_half_carry_flag();
}

/// Rotate Left Accumulator
///
/// 7th bit is moved into carry, and the carry is moved into the 0th bit
pub fn rla(cpu: &mut Cpu) {
    let lmb: u8 = cpu.registers.a & 0x80;

    //Rotate Accumulator Left
    cpu.registers.a <<= 1;

    //Store carry into 0th bit of Accumulator
    cpu.registers.a |= (1 << 0) & (cpu.registers.f.carry_flag());

    //Move 7th bit into carry

    if lmb == 0 {
        cpu.registers.f.clear_carry_flag();
    } else {
        cpu.registers.f.set_carry_flag();
    }

    //Clear Zero Flag
    cpu.registers.f.clear_zero_flag();

    //Clear Sub Flag
    cpu.registers.f.clear_sub_flag();

    //Clear Half Carry
    cpu.registers.f.clear_half_carry_flag();
}

/// Rotate Right Accumulator
///
/// 0th bit of A is moved into the carry, and the carry is moved into the 7th bit of A
pub fn rra(cpu: &mut Cpu) {
    let rmb: u8 = cpu.registers.a & 0x01;

    //Rotate Accumulator to right
    cpu.registers.a >>= 1;

    //Store carry in 7th bit of A
    cpu.registers.a |= (1 << 7) & (cpu.registers.f.carry_flag() << 7);

    //Store original 0th bit in carry
    if rmb == 0 {
        cpu.registers.f.clear_carry_flag();
    } else {
        cpu.registers.f.set_carry_flag();
    }

    //Clear Zero Flag
    cpu.registers.f.clear_zero_flag();

    //Clear Sub Flag
    cpu.registers.f.clear_sub_flag();

    //Clear Half Carry
    cpu.registers.f.clear_half_carry_flag();
}

/// Rotate Left Register
///
/// Contents of registert R are rotated to left 1 bit position.
///
/// Contents of 7th bit are into carry and also to 0th bit
pub fn rlc(f: &mut Flags, r: &mut u8) {
    //Register
    let mut reg: u8 = *r;

    //Content of 7th bit
    let lmb: u8 = (reg & 0x80) >> 7;

    //Rotate register left
    reg <<= 1;

    //Store 7th bit into 0th bit of register
    reg |= (1 << 0) & (lmb);

    //Move 7th bit into carry
    if lmb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    //Update Zero Flag
    f.update_zero_flag(reg);

    //Clear Sub Flag
    f.clear_sub_flag();

    //Clear Half Carry
    f.clear_half_carry_flag();

    *r = reg;
}

/// Rotate Left Regsiter (mem[HL])
///
/// Contents of mem[HL] are rotated to left 1 bit position.
///
/// Contents of 7th bit are into carry and also to 0th bit
pub fn rlc_hl(f: &mut Flags, interconnect: &mut Interconnect, addr: u16) {
    //Value in memory at addr
    let mut value = interconnect.read_mem(addr);
    interconnect.emu_cycles(1);
    //Conent of 7th bit
    let lmb: u8 = (value & 0x80) >> 7;

    //Rotate mem[HL] left
    value <<= 1;

    //Store 7th bit in 0th bit of mem[HL]
    value |= (1 << 0) & (lmb);

    //Move 7th bit into carry
    if lmb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    //Update Zero Flag
    f.update_zero_flag(value);

    //Clear Sub Flag
    f.clear_sub_flag();

    //Clear Half Carry
    f.clear_half_carry_flag();

    //Write new value into memroy
    interconnect.write_mem(addr, value);
    interconnect.emu_cycles(1);
}

/// Rotate Register Right
///
/// Register is rotated to the right 1 bit position.
///
/// Contents of Bit 0 are copied to Carry Flag and also to bit 7.
pub fn rrc(f: &mut Flags, r: &mut u8) {
    //Register
    let mut reg: u8 = *r;

    //Content of 0th bit
    let rmb: u8 = reg & 0x01;

    //Rotate register right
    reg >>= 1;

    //Store 0th bit into 7th bit of register
    reg |= (1 << 7) & (rmb << 7);

    if rmb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    //Update Zero Flag
    f.update_zero_flag(reg);

    //Clear Sub Flag
    f.clear_sub_flag();

    //Clear Half Carry
    f.clear_half_carry_flag();

    *r = reg;
}

/// Rotate Register Right (mem[HL])
///
/// mem[HL] is rotated to the right 1 bit position.
///
/// Contents of Bit 0 are copied to Carry Flag and also to bit 7.
pub fn rrc_hl(f: &mut Flags, interconnect: &mut Interconnect, addr: u16) {
    let mut value: u8 = interconnect.read_mem(addr);
    interconnect.emu_cycles(1);

    //Content of 0th bit
    let rmb: u8 = value & 0x01;

    //Rotate mem[HL] right
    value >>= 1;

    value |= (1 << 7) & (rmb << 7);

    if rmb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    //Update Zero Flag
    f.update_zero_flag(value);

    //Clear Sub Flag
    f.clear_sub_flag();

    //Clear Half Carry
    f.clear_half_carry_flag();

    //Write new value into memory
    interconnect.write_mem(addr, value);
    interconnect.emu_cycles(1);
}

/// Rotate Left
///
/// Contents of operand are roateted left 1 bit position
///
/// Contents of bit 7 are copied to Carry Flag. Previous contents of Carry Flag are copied to bit 0
pub fn rl(f: &mut Flags, r: &mut u8) {
    //Register
    let mut reg: u8 = *r;

    //Contents of 7th bit
    let lmb: u8 = (reg & 0x80) >> 7;

    //Rotate Regiter Left
    reg <<= 1;

    //Copy carry to 0th bit
    reg |= (1 << 0) & f.carry_flag();

    if lmb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    //Update Zero Flag
    f.update_zero_flag(reg);

    //Clear Sub Flag
    f.clear_sub_flag();

    //Clear Half Carry
    f.clear_half_carry_flag();

    *r = reg;
}

/// Rotate Left
///
/// Contents of mem[HL] are roateted left 1 bit position
///
/// Contents of bit 7 are copied to Carry Flag. Previous contents of Carry Flag are copied to bit 0
pub fn rl_hl(f: &mut Flags, interconnect: &mut Interconnect, addr: u16) {
    let mut value: u8 = interconnect.read_mem(addr);
    interconnect.emu_cycles(1);

    let lmb: u8 = (value & 0x80) >> 7;

    //Rotate Left
    value <<= 1;

    //Copy carry to 0th bit
    value |= (1 << 0) & f.carry_flag();

    if lmb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    //Update Zero Flag
    f.update_zero_flag(value);

    //Clear Sub Flag
    f.clear_sub_flag();

    //Clear Half Carry
    f.clear_half_carry_flag();

    //Write new value into memory
    interconnect.write_mem(addr, value);
    interconnect.emu_cycles(1);
}

/// Rotate Right
///
/// Contents of mem[HL] are rotated right 1 bit position through Carry Flag
///
/// Conents of bit 0 are copied to carry flag and previoius contents of carry flag
/// are copied to bit 7
pub fn rr(f: &mut Flags, r: &mut u8) {
    let mut value: u8 = *r;

    let rmb: u8 = value & 0x01;

    //Rotate Right
    value >>= 1;

    //Copy carry to 0th bit
    value |= (1 << 7) & (f.carry_flag() << 7);

    if rmb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    //Update Zero Flag
    f.update_zero_flag(value);

    //Clear Sub Flag
    f.clear_sub_flag();

    //Clear Half Carry
    f.clear_half_carry_flag();

    *r = value;
}

pub fn rr_hl(f: &mut Flags, interconnect: &mut Interconnect, addr: u16) {
    let mut value: u8 = interconnect.read_mem(addr);
    interconnect.emu_cycles(1);

    let rmb: u8 = value & 0x01;

    //Rotate Right
    value >>= 1;

    //Copy carry to 0th bit
    value |= (1 << 7) & (f.carry_flag() << 7);

    if rmb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    //Update Zero Flag
    f.update_zero_flag(value);

    //Clear Sub Flag
    f.clear_sub_flag();

    //Clear Half Carry
    f.clear_half_carry_flag();

    //Write new value to memory
    interconnect.write_mem(addr, value);
    interconnect.emu_cycles(1);
}

/// Shift Left Arithmetic
///
/// An arithmetic shift left 1 bit position is performed on contents of register
///
/// Contents of bit 7 are copied ot carry flag
pub fn sla(f: &mut Flags, r: &mut u8) {
    let mut value: u8 = *r;

    //7th bit
    let lmb: u8 = (value & 0x80) >> 7;

    //shift left one bit position
    value <<= 1;

    if lmb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    //Update Zero Flag
    f.update_zero_flag(value);

    //Clear Sub Flag
    f.clear_sub_flag();

    //Clear Half Carry
    f.clear_half_carry_flag();

    *r = value;
}

/// Shift Left Arithmetic
///
/// An arithmetic shift left 1 bit position is performed on contents of mem[HL]
///
/// Contents of bit 7 are copied ot carry flag
pub fn sla_hl(f: &mut Flags, interconnect: &mut Interconnect, addr: u16) {
    let mut value: u8 = interconnect.read_mem(addr);
    interconnect.emu_cycles(1);

    //7th bit
    let lmb: u8 = (value & 0x80) >> 7;

    //shift left one bit position
    value <<= 1;

    if lmb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    //Update Zero Flag
    f.update_zero_flag(value);

    //Clear Sub Flag
    f.clear_sub_flag();

    //Clear Half Carry
    f.clear_half_carry_flag();

    //Write new value into memory
    interconnect.write_mem(addr, value);
    interconnect.emu_cycles(1);
}

///Shift Right Arithmetic
pub fn sra(f: &mut Flags, r: &mut u8) {
    let mut value: u8 = *r;

    //7th bit
    let lmb: u8 = value & 0x80;

    //0th bit
    let rmb: u8 = value & 0x01;

    //shift right one bit position
    value >>= 1;

    //put 7th bit back in
    value |= (1 << 7) & (lmb);

    if rmb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    //Update Zero Flag
    f.update_zero_flag(value);

    //Clear Sub Flag
    f.clear_sub_flag();

    //Clear Half Carry
    f.clear_half_carry_flag();

    *r = value;
}

//Shift Right Arithmetic
pub fn sra_hl(f: &mut Flags, interconnect: &mut Interconnect, addr: u16) {
    let mut value: u8 = interconnect.read_mem(addr);
    interconnect.emu_cycles(1);

    //7th bit
    let lmb: u8 = value & 0x80;

    //0th bit
    let rmb: u8 = value & 0x01;

    //shift right one bit position
    value >>= 1;

    //put 7th bit back in
    value |= (1 << 7) & (lmb);

    if rmb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    //Update Zero Flag
    f.update_zero_flag(value);

    //Clear Sub Flag
    f.clear_sub_flag();

    //Clear Half Carry
    f.clear_half_carry_flag();

    interconnect.write_mem(addr, value);
    interconnect.emu_cycles(1);
}

///Swap r
///
/// Exchange lower and higher nibbles
pub fn swap(f: &mut Flags, r: &mut u8) {
    let mut value: u8 = *r;

    //Lower Nibble
    let low: u8 = value & 0x0F;
    //Upper Nibble
    let up: u8 = value & 0xF0;

    //Swap
    value = (((low as u16) << 4) | ((up as u16) >> 4)) as u8;

    f.update_zero_flag(value);

    f.clear_carry_flag();

    f.clear_sub_flag();

    f.clear_half_carry_flag();

    *r = value;
}

///Swap mem[HL]
///
/// Exchange lower and higher nibbles
pub fn swap_hl(f: &mut Flags, interconnect: &mut Interconnect, addr: u16) {
    let mut value = interconnect.read_mem(addr);
    interconnect.emu_cycles(1);

    //Lower Nibble
    let low: u8 = value & 0x0F;

    //Upper Nibble
    let up: u8 = value & 0xF0;

    //Swap
    value = (((low as u16) << 4) | ((up as u16) >> 4)) as u8;

    f.update_zero_flag(value);

    f.clear_carry_flag();

    f.clear_sub_flag();

    f.clear_half_carry_flag();

    interconnect.write_mem(addr, value);
    interconnect.emu_cycles(1);
}

/// Shift Right Logical
///
/// Performs right shift on operand. 0th bit is copied to carry
///
/// 7th bit is cleared
pub fn srl(f: &mut Flags, r: &mut u8) {
    //register
    let mut value: u8 = *r;

    //0th bit
    let rmb: u8 = value & 0x01;

    //Perform shift
    value >>= 1;

    //Clear 7th bit
    value &= !(1 << 7);

    //Copy 0th bit into carry
    if rmb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    f.update_zero_flag(value);

    f.clear_sub_flag();

    f.clear_half_carry_flag();

    *r = value;
}

///  Shift Right Logical
///
/// Performs right shift on operand. 0th bit is copied to carry
///
/// 7th bit is cleared
pub fn srl_hl(f: &mut Flags, interconnect: &mut Interconnect, addr: u16) {
    let mut value: u8 = interconnect.read_mem(addr);
    interconnect.emu_cycles(1);

    //0th bit
    let rmb: u8 = value & 0x01;

    //Perform shift
    value >>= 1;

    //Clear 7th bit
    value &= !(1 << 7);

    //Copy 0th bit into carry
    if rmb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    f.update_zero_flag(value);

    f.clear_sub_flag();

    f.clear_half_carry_flag();

    //Write new value to memory
    interconnect.write_mem(addr, value);
    interconnect.emu_cycles(1);
}
/************************************************************************
 * 16-bit Arithmetic instructions
 * *********************************************************************/

///
/// Increment register pair

pub fn inc_16bit(cpu: &mut Cpu, register: &str) {
    match register {
        "BC" => {
            cpu.registers.set_bc(cpu.registers.bc().wrapping_add(1));
        }

        "DE" => {
            cpu.registers.set_de(cpu.registers.de().wrapping_add(1));
        }

        "HL" => {
            cpu.registers.set_hl(cpu.registers.hl().wrapping_add(1));
        }

        "SP" => {
            cpu.sp = cpu.sp.wrapping_add(1);
        }
        _ => println!("{}, Not a register PAIR", register),
    }
}

///Decrement Register Pair
pub fn dec_16bit(cpu: &mut Cpu, register: &str) {
    match register {
        "BC" => {
            cpu.registers.set_bc(cpu.registers.bc().wrapping_sub(1));
        }
        "DE" => {
            cpu.registers.set_de(cpu.registers.de().wrapping_sub(1));
        }
        "HL" => {
            cpu.registers.set_hl(cpu.registers.hl().wrapping_sub(1));
        }
        "SP" => {
            cpu.sp = cpu.sp.wrapping_sub(1);
        }
        _ => println!("{}, Not a register PAIR", register),
    }
}

pub fn add_rr_hl(cpu: &mut Cpu, register: &str) {
    match register {
        "BC" => {
            let a = cpu.registers.hl() as u32;
            let b = cpu.registers.bc() as u32;

            //HL + BC
            let c = a + b;

            //Clear Sub Flag
            cpu.registers.f.clear_sub_flag();

            //Update Half Carry
            cpu.registers.f.update_half_carry_flag_sum_16bit(a, b);

            //Calculate Carry
            cpu.registers
                .f
                .update_carry_flag_sum_16bit(cpu.registers.hl(), cpu.registers.bc());

            cpu.registers.set_hl(c as u16);
        }
        "DE" => {
            let a = cpu.registers.hl() as u32;
            let b = cpu.registers.de() as u32;

            //HL + DE
            let c = a + b;

            //Clear Sub Flag
            cpu.registers.f.clear_sub_flag();

            //Update Half Carry
            cpu.registers.f.update_half_carry_flag_sum_16bit(a, b);

            //Calculate Carry
            cpu.registers
                .f
                .update_carry_flag_sum_16bit(cpu.registers.hl(), cpu.registers.de());

            cpu.registers.set_hl(c as u16);
        }
        "HL" => {
            let a = cpu.registers.hl() as u32;
            let b = cpu.registers.hl() as u32;

            //HL + HL
            let c = a + b;

            //Clear Sub Flag
            cpu.registers.f.clear_sub_flag();

            //Update Half Carry;
            cpu.registers.f.update_half_carry_flag_sum_16bit(a, b);

            //Calculate Carry
            cpu.registers
                .f
                .update_carry_flag_sum_16bit(cpu.registers.hl(), cpu.registers.hl());

            cpu.registers.set_hl(c as u16);
        }

        "SP" => {
            let a = cpu.registers.hl() as u32;
            let b = cpu.sp as u32;

            //HL + HL
            let c = a + b;

            //Clear Sub Flag
            cpu.registers.f.clear_sub_flag();

            //Update Half Carry;
            cpu.registers.f.update_half_carry_flag_sum_16bit(a, b);

            //Calculate Carry
            cpu.registers
                .f
                .update_carry_flag_sum_16bit(cpu.registers.hl(), cpu.sp);

            cpu.registers.set_hl(c as u16);
        }
        _ => println!("{}, Not a register PAIR", register),
    }
}

/************************************************************************
 * Jump Instructions
 * *********************************************************************/
///
/// Relative Jump
/// PC = PC + 8bit signed
pub fn jr(cpu: &mut Cpu, dd: u8) {
    let offset = dd as i8;

    cpu.pc = cpu.pc.wrapping_add(offset as u16).wrapping_add(2);
}

///
/// Relative Jump if Zero flag is set
pub fn jr_z(cpu: &mut Cpu, interconnect: &mut Interconnect, dd: u8) {
    if cpu.registers.f.zero_flag() == 1 {
        jr(cpu, dd);
        interconnect.emu_cycles(3);
    } else {
        cpu.pc += 2;
        interconnect.emu_cycles(2);
    }
}

///
/// Relative Jump if Zero flag is clear
pub fn jr_nz(cpu: &mut Cpu, interconnect: &mut Interconnect, dd: u8) {
    if cpu.registers.f.zero_flag() == 0 {
        jr(cpu, dd);
        interconnect.emu_cycles(3);
    } else {
        cpu.pc += 2;
        interconnect.emu_cycles(2);
    }
}

///
/// Relative Jump if Carry flag is Set
pub fn jr_c(cpu: &mut Cpu, interconnect: &mut Interconnect, dd: u8) {
    if cpu.registers.f.carry_flag() == 1 {
        jr(cpu, dd);
        interconnect.emu_cycles(3);
    } else {
        cpu.pc += 2;
        interconnect.emu_cycles(2);
    }
}

///
/// Relative Jump if Carry flag is clear
pub fn jr_nc(cpu: &mut Cpu, interconnect: &mut Interconnect, dd: u8) {
    if cpu.registers.f.carry_flag() == 0 {
        jr(cpu, dd);
        interconnect.emu_cycles(3);
    } else {
        cpu.pc += 2;
        interconnect.emu_cycles(2);
    }
}

///
/// Jump to nn
pub fn jp(cpu: &mut Cpu, nn: u16) {
    cpu.pc = nn;
}

///
/// Jump to nn if zero flag is set
pub fn jp_z(cpu: &mut Cpu, interconnect: &mut Interconnect, nn: u16) {
    if cpu.registers.f.zero_flag() == 1 {
        jp(cpu, nn);
        interconnect.emu_cycles(4);
    } else {
        cpu.pc += 3;
        interconnect.emu_cycles(3);
    }
}
///
/// Jump to nn if zero flag is clear
pub fn jp_nz(cpu: &mut Cpu, interconnect: &mut Interconnect, nn: u16) {
    if cpu.registers.f.zero_flag() == 0 {
        jp(cpu, nn);
        interconnect.emu_cycles(4);
    } else {
        cpu.pc += 3;
        interconnect.emu_cycles(3);
    }
}

///
/// Jump to nn if carry flag is set
pub fn jp_c(cpu: &mut Cpu, interconnect: &mut Interconnect, nn: u16) {
    if cpu.registers.f.carry_flag() == 1 {
        jp(cpu, nn);
        interconnect.emu_cycles(4);
    } else {
        cpu.pc += 3;
        interconnect.emu_cycles(3);
    }
}

///
/// Jump to nn if carry flag is clear
pub fn jp_nc(cpu: &mut Cpu, interconnect: &mut Interconnect, nn: u16) {
    if cpu.registers.f.carry_flag() == 0 {
        jp(cpu, nn);
        interconnect.emu_cycles(4);
    } else {
        cpu.pc += 3;
        interconnect.emu_cycles(3);
    }
}

///
/// Call to nn
pub fn call(cpu: &mut Cpu, interconnect: &mut Interconnect, nn: u16) {
    let mut stack_pointer: u16 = cpu.sp;

    //SP = SP - 2
    stack_pointer -= 2;

    //Increment PC by 3 before push
    cpu.pc += 3;

    //mem[sp] = pc
    interconnect.write_mem(stack_pointer, (cpu.pc & 0x00FF) as u8);
    interconnect.write_mem(stack_pointer + 1, ((cpu.pc & 0xFF00) >> 8) as u8);

    //PC = nn
    cpu.pc = nn;

    cpu.sp = stack_pointer;
}

///
/// Call to nn if zero flag is set
pub fn call_z(cpu: &mut Cpu, interconnect: &mut Interconnect, nn: u16) {
    if cpu.registers.f.zero_flag() == 1 {
        call(cpu, interconnect, nn);
        interconnect.emu_cycles(6);
    } else {
        cpu.pc += 3;
        interconnect.emu_cycles(3);
    }
}

/// Call to nn if zero flag is clear
pub fn call_nz(cpu: &mut Cpu, interconnect: &mut Interconnect, nn: u16) {
    if cpu.registers.f.zero_flag() == 0 {
        call(cpu, interconnect, nn);
        interconnect.emu_cycles(6);
    } else {
        cpu.pc += 3;
        interconnect.emu_cycles(3);
    }
}

/// Call to nn if carry flag is set
pub fn call_c(cpu: &mut Cpu, interconnect: &mut Interconnect, nn: u16) {
    if cpu.registers.f.carry_flag() == 1 {
        call(cpu, interconnect, nn);
        interconnect.emu_cycles(6);
    } else {
        cpu.pc += 3;
        interconnect.emu_cycles(3);
    }
}

/// Call to nn if carry flag is clear
pub fn call_nc(cpu: &mut Cpu, interconnect: &mut Interconnect, nn: u16) {
    if cpu.registers.f.carry_flag() == 0 {
        call(cpu, interconnect, nn);
        interconnect.emu_cycles(6);
    } else {
        cpu.pc += 3;
        interconnect.emu_cycles(3);
    }
}

///
/// Call to 00, 08, 10, 18, 20, 28, 30, 38(hex)
pub fn rst(cpu: &mut Cpu, interconnect: &mut Interconnect, n: u8) {
    let mut stack_pointer: u16 = cpu.sp;

    //SP = SP - 2
    stack_pointer = stack_pointer.wrapping_sub(2);

    //Increment PC before push
    cpu.pc += 1;

    //mem[SP] = lower byte of program counter
    interconnect.write_mem(stack_pointer, (cpu.pc & 0x00FF) as u8);

    //mem[SP+1] = upper byte of program counter (its + 1 below because we already moved the stack pointer)
    interconnect.write_mem(stack_pointer + 1, ((cpu.pc & 0xFF00) >> 8) as u8);

    //PC = n
    cpu.pc = n as u16;

    cpu.sp = stack_pointer;
}

///Return
pub fn ret(cpu: &mut Cpu, interconnect: &Interconnect) {
    let mut sp = cpu.sp;

    //PC = (SP)
    let pc = u16::from_be_bytes([
        interconnect.read_mem(sp.wrapping_add(1)),
        interconnect.read_mem(sp),
    ]);

    cpu.pc = pc;

    //SP = SP + 2
    sp = sp.wrapping_add(2);

    cpu.sp = sp;
}

pub fn ret_z(cpu: &mut Cpu, interconnect: &mut Interconnect) {
    if cpu.registers.f.zero_flag() == 1 {
        ret(cpu, interconnect);
        interconnect.emu_cycles(5);
    } else {
        cpu.pc += 1;
        interconnect.emu_cycles(2);
    }
}
pub fn ret_nz(cpu: &mut Cpu, interconnect: &mut Interconnect) {
    if cpu.registers.f.zero_flag() == 0 {
        ret(cpu, interconnect);
        interconnect.emu_cycles(5);
    } else {
        cpu.pc += 1;
        interconnect.emu_cycles(2);
    }
}

pub fn ret_c(cpu: &mut Cpu, interconnect: &mut Interconnect) {
    if cpu.registers.f.carry_flag() == 1 {
        ret(cpu, interconnect);
        interconnect.emu_cycles(5);
    } else {
        cpu.pc += 1;
        interconnect.emu_cycles(2);
    }
}

pub fn ret_nc(cpu: &mut Cpu, interconnect: &mut Interconnect) {
    if cpu.registers.f.carry_flag() == 0 {
        ret(cpu, interconnect);
        interconnect.emu_cycles(5);
    } else {
        cpu.pc += 1;

        interconnect.emu_cycles(2);
    }
}

/************************************************************************
 * 8-bit LOAD instructions
 * *********************************************************************/

///
/// Load 8 bit value into specific register
pub fn ld_8bit(r: &mut u8, data: u8) {
    //Rd = Rr
    *r = data;
}

///
/// Load data from io-port 'n' into A register
pub fn ld_a_from_io(cpu: &mut Cpu, interconnect: &Interconnect, n: u8) {
    let addr: u16 = 0xFF00 + (n as u16);
    cpu.registers.a = interconnect.read_mem(addr);
}

///
/// Load data from A register into io-port 'n'
pub fn ld_io_from_a(cpu: &Cpu, interconnect: &mut Interconnect, n: u8) {
    let addr: u16 = 0xFF00 + (n as u16);
    interconnect.write_mem(addr, cpu.registers.a);
}

///
/// Load data from ($FF00 + register C) into A register
pub fn ld_a_from_io_c(cpu: &mut Cpu, interconnect: &Interconnect) {
    let addr: u16 = 0xFF00 + (cpu.registers.c as u16);
    cpu.registers.a = interconnect.read_mem(addr);
}

///
/// Load data from register A into ($FF00 + register C)
pub fn ld_io_c_from_a(cpu: &Cpu, interconnect: &mut Interconnect) {
    let addr: u16 = 0xFF00 + (cpu.registers.c as u16);
    interconnect.write_mem(addr, cpu.registers.a);
}

/************************************************************************
 * 16-bit LOAD instructions
 * *********************************************************************/
///
/// Contents of Register Pair are popped off stack
pub fn pop_rr(interconnect: &Interconnect, upper: &mut u8, lower: &mut u8, sp: &mut u16) {
    //Stack Pointer
    let mut stack_pointer = *sp;

    //Value in memory (mem[sp])
    let low: u8 = interconnect.read_mem(stack_pointer);
    let up: u8 = interconnect.read_mem(stack_pointer.wrapping_add(1));

    //rr = mem[sp]
    *lower = low;
    *upper = up;

    //SP = SP + 2
    stack_pointer = stack_pointer.wrapping_add(2);

    *sp = stack_pointer;
}

///
/// Contents of Register Pair are pushed onto stack
pub fn push_rr(interconnect: &mut Interconnect, upper: u8, lower: u8, sp: &mut u16) {
    //Stack Pointer
    let mut stack_pointer = *sp;

    //SP = SP - 2
    stack_pointer = stack_pointer.wrapping_sub(2);

    //mem[sp] = rr
    interconnect.write_mem(stack_pointer, lower);
    interconnect.write_mem(stack_pointer + 1, upper);

    *sp = stack_pointer;
}

/************************************************************************
 * Single-bit Operation instructions
 * *********************************************************************/

///
/// Checks the nth bit of r and stores the inverse in the zero flag.
pub fn bit_n_r(f: &mut Flags, r: &mut u8, n: u8) {
    let reg: u8 = *r;

    let res = (reg >> n) & 0x01;

    //Update Zero Flag
    if res == 0 {
        f.set_zero_flag();
    } else {
        f.clear_zero_flag();
    }

    //Clear Sub Flag
    f.clear_sub_flag();

    //Set Half CArry
    f.set_half_carry_flag();
}

///
///
/// Checks the nth bit of mem[hl] and stores the inverse in the zero flag.
pub fn bit_n_hl(f: &mut Flags, interconnect: &mut Interconnect, addr: u16, n: u8) {
    let mut value: u8 = interconnect.read_mem(addr);

    value = (value >> n) & 0x01;

    //Update Zero Flag
    if value == 0 {
        f.set_zero_flag();
    } else {
        f.clear_zero_flag();
    }

    //Clear Sub Flag
    f.clear_sub_flag();

    //Set Half Carry
    f.set_half_carry_flag();
}

///
/// Sets the nth bit of r.
pub fn set_n_r(r: &mut u8, n: u8) {
    let mut reg: u8 = *r;

    //Set the nth bit
    reg |= 1 << n;

    *r = reg;
}

pub fn set_n_hl(interconnect: &mut Interconnect, addr: u16, n: u8) {
    let mut value: u8 = interconnect.read_mem(addr);
    interconnect.emu_cycles(1);

    //Set the nth bit
    value |= 1 << n;

    interconnect.write_mem(addr, value);
    interconnect.emu_cycles(1);
}

///
/// Clears the nth bit of r
pub fn res_n_r(r: &mut u8, n: u8) {
    let mut reg: u8 = *r;

    //Clear the nth bit
    reg &= !(1 << n);

    *r = reg;
}

pub fn res_n_hl(interconnect: &mut Interconnect, addr: u16, n: u8) {
    let mut value: u8 = interconnect.read_mem(addr);
    interconnect.emu_cycles(1);

    //Clear the nth bit
    value &= !(1 << n);

    interconnect.write_mem(addr, value);
    interconnect.emu_cycles(1);
}

/************************************************************************
 * Interrupt Instructions
 * *********************************************************************/
pub fn ei(cpu: &mut Cpu) {
    cpu.ime_to_be_enabled = true;
    //cpu.ime = true;
}

pub fn di(cpu: &mut Cpu) {
    cpu.ime = false;
}
