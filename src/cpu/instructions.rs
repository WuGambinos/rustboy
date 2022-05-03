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
    cpu.registers.f.update_half_carry_flag_sum_8bit(value, 1);

    //Increment value
    value = value.wrapping_add(1);

    //Write new incremented value back into memory
    mmu.write_mem(cpu.registers.hl(), value);

    //Update Zero Flag
    cpu.registers.f.update_zero_flag(value);

    //Clear Sub Flag
    cpu.registers.f.clear_sub_flag();
}

///Decrement value in memory using HL pointer
pub fn dec_mem(cpu: &mut Cpu, mmu: &mut Mmu) {
    //Grab value in memory
    let mut value: u8 = mmu.read_mem(cpu.registers.hl());

    //Check for Half Carry
    cpu.registers.f.update_half_carry_flag_sub_8bit(value, 1);

    //Decrement Value
    value = value.wrapping_sub(1);

    //Write new decremented value back into memory
    mmu.write_mem(cpu.registers.hl(), value);

    //Update Zero Flag
    cpu.registers.f.update_zero_flag(value);

    //Set sub flag
    cpu.registers.f.set_sub_flag();
}

///Adds Accumulator(register A) and another register together, storing result in the accumulator
///
/// a = r + a
///
/// Flags: Z0HC
pub fn add_a_r(cpu: &mut Cpu, operand: u8) {
    let mut a: u8 = cpu.registers.a;
    //Update Half Carry
    cpu.registers.f.update_half_carry_flag_sum_8bit(a, operand);

    //Update Carry Flag
    cpu.registers.f.update_carry_flag_sum_8bit(a, operand);

    //a = r + a
    a = a.wrapping_add(operand);

    //Clear Sub Flag
    cpu.registers.f.clear_sub_flag();

    //Update Zero Flag
    cpu.registers.f.update_zero_flag(a);

    //Set Actual accumulator equal to resulting value
    cpu.registers.a = a;
}

///Adds Accumulator(register A), another register, and carry all together, storing result in the accumulator
///
/// a = a + r + c
pub fn adc_a_r(cpu: &mut Cpu, operand: u8) {
    let mut a: u8 = cpu.registers.a;

    //Need to sum operand and carry for the half carry to be calculated correctly
    let new_operand: u8 = operand.wrapping_add(cpu.registers.f.carry_flag());

    //Update Half Carry
    cpu.registers
        .f
        .update_half_carry_flag_sum_8bit(a, new_operand);

    //Update Carry Flag
    cpu.registers.f.update_carry_flag_sum_8bit(a, new_operand);

    //a = r + a + c
    a = a.wrapping_add(new_operand);

    //Clear Sub Flag
    cpu.registers.f.clear_sub_flag();

    //Update Zero Flag
    cpu.registers.f.update_zero_flag(a);

    //Set actual accumulator equal to resulting value
    cpu.registers.a = a;
}

///Subtracts another register from the accumulator, storing the result in the accumulator
///
/// a = a - r
pub fn sub_r_r(cpu: &mut Cpu, operand: u8) {
    let mut a: u8 = cpu.registers.a;

    //Update Half Carry
    cpu.registers.f.update_half_carry_flag_sub_8bit(a, operand);

    //Update Carry(Borrow) Flag
    cpu.registers.f.update_carry_flag_sub_8bit(a, operand);

    //a = a - r
    a = a.wrapping_sub(operand);

    //Set Sub flag
    cpu.registers.f.set_sub_flag();

    //Update Zero Flag
    cpu.registers.f.update_zero_flag(a);

    //Set actual accumulator equal to resulting value
    cpu.registers.a = a;
}

pub fn sbc_r_r(cpu: &mut Cpu, operand: u8) {
    let mut a: u8 = cpu.registers.a;

    //Need to subtract operand and carry for the half carry to be calculated correctly
    a = a.wrapping_sub(operand);

    //Update Half Carry
    cpu.registers.f.update_half_carry_flag_sub_8bit(a, 1);

    //Update Carry(Borrow) Flag
    cpu.registers
        .f
        .update_carry_flag_sub_8bit(cpu.registers.a, 1);

    //a = a - r - c
    a = a.wrapping_sub(1);

    //Set Sub Flag
    cpu.registers.f.set_sub_flag();

    //Update Zero Flag
    cpu.registers.f.update_zero_flag(a);

    //Set actual accumulator equal to resulting value
    cpu.registers.a = a;
}

