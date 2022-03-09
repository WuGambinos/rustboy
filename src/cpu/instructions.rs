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
