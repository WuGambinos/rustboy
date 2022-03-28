use super::*;

/************************************************************************
 * 8-bit Arithmetic instructions
 * *********************************************************************/

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

    *register = value;
}

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

    *register = value;
}

///Increment vlaue in memory using HL pointer
pub fn inc_mem(cpu: &mut Cpu, mmu: &mut Mmu) {
    //Grab value in memory
    let mut value = mmu.read_mem(cpu.registers.hl());

    //Check for Half Carry
    cpu.f.update_half_carry_flag_sum_8bit(value, 1);

    //Increment value
    value = value.wrapping_add(1);

    //Write new incremented value back into memory
    mmu.write_mem(cpu.registers.hl(), value);

    //Update Zero Flag
    cpu.f.update_zero_flag(value);

    //Clear Sub Flag
    cpu.f.sub_flag = 0;
}

///Decrement value in memory using HL pointer
pub fn dec_mem(cpu: &mut Cpu, mmu: &mut Mmu) {
    //Grab value in memory
    let mut value: u8 = mmu.read_mem(cpu.registers.hl());

    //Check for Half Carry
    cpu.f.update_half_carry_flag_sub_8bit(value, 1);

    //Decrement Value
    value = value.wrapping_sub(1);

    //Write new decremented value back into memory
    mmu.write_mem(cpu.registers.hl(), value);

    //Update Zero Flag
    cpu.f.update_zero_flag(value);

    //Set sub flag
    cpu.f.sub_flag = 1;
}

///Adds Accumulator(register A) and another register together, storing result in the accumulator
///
/// a = r + a
///
/// Flags: Z0HC
pub fn add_a_r(cpu: &mut Cpu, operand: u8) {
    let mut a: u8 = cpu.registers.a;
    //Update Half Carry
    cpu.f.update_half_carry_flag_sum_8bit(a, operand);

    //Update Carry Flag
    cpu.f.update_carry_flag_sum_8bit(a, operand);

    //a = r + a
    a = a.wrapping_add(operand);

    //Clear Sub Flag
    cpu.f.clear_sub_flag();

    //Update Zero Flag
    cpu.f.update_zero_flag(a);

    //Set Actual accumulator equal to resulting value
    cpu.registers.a = a;
}

///Adds Accumulator(register A), another register, and carry all together, storing result in the accumulator
///
/// a = a + r + c
pub fn adc_a_r(cpu: &mut Cpu, operand: u8) {
    let mut a: u8 = cpu.registers.a;

    //Need to sum operand and carry for the half carry to be calculated correctly
    let new_operand: u8 = operand.wrapping_add(cpu.f.carry_flag);

    //Update Half Carry
    cpu.f.update_half_carry_flag_sum_8bit(a, new_operand);

    //Update Carry Flag
    cpu.f.update_carry_flag_sum_8bit(a, new_operand);

    //a = r + a + c
    a = a.wrapping_add(new_operand);

    //Clear Sub Flag
    cpu.f.clear_sub_flag();

    //Update Zero Flag
    cpu.f.update_zero_flag(a);

    //Set actual accumulator equal to resulting value
    cpu.registers.a = a;
}

///Subtracts another register from the accumulator, storing the result in the accumulator
///
/// a = a - r
pub fn sub_r_r(cpu: &mut Cpu, operand: u8) {
    let mut a: u8 = cpu.registers.a;

    //Update Half Carry
    cpu.f.update_half_carry_flag_sub_8bit(a, operand);

    //Update Carry(Borrow) Flag
    cpu.f.update_carry_flag_sub_8bit(a, operand);

    //a = a - r
    a = a.wrapping_sub(operand);

    //Set Sub flag
    cpu.f.set_sub_flag();

    //Update Zero Flag
    cpu.f.update_zero_flag(a);

    //Set actual accumulator equal to resulting value
    cpu.registers.a = a;
}

pub fn sbc_r_r(cpu: &mut Cpu, operand: u8) {
    let mut a: u8 = cpu.registers.a;

    //Need to subtract operand and carry for the half carry to be calculated correctly
    a = a.wrapping_sub(operand);

    //Update Half Carry
    cpu.f.update_half_carry_flag_sub_8bit(a, 1);

    //Update Carry(Borrow) Flag
    cpu.f.update_carry_flag_sub_8bit(cpu.registers.a, 1);

    //a = a - r - c
    a = a.wrapping_sub(1);

    //Set Sub Flag
    cpu.f.set_sub_flag();

    //Update Zero Flag
    cpu.f.update_zero_flag(a);

    //Set actual accumulator equal to resulting value
    cpu.registers.a = a;
}

/************************************************************************
 * 8-bit Rotate instructions
 * *********************************************************************/

///Rotate Left Circular Accumulator
///
/// 7th bit of Accumulator is copied into carry and into the 0th bit of A
pub fn rlca(cpu: &mut Cpu) {
    let lmb: u8 = cpu.registers.a & 0x80;

    //Rotate Accumulator to left
    cpu.registers.a <<= 1;

    //Store previous 7th bit in 0th position
    cpu.registers.a |= (1 << 0) & lmb;

    //Store original 7th bit in carry
    cpu.f.carry_flag = lmb;
}