pub fn and_r_r(cpu: &mut Cpu, operand: u8) {
    let mut a: u8 = cpu.registers.a;

    //and
    a = a & operand;

    //Update Zero Flag
    cpu.registers.f.update_zero_flag(a);

    //Clear Sub Flag
    cpu.registers.f.clear_sub_flag();

    //Set Half Carry
    cpu.registers.f.set_half_carry_flag();

    //Clear Carry Flag
    cpu.registers.f.clear_carry_flag();

    //Set actual accumualtor equal to resulting value
    cpu.registers.a = a;
}

pub fn xor_r_r(cpu: &mut Cpu, operand: u8) {
    let mut a: u8 = cpu.registers.a;

    //xor
    a = a ^ operand;

    //Update Zero Flag
    cpu.registers.f.update_zero_flag(a);

    //Clear Sub Flag
    cpu.registers.f.clear_sub_flag();

    //Clear Half Carry
    cpu.registers.f.clear_half_carry_flag();

    //Clear Carry
    cpu.registers.f.clear_carry_flag();

    //Set actual accumualtor equal to resulting value
    cpu.registers.a = a;
}

pub fn or_r_r(cpu: &mut Cpu, operand: u8) {
    let mut a: u8 = cpu.registers.a;

    //or
    a = a | operand;

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

pub fn cp_r_r(cpu: &mut Cpu, operand: u8) {
    let a: u8 = cpu.registers.a;

    let res = a - operand;

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
    if (cpu.registers.a & 0x0F) > 0x09 || cpu.registers.f.half_carry_flag() == 1 {
        cpu.registers.a += 0x06;
    }

    let upper_nibble = cpu.registers.a & 0xF0 >> 4;
    let mut reached = false;

    if upper_nibble > 9 || cpu.registers.f.carry_flag() == 1 {
        cpu.registers.a += 0x60;
        reached = true;
    }

    //Set carry if second addition was needed, otherwise reset carry
    if reached {
        cpu.registers.f.set_carry_flag();
    } else {
        cpu.registers.f.clear_carry_flag();
    }
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
    *r = reg;
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
    *r = reg;
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
            cpu.registers
                .f
                .update_half_carry_flag_sum_16bit(cpu.registers.hl(), cpu.registers.bc());
            cpu.registers
                .set_hl(cpu.registers.hl().wrapping_add(cpu.registers.bc()));

            if cpu.registers.hl() == 0 {
                cpu.registers.f.set_zero_flag();
            } else {
                cpu.registers.f.clear_zero_flag();
            }
            cpu.registers.f.clear_sub_flag();
        }
        "DE" => {
            cpu.registers
                .f
                .update_half_carry_flag_sum_16bit(cpu.registers.hl(), cpu.registers.de());
            cpu.registers
                .set_hl(cpu.registers.hl().wrapping_add(cpu.registers.de()));

            if cpu.registers.hl() == 0 {
                cpu.registers.f.set_zero_flag();
            } else {
                cpu.registers.f.clear_zero_flag();
            }
            cpu.registers.f.clear_sub_flag();
        }
        "HL" => {
            cpu.registers
                .f
                .update_half_carry_flag_sum_16bit(cpu.registers.hl(), cpu.registers.hl());
            cpu.registers
                .set_hl(cpu.registers.hl().wrapping_add(cpu.registers.hl()));

            if cpu.registers.hl() == 0 {
                cpu.registers.f.set_zero_flag();
            } else {
                cpu.registers.f.clear_zero_flag();
            }
            cpu.registers.f.clear_sub_flag();
        }
        _ => println!("NOT A REGISTER PAIR"),
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

    cpu.pc = cpu.pc.wrapping_add(offset as u16);
}

