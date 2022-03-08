
use super::*;
#[test]
fn internal() {
    assert_eq!(4, 4);
}

#[test]
fn ld_bc_u16() {
    let mut cpu = Cpu::new();
    let mut mmu = MMU::new();
    cpu.pc = 0;

    mmu.write_mem(0, 0x01);
    mmu.write_mem(1, 0xFA);
    mmu.write_mem(2, 0xDC);

    /*cpu.memory[0] = 0x01;
    cpu.memory[1] = 0xFA;
    cpu.memory[2] = 0xDC;*/

    //FADC

    cpu.emulate_cycle(&mut mmu);

    assert_eq!(cpu.registers.bc(), 0xDCFA);
}

/*************************************************************************
 * 8-bit Arithmetic Tests
 *************************************************************************/

#[test]
fn inc_b() {
    let mut cpu = Cpu::new();

    cpu.registers.b = 0x01;

    cpu.inc_8bit('B');

    //cpu.registers.b = cpu.registers.b.wrapping_add(1);

    assert_eq!(cpu.registers.b, 0x02);
}

#[test]
fn inc_c() {
    let mut cpu = Cpu::new();

    cpu.registers.c = 0x01;

    cpu.inc_8bit('C');

    assert_eq!(cpu.registers.c, 0x02);
}

#[test]
fn inc_d() {
    let mut cpu = Cpu::new();

    cpu.registers.d = 0x05;
    cpu.inc_8bit('D');

    assert_eq!(cpu.registers.d, 0x06);
}

#[test]
fn inc_e() {
    let mut cpu = Cpu::new();

    cpu.registers.e = 0x05;
    cpu.inc_8bit('E');

    assert_eq!(cpu.registers.e, 0x06);
}

#[test]
fn inc_h() {
    let mut cpu = Cpu::new();

    cpu.registers.h = 0x05;
    cpu.inc_8bit('H');

    assert_eq!(cpu.registers.h, 0x06);
}

#[test]
fn inc_l() {
    let mut cpu = Cpu::new();

    cpu.registers.l = 0x05;
    cpu.inc_8bit('L');

    assert_eq!(cpu.registers.l, 0x06);
}

#[test]
fn dec_b() {
    let mut cpu = Cpu::new();

    cpu.registers.b = 0x02;
    cpu.dec_8bit('B');

    assert_eq!(cpu.registers.b, 0x01);
}

#[test]
fn dec_c() {
    let mut cpu = Cpu::new();

    cpu.registers.c = 0x05;
    cpu.dec_8bit('C');

    assert_eq!(cpu.registers.c, 0x04);
}

#[test]
fn dec_d() {
    let mut cpu = Cpu::new();

    cpu.registers.d = 0x03;
    cpu.dec_8bit('D');

    assert_eq!(cpu.registers.d, 0x02);
}

#[test]
fn dec_e() {
    let mut cpu = Cpu::new();

    cpu.registers.e = 0x01;
    cpu.dec_8bit('E');

    assert_eq!(cpu.registers.e, 0x00);
    assert_eq!(cpu.f.zero_flag, 1);
}

#[test]
fn dec_h() {
    let mut cpu = Cpu::new();

    cpu.registers.h = 0x00;
    cpu.dec_8bit('H');

    assert_eq!(cpu.registers.h, 0xFF);
}

#[test]
fn dec_l() {
    let mut cpu = Cpu::new();

    cpu.registers.l = 0x05;
    cpu.dec_8bit('L');

    assert_eq!(cpu.registers.l, 0x04);
}

#[test]
fn inc_8bit_overflow() {
    let mut cpu = Cpu::new();

    cpu.registers.b = 0xFF;
    cpu.inc_8bit('B');

    assert_eq!(cpu.registers.l, 0x00);
    assert_eq!(cpu.f.zero_flag, 0x01);
    assert_eq!(cpu.f.sub_flag, 0x00);
    assert_eq!(cpu.f.half_carry_flag, 0x01);
}

/*************************************************************************
 * 16-bit Arithmetic Tests
 *************************************************************************/

#[test]
fn inc_bc() {
    let mut cpu = Cpu::new();
    cpu.registers.set_bc(0x00FF);
    cpu.inc_16bit("BC");
    assert_eq!(cpu.registers.bc(), 256);
}

#[test]
fn inc_de() {
    let mut cpu = Cpu::new();
    cpu.registers.set_de(0xFFFF);
    cpu.inc_16bit("DE");
    assert_eq!(cpu.f.sub_flag, 1);
    assert_eq!(cpu.registers.de(), 0);
}

#[test]
fn inc_hl() {
    let mut cpu = Cpu::new();
    cpu.registers.set_hl(0x0008);
    cpu.inc_16bit("HL");
    assert_eq!(cpu.registers.hl(), 0x09);
}

#[test]
fn half_carry() {
    let mut cpu = Cpu::new();
    cpu.registers.b = 0x09;

    let operand: u8 = 0x0A;

    cpu.f.half_carry_flag = ((cpu.registers.b & 0xF) + (operand & 0xF) > 0xF) as u8;

    //cpu.inc_8bit_register('B');

    assert_eq!(cpu.f.half_carry_flag, 1);
}

#[test]
fn carry() {
    let mut cpu = Cpu::new();

    cpu.registers.set_bc(0xFFFF);
    cpu.registers.set_hl(0x0001);

    cpu.update_carry_flag_16bit(cpu.registers.bc(), cpu.registers.hl());

    //assert_eq!(cpu.f.carry_flag, 1);
    assert_eq!(cpu.registers.bc(), 0xFFFF);
    assert_eq!(cpu.registers.hl(), 0x0001);
}
