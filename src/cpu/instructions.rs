use super::*;

pub fn inc_8bit(cpu: &mut Cpu, register: char) {
    match register {
        'A' => {
            //Update Half Carry
            cpu.update_half_carry_flag_sum_8bit(cpu.registers.a, 1);

            //A = A + 1
            cpu.registers.a = cpu.registers.a.wrapping_add(1);

            //Update Zero Flag
            cpu.update_zero_flag(cpu.registers.a);

            //Clear Sub Flag
            cpu.f.sub_flag = 0;
        }
        'B' => {
            //Update Half Carry
            cpu.update_half_carry_flag_sum_8bit(cpu.registers.b, 1);

            //B = B + 1
            cpu.registers.b = cpu.registers.b.wrapping_add(1);

            //Update Zero Flag
            cpu.update_zero_flag(cpu.registers.b);

            //Clear Sub Flag
            cpu.f.sub_flag = 0;
        }
        'C' => {
            //Update Half Carry
            cpu.update_half_carry_flag_sum_8bit(cpu.registers.c, 1);

            //C = C + 1
            cpu.registers.c = cpu.registers.c.wrapping_add(1);

            //Update Zero Flag
            cpu.update_zero_flag(cpu.registers.c);

            //Clear Sub Flag
            cpu.f.sub_flag = 0;
        }
        'D' => {
            //Update Half Carry
            cpu.update_half_carry_flag_sum_8bit(cpu.registers.d, 1);

            //D = D + 1
            cpu.registers.d = cpu.registers.d.wrapping_add(1);

            //Update Zero Flag
            cpu.update_zero_flag(cpu.registers.d);

            //Clear Sub Flag
            cpu.f.sub_flag = 0;
        }
        'E' => {
            //Update Half Carry
            cpu.update_half_carry_flag_sum_8bit(cpu.registers.e, 1);

            //E = E + 1
            cpu.registers.e = cpu.registers.e.wrapping_add(1);

            //Update Zero Flag
            cpu.update_zero_flag(cpu.registers.e);

            //Clear Sub Flag
            cpu.f.sub_flag = 0;
        }
        'H' => {
            //Update Half Carry
            cpu.update_half_carry_flag_sum_8bit(cpu.registers.h, 1);

            //H = H + 1
            cpu.registers.h = cpu.registers.h.wrapping_add(1);

            //Update Zero Flag
            cpu.update_zero_flag(cpu.registers.h);

            //Clear Sub Flag
            cpu.f.sub_flag = 0;
        }
        'L' => {
            //Update Half Carry
            cpu.update_half_carry_flag_sum_8bit(cpu.registers.l, 1);

            //L = L + 1
            cpu.registers.l = cpu.registers.l.wrapping_add(1);

            //Updafte Zero Flag
            cpu.update_zero_flag(cpu.registers.l);

            //Clear Sub Flag
            cpu.f.sub_flag = 0;
        }
        _ => println!("NOT A REGISTER!"),
    }
}