///
/// Relative Jump if Zero flag is set
pub fn jr_z(cpu: &mut Cpu, dd: u8) {
    if cpu.registers.f.zero_flag() == 1 {
        jr(cpu, dd);
    } else {
        cpu.pc += 2;
    }
}

///
/// Relative Jump if Zero flag is clear
pub fn jr_nz(cpu: &mut Cpu, dd: u8) {
    if cpu.registers.f.zero_flag() == 0 {
        jr(cpu, dd);
    } else {
        cpu.pc += 2;
    }
}

///
/// Relative Jump if Carry flag is Set
pub fn jr_c(cpu: &mut Cpu, dd: u8) {
    if cpu.registers.f.carry_flag() == 1 {
        jr(cpu, dd);
    } else {
        cpu.pc += 2;
    }
}

///
/// Relative Jump if Carry flag is clear
pub fn jr_nc(cpu: &mut Cpu, dd: u8) {
    if cpu.registers.f.carry_flag() == 0 {
        jr(cpu, dd);
    } else {
        cpu.pc += 2;
    }
}

///
/// Jump to nn
pub fn jp(cpu: &mut Cpu, nn: u16) {
    cpu.pc = nn;
}

///
/// Jump to nn if zero flag is set
pub fn jp_z(cpu: &mut Cpu, nn: u16) {
    if cpu.registers.f.zero_flag() == 1 {
        jp(cpu, nn);
    } else {
        cpu.pc += 3;
    }
}
///
/// Jump to nn if zero flag is clear
pub fn jp_nz(cpu: &mut Cpu, nn: u16) {
    if cpu.registers.f.zero_flag() == 0 {
        jp(cpu, nn);
    } else {
        cpu.pc += 3;
    }
}

///
/// Jump to nn if carry flag is set
pub fn jp_c(cpu: &mut Cpu, nn: u16) {
    if cpu.registers.f.carry_flag() == 1 {
        jp(cpu, nn);
    } else {
        cpu.pc += 3;
    }
}

///
/// Jump to nn if carry flag is clear
pub fn jp_nc(cpu: &mut Cpu, nn: u16) {
    if cpu.registers.f.carry_flag() == 0 {
        jp(cpu, nn);
    } else {
        cpu.pc += 3;
    }
}

///
/// Call to nn
pub fn call(cpu: &mut Cpu, mmu: &mut Mmu, nn: u16) {
    let mut stack_pointer = cpu.sp;

    //SP = SP - 2
    stack_pointer = stack_pointer - 2;

    //mem[sp] = pc
    mmu.write_mem(stack_pointer, (cpu.pc & 0x00FF) as u8);
    mmu.write_mem(stack_pointer + 1, ((cpu.pc & 0xFF00) >> 8) as u8);

    //PC = nn
    cpu.pc = nn;

    cpu.sp = stack_pointer;
}

///
/// Call to nn if zero flag is set
pub fn call_z(cpu: &mut Cpu, mmu: &mut Mmu, nn: u16) {
    if cpu.registers.f.zero_flag() == 1 {
        call(cpu, mmu, nn);
    } else {
        cpu.pc += 3;
    }
}

/// Call to nn if zero flag is clear
pub fn call_nz(cpu: &mut Cpu, mmu: &mut Mmu, nn: u16) {
    if cpu.registers.f.zero_flag() == 0 {
        call(cpu, mmu, nn);
    } else {
        cpu.pc += 3;
    }
}

/// Call to nn if carry flag is set
pub fn call_c(cpu: &mut Cpu, mmu: &mut Mmu, nn: u16) {
    if cpu.registers.f.carry_flag() == 1 {
        call(cpu, mmu, nn);
    } else {
        cpu.pc += 3;
    }
}

/// Call to nn if carry flag is clear
pub fn call_nc(cpu: &mut Cpu, mmu: &mut Mmu, nn: u16) {
    if cpu.registers.f.carry_flag() == 0 {
        call(cpu, mmu, nn);
    } else {
        cpu.pc += 3;
    }
}