///Rotate Right Circular Accumulator
///
/// 0th Bit of Accumulator is copied into the carry and into 7th bit of Accumulator
pub fn rrca(cpu: &mut Cpu) {
    let rmb: u8 = cpu.registers.a & 0x01;

    //Rotate Accumulator to right
    cpu.registers.a >>= 1;

    //Store previous 0th bit in 7th bit of A
    cpu.registers.a |= (1 << 7) & rmb;

    //Store original 0th bit in carry
    cpu.f.carry_flag = rmb;
}

/// Rotate Left Accumulator
///
/// 7th bit is moved into carry, and the carry is moved into the 0th bit
pub fn rla(cpu: &mut Cpu) {
    let lmb: u8 = cpu.registers.a & 0x80;

    //Rotate Accumulator Left
    cpu.registers.a <<= 1;

    //Store carry into 0th bit of Accumulator
    cpu.registers.a |= (1 << 0) & (cpu.f.carry_flag);

    //Move 7th bit into carry
    cpu.f.carry_flag = lmb;
}

/// Rotate Right Accumulator
///
/// 0th bit of A is moved into the carry, and the carry is moved into the 7th bit of A
pub fn rra(cpu: &mut Cpu) {
    let rmb: u8 = cpu.registers.a & 0x01;

    //Rotate Accumulator to right
    cpu.registers.a >>= 1;

    //Store carry in 7th bit of A
    cpu.registers.a |= (1 << 7) & cpu.f.carry_flag;

    //Store original 0th bit in carry
    cpu.f.carry_flag = rmb;
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
        _ => println!("Not a register PAIR"),
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
        _ => println!("NOT A REGISTER PAIR"),
    }
}

pub fn add_rr_hl(cpu: &mut Cpu, register: &str) {
    match register {
        "BC" => {
            cpu.f
                .update_half_carry_flag_sum_16bit(cpu.registers.hl(), cpu.registers.bc());
            cpu.registers
                .set_hl(cpu.registers.hl().wrapping_add(cpu.registers.bc()));
            cpu.f.zero_flag = (cpu.registers.hl() == 0) as u8;
            cpu.f.sub_flag = 0;
        }
        "DE" => {
            cpu.f
                .update_half_carry_flag_sum_16bit(cpu.registers.hl(), cpu.registers.de());
            cpu.registers
                .set_hl(cpu.registers.hl().wrapping_add(cpu.registers.de()));
            cpu.f.zero_flag = (cpu.registers.hl() == 0) as u8;
            cpu.f.sub_flag = 0;
        }
        "HL" => {
            cpu.f
                .update_half_carry_flag_sum_16bit(cpu.registers.hl(), cpu.registers.hl());
            cpu.registers
                .set_hl(cpu.registers.hl().wrapping_add(cpu.registers.hl()));
            cpu.f.zero_flag = (cpu.registers.hl() == 0) as u8;
            cpu.f.sub_flag = 0;
        }
        _ => println!("NOT A REGISTER PAIR"),
    }
}

///
/// Relative Jump
/// PC = PC + 8bit signed
pub fn jr(cpu: &mut Cpu, dd: u8) {
    let offset = dd as i8;

    cpu.pc += cpu.pc.wrapping_add(offset as u16);
}

///
/// Relative Jump if Zero flag is set
pub fn jr_z(cpu: &mut Cpu, dd: u8) {
    if cpu.f.zero_flag == 1 {
        jr(cpu, dd);
    } else {
        cpu.pc += 2;
    }
}

///
/// Relative Jump if Zero flag is clear
pub fn jr_nz(cpu: &mut Cpu, dd: u8) {
    if cpu.f.zero_flag == 0 {
        jr(cpu, dd);
    } else {
        cpu.pc += 2;
    }
}

///
/// Relative Jump if Carry flag is Set
pub fn jr_c(cpu: &mut Cpu, dd: u8) {
    if cpu.f.carry_flag == 1 {
        jr(cpu, dd);
    } else {
        cpu.pc += 2;
    }
}

///
/// Relative Jump if Carry flag is clear
pub fn jr_nc(cpu: &mut Cpu, dd: u8) {
    if cpu.f.carry_flag == 0 {
        jr(cpu, dd);
    } else {
        cpu.pc += 2;
    }
}

pub fn daa(cpu: &mut Cpu) {
    if (cpu.registers.a & 0x0F) > 0x09 || cpu.f.half_carry_flag == 1 {
        cpu.registers.a += 0x06;
    }

    let upper_nibble = cpu.registers.a & 0xF0 >> 4;
    let mut reached = false;

    if upper_nibble > 9 || cpu.f.carry_flag == 1 {
        cpu.registers.a += 0x60;
        reached = true;
    }

    //Set carry if second addition was needed, otherwise reset carry
    if reached {
        cpu.f.carry_flag = 1;
    } else {
        cpu.f.carry_flag = 0;
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