pub fn dec_8bit(cpu: &mut Cpu, register: char) {
    match register {
        'A' => {
            cpu.update_half_carry_flag_sub_8bit(cpu.registers.a, 1);
            cpu.registers.a = cpu.registers.a.wrapping_sub(1);
            cpu.update_zero_flag(cpu.registers.a);
            cpu.f.sub_flag = 1;
        }
        'B' => {
            cpu.update_half_carry_flag_sub_8bit(cpu.registers.b, 1);
            cpu.registers.b = cpu.registers.b.wrapping_sub(1);
            cpu.update_zero_flag(cpu.registers.b);
            cpu.f.sub_flag = 1;
        }
        'C' => {
            cpu.update_half_carry_flag_sub_8bit(cpu.registers.c, 1);
            cpu.registers.c = cpu.registers.c.wrapping_sub(1);
            cpu.update_zero_flag(cpu.registers.c);
            cpu.f.sub_flag = 1;
        }
        'D' => {
            cpu.update_half_carry_flag_sub_8bit(cpu.registers.d, 1);
            cpu.registers.d = cpu.registers.d.wrapping_sub(1);
            cpu.update_zero_flag(cpu.registers.d);
            cpu.f.sub_flag = 1;
        }
        'E' => {
            cpu.update_half_carry_flag_sub_8bit(cpu.registers.e, 1);
            cpu.registers.e = cpu.registers.e.wrapping_sub(1);
            cpu.update_zero_flag(cpu.registers.e);
            cpu.f.sub_flag = 1;
        }
        'H' => {
            cpu.update_half_carry_flag_sub_8bit(cpu.registers.h, 1);
            cpu.registers.h = cpu.registers.h.wrapping_sub(1);
            cpu.update_zero_flag(cpu.registers.h);
            cpu.f.sub_flag = 1;
        }
        'L' => {
            cpu.update_half_carry_flag_sub_8bit(cpu.registers.l, 1);
            cpu.registers.l = cpu.registers.l.wrapping_sub(1);
            cpu.update_zero_flag(cpu.registers.l);
            cpu.f.sub_flag = 1;
        }
        _ => println!("NOT A REGISTER"),
    }
}

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

pub fn inc_16bit(cpu: &mut Cpu, register: &str) {
    match register {
        "BC" => {
            cpu.update_half_carry_flag_sum_16bit(cpu.registers.bc(), 1);
            cpu.registers.set_bc(cpu.registers.bc().wrapping_add(1));
            cpu.f.zero_flag = (cpu.registers.bc() == 0) as u8;
            cpu.f.sub_flag = 1;
        }

        "DE" => {
            cpu.update_half_carry_flag_sum_16bit(cpu.registers.de(), 1);
            cpu.registers.set_de(cpu.registers.de().wrapping_add(1));
            cpu.f.zero_flag = (cpu.registers.de() == 0) as u8;
            cpu.f.sub_flag = 1;
        }

        "HL" => {
            cpu.update_half_carry_flag_sum_16bit(cpu.registers.hl(), 1);
            cpu.registers.set_hl(cpu.registers.hl().wrapping_add(1));
            cpu.f.zero_flag = (cpu.registers.hl() == 0) as u8;
            cpu.f.sub_flag = 1;
        }

        "SP" => {
            cpu.update_half_carry_flag_sum_16bit(cpu.sp, 1);
            cpu.pc = cpu.pc.wrapping_add(1);
            cpu.f.zero_flag = (cpu.pc == 0) as u8;
            cpu.f.sub_flag = 1;
        }
        _ => println!("Not a register PAIR"),
    }
}

fn add_rr_hl(cpu: &mut Cpu, register: &str) {
    match register {
        "BC" => {
            cpu.update_half_carry_flag_sum_16bit(cpu.registers.hl(), cpu.registers.bc());
            cpu.registers
                .set_hl(cpu.registers.hl().wrapping_add(cpu.registers.bc()));
            cpu.f.zero_flag = (cpu.registers.hl() == 0) as u8;
            cpu.f.sub_flag = 0;
        }
        "DE" => {
            cpu.update_half_carry_flag_sum_16bit(cpu.registers.hl(), cpu.registers.de());
            cpu.registers
                .set_hl(cpu.registers.hl().wrapping_add(cpu.registers.de()));
            cpu.f.zero_flag = (cpu.registers.hl() == 0) as u8;
            cpu.f.sub_flag = 0;
        }
        "HL" => {
            cpu.update_half_carry_flag_sum_16bit(cpu.registers.hl(), cpu.registers.hl());
            cpu.registers
                .set_hl(cpu.registers.hl().wrapping_add(cpu.registers.hl()));
            cpu.f.zero_flag = (cpu.registers.hl() == 0) as u8;
            cpu.f.sub_flag = 0;
        }
        _ => println!("NOT A REGISTER PAIR"),
    }
}