///
/// Call to 00, 08, 10, 18, 20, 28, 30, 38(hex)
pub fn rst(cpu: &mut Cpu, mmu: &mut Mmu, n: u8) {
    let mut stack_pointer: u16 = cpu.sp;

    //SP = SP - 2
    stack_pointer -= 2;

    //mem[SP] = lower byte of program counter
    mmu.write_mem(stack_pointer, (cpu.pc & 0x00FF) as u8);

    //mem[SP+1] = upper byte of program counter (its + 1 below because we already moved the stack pointer)
    mmu.write_mem(stack_pointer + 1, ((cpu.pc & 0xFF00) >> 8) as u8);

    //PC = n
    cpu.pc = u16::from_be_bytes([0, n]);
}

///Return
pub fn ret(cpu: &mut Cpu, mmu: &Mmu) {
    let mut sp = cpu.sp;

    //PC = (SP)
    let pc = u16::from_be_bytes([mmu.read_mem(sp + 1), mmu.read_mem(sp)]);

    cpu.pc = pc;

    //SP = SP + 2
    sp += 2;

    cpu.sp = sp;
}

pub fn ret_z(cpu: &mut Cpu, mmu: &Mmu) {
    if cpu.registers.f.zero_flag() == 1 {
        ret(cpu, mmu)
    } else {
        cpu.pc += 1;
    }
}
pub fn ret_nz(cpu: &mut Cpu, mmu: &Mmu) {
    if cpu.registers.f.zero_flag() == 0 {
        ret(cpu, mmu);
    } else {
        cpu.pc += 1;
    }
}

pub fn ret_c(cpu: &mut Cpu, mmu: &Mmu) {
    if cpu.registers.f.carry_flag() == 1 {
        ret(cpu, mmu);
    } else {
        cpu.pc += 1;
    }
}

pub fn ret_nc(cpu: &mut Cpu, mmu: &Mmu) {
    if cpu.registers.f.carry_flag() == 0 {
        ret(cpu, mmu);
    } else {
        cpu.pc += 1;
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
pub fn ld_a_from_io(cpu: &mut Cpu, mmu: &Mmu, n: u8) {
    let addr: u16 = 0xFF00 + (n as u16);
    cpu.registers.a = mmu.read_mem(addr);
}

///
/// Load data from A register into io-port 'n'
pub fn ld_io_from_a(cpu: &Cpu, mmu: &mut Mmu, n: u8) {
    let addr: u16 = 0xFF00 + (n as u16);
    mmu.write_mem(addr, cpu.registers.a);
}

///
/// Load data from io-port C into A register
pub fn ld_a_from_io_c(cpu: &mut Cpu, mmu: &Mmu) {
    let addr: u16 = 0xFF00 + 0x000C;
    cpu.registers.a = mmu.read_mem(addr);
}

///
/// Load data from register A into io-port C
pub fn ld_io_c_from_a(cpu: &Cpu, mmu: &mut Mmu) {
    let addr: u16 = 0xFF00 + 0x000C;
    mmu.write_mem(addr, cpu.registers.a);
}

/************************************************************************
 * 16-bit LOAD instructions
 * *********************************************************************/
///
/// Contents of Register Pair are popped off stack
pub fn pop_rr(mmu: &Mmu, upper: &mut u8, lower: &mut u8, sp: &mut u16) {
    //Stack Pointer
    let mut stack_pointer = *sp;

    //Value in memory (mem[sp])
    let low: u8 = mmu.read_mem(stack_pointer);
    let up: u8 = mmu.read_mem(stack_pointer + 1);

    //rr = mem[sp]
    *lower = low;
    *upper = up;

    //SP = SP + 2
    stack_pointer = stack_pointer + 2;

    *sp = stack_pointer;
}

///
/// Contents of Register Pair are pushed onto stack
pub fn push_rr(mmu: &mut Mmu, upper: u8, lower: u8, sp: &mut u16) {
    //Stack Pointer
    let mut stack_pointer = *sp;

    //SP = SP - 2
    stack_pointer -= 2;

    //mem[sp] = rr
    mmu.write_mem(stack_pointer, lower);
    mmu.write_mem(stack_pointer + 1, upper);

    *sp = stack_pointer;
}
