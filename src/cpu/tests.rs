use super::*;
#[test]
fn internal() {
    assert_eq!(4, 4);
}

#[test]
fn ld_bc_u16() {
    let mut cpu = Cpu::new();
    let mut mmu = Mmu::new();
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

    instructions::inc_8bit(&mut cpu.f, &mut cpu.registers.b);

    //ZNHC
    let check = vec![
        cpu.registers.b,
        cpu.f.zero_flag,
        cpu.f.sub_flag,
        cpu.f.half_carry_flag,
        cpu.f.carry_flag,
    ];

    assert_eq!(check, [0x02, 0, 0, 0, 0]);
}

#[test]
fn inc_c() {
    let mut cpu = Cpu::new();

    cpu.registers.c = 0x01;

    instructions::inc_8bit(&mut cpu.f, &mut cpu.registers.c);

    //ZNHC
    let check = vec![
        cpu.registers.c,
        cpu.f.zero_flag,
        cpu.f.sub_flag,
        cpu.f.half_carry_flag,
        cpu.f.carry_flag,
    ];

    assert_eq!(check, [0x02, 0, 0, 0, 0]);
}

#[test]
fn inc_d() {
    let mut cpu = Cpu::new();

    cpu.registers.d = 0x05;
    instructions::inc_8bit(&mut cpu.f, &mut cpu.registers.d);

    //ZNHC
    let check = vec![
        cpu.registers.d,
        cpu.f.zero_flag,
        cpu.f.sub_flag,
        cpu.f.half_carry_flag,
        cpu.f.carry_flag,
    ];

    assert_eq!(check, [0x06, 0, 0, 0, 0]);
}

#[test]
fn inc_e() {
    let mut cpu = Cpu::new();

    cpu.registers.e = 0x05;
    instructions::inc_8bit(&mut cpu.f, &mut cpu.registers.e);

    //ZNHC
    let check = vec![
        cpu.registers.e,
        cpu.f.zero_flag,
        cpu.f.sub_flag,
        cpu.f.half_carry_flag,
        cpu.f.carry_flag,
    ];

    assert_eq!(check, [0x06, 0, 0, 0, 0]);
}

#[test]
fn inc_h() {
    let mut cpu = Cpu::new();

    cpu.registers.h = 0x05;
    instructions::inc_8bit(&mut cpu.f, &mut cpu.registers.h);

    //ZNHC
    let check = vec![
        cpu.registers.h,
        cpu.f.zero_flag,
        cpu.f.sub_flag,
        cpu.f.half_carry_flag,
        cpu.f.carry_flag,
    ];

    assert_eq!(check, [0x06, 0, 0, 0, 0]);
}

#[test]
fn inc_l() {
    let mut cpu = Cpu::new();

    cpu.registers.l = 0x05;
    instructions::inc_8bit(&mut cpu.f, &mut cpu.registers.l);

    assert_eq!(cpu.registers.l, 0x06);
}

#[test]
fn inc_8bit_overflow() {
    let mut cpu = Cpu::new();

    cpu.registers.b = 0xFF;
    instructions::inc_8bit(&mut cpu.f, &mut cpu.registers.b);

    //ZNHC
    let check = vec![
        cpu.registers.l,
        cpu.f.zero_flag,
        cpu.f.sub_flag,
        cpu.f.half_carry_flag,
        cpu.f.carry_flag,
    ];

    assert_eq!(check, [0x00, 1, 0, 1, 0]);
}

#[test]
fn dec_b() {
    let mut cpu = Cpu::new();

    cpu.registers.b = 0x02;
    instructions::dec_8bit(&mut cpu.f, &mut cpu.registers.b);

    //ZNHC
    let check = vec![
        cpu.registers.b,
        cpu.f.zero_flag,
        cpu.f.sub_flag,
        cpu.f.half_carry_flag,
        cpu.f.carry_flag,
    ];

    assert_eq!(check, [0x01, 0, 1, 0, 0]);
}

#[test]
fn dec_c() {
    let mut cpu = Cpu::new();

    cpu.registers.c = 0x05;
    instructions::dec_8bit(&mut cpu.f, &mut cpu.registers.c);

    //ZNHC
    let check = vec![
        cpu.registers.c,
        cpu.f.zero_flag,
        cpu.f.sub_flag,
        cpu.f.half_carry_flag,
        cpu.f.carry_flag,
    ];

    assert_eq!(check, [0x04, 0, 1, 0, 0]);
}

#[test]
fn dec_d() {
    let mut cpu = Cpu::new();

    cpu.registers.d = 0x03;
    instructions::dec_8bit(&mut cpu.f, &mut cpu.registers.d);

    //Flags : ZNHC
    let check = vec![
        cpu.registers.d,
        cpu.f.zero_flag,
        cpu.f.sub_flag,
        cpu.f.half_carry_flag,
        cpu.f.carry_flag,
    ];

    assert_eq!(check, [0x02, 0, 1, 0, 0])
}

#[test]
fn dec_e() {
    let mut cpu = Cpu::new();

    cpu.registers.e = 0x01;
    instructions::dec_8bit(&mut cpu.f, &mut cpu.registers.e);

    //ZNHC
    let check = vec![
        cpu.registers.e,
        cpu.f.zero_flag,
        cpu.f.sub_flag,
        cpu.f.half_carry_flag,
        cpu.f.carry_flag,
    ];

    assert_eq!(check, [0x00, 1, 1, 0, 0]);
}

#[test]
fn dec_h() {
    let mut cpu = Cpu::new();

    cpu.registers.h = 0x00;
    instructions::dec_8bit(&mut cpu.f, &mut cpu.registers.h);

    //ZNHC
    let check = vec![
        cpu.registers.h,
        cpu.f.zero_flag,
        cpu.f.sub_flag,
        cpu.f.half_carry_flag,
        cpu.f.carry_flag,
    ];

    assert_eq!(check, [0xFF, 0, 1, 1, 0]);
}

#[test]
fn dec_l() {
    let mut cpu = Cpu::new();

    cpu.registers.l = 0x05;
    instructions::dec_8bit(&mut cpu.f, &mut cpu.registers.l);

    //ZNHC
    let check = vec![
        cpu.registers.l,
        cpu.f.zero_flag,
        cpu.f.sub_flag,
        cpu.f.half_carry_flag,
        cpu.f.carry_flag,
    ];

    assert_eq!(check, [0x04, 0, 1, 0, 0]);
}

///Basic test for ADD r r instruction
#[test]
fn add_r() {
    let mut cpu = Cpu::new();

    cpu.registers.a = 0xFF;
    cpu.registers.b = 0x05;
    let b = cpu.registers.b;

    instructions::add_a_r(&mut cpu, b);

    //ZNHC
    let check = vec![
        cpu.registers.a,
        cpu.f.zero_flag,
        cpu.f.sub_flag,
        cpu.f.half_carry_flag,
        cpu.f.carry_flag,
    ];
    assert_eq!(check, [0x04, 0, 0, 1, 1]);
}

//Test For Overflow with ADD r r instruction
#[test]
fn add_r_overflow() {
    let mut cpu = Cpu::new();

    cpu.registers.a = 0xFF;
    cpu.registers.b = 0x80;

    let b: u8 = cpu.registers.b;

    instructions::add_a_r(&mut cpu, b);

    //ZNHC
    let check = vec![
        cpu.registers.a,
        cpu.f.zero_flag,
        cpu.f.sub_flag,
        cpu.f.half_carry_flag,
        cpu.f.carry_flag,
    ];

    assert_eq!(check, [127, 0, 0, 0, 1]);
}

///Basic test for ADC r r instruction
#[test]
fn adc_r() {
    let mut cpu = Cpu::new();

    cpu.registers.a = 0x02;
    cpu.registers.b = 0x25;
    cpu.f.carry_flag = 1;

    let b: u8 = cpu.registers.b;

    instructions::adc_a_r(&mut cpu, b);

    //ZNHC
    let check = vec![
        cpu.registers.a,
        cpu.f.zero_flag,
        cpu.f.sub_flag,
        cpu.f.half_carry_flag,
        cpu.f.carry_flag,
    ];

    assert_eq!(check, [0x28, 0, 0, 0, 0]);
}

///Basic test for SUB r r instruction
#[test]
fn sub_r() {
    let mut cpu = Cpu::new();

    cpu.registers.a = 0x05;
    cpu.registers.b = 0x03;

    let b: u8 = cpu.registers.b;

    instructions::sub_r_r(&mut cpu, b);

    //ZNHC
    let check = vec![
        cpu.registers.a,
        cpu.f.zero_flag,
        cpu.f.sub_flag,
        cpu.f.half_carry_flag,
        cpu.f.carry_flag,
    ];

    assert_eq!(check, [0x02, 0, 1, 0, 0]);
}

///Basic test for SBC r r instruction
#[test]
fn sbc_r() {
    let mut cpu = Cpu::new();

    cpu.registers.a = 0x08;
    cpu.registers.b = 0x02;
    cpu.f.carry_flag = 1;

    let b: u8 = cpu.registers.b;

    instructions::sbc_r_r(&mut cpu, b);

    //ZNHC
    let check = vec![
        cpu.registers.a,
        cpu.f.zero_flag,
        cpu.f.sub_flag,
        cpu.f.half_carry_flag,
        cpu.f.carry_flag,
    ];

    assert_eq!(check, [0x05, 0, 1, 0, 0]);
}

//Testing for correct borrow detection
#[test]
fn sub_r_borrow() {
    let mut cpu = Cpu::new();

    cpu.registers.a = 0x07;
    cpu.registers.b = 0x10;

    let b: u8 = cpu.registers.b;

    instructions::sub_r_r(&mut cpu, b);

    //ZNHC
    let check = vec![
        cpu.registers.a,
        cpu.f.zero_flag,
        cpu.f.sub_flag,
        cpu.f.half_carry_flag,
        cpu.f.carry_flag,
    ];

    assert_eq!(check, [0xF7, 0, 1, 0, 1])
}

///Testing for correct result when borrow(carry) is set
#[test]
fn sbc_r_borrow_set() {
    let mut cpu = Cpu::new();

    cpu.registers.a = 0x08;
    cpu.registers.b = 0x03;
    cpu.f.carry_flag = 1;

    let b: u8 = cpu.registers.b;

    instructions::sbc_r_r(&mut cpu, b);

    //ZNHC
    let check = vec![
        cpu.registers.a,
        cpu.f.zero_flag,
        cpu.f.sub_flag,
        cpu.f.half_carry_flag,
        cpu.f.carry_flag,
    ];

    assert_eq!(check, [0x04, 0, 1, 0, 0]);
}

/*************************************************************************
 * 16-bit Arithmetic Tests
 *************************************************************************/

#[test]
fn inc_bc() {
    let mut cpu = Cpu::new();
    cpu.registers.set_bc(0x00FF);
    instructions::inc_16bit(&mut cpu, "BC");
    assert_eq!(cpu.registers.bc(), 256);
}

#[test]
fn inc_de() {
    let mut cpu = Cpu::new();
    cpu.registers.set_de(0xFFFF);
    instructions::inc_16bit(&mut cpu, "DE");
    assert_eq!(cpu.registers.de(), 0);
}

#[test]
fn inc_hl() {
    let mut cpu = Cpu::new();
    cpu.registers.set_hl(0x0008);
    instructions::inc_16bit(&mut cpu, "HL");
    assert_eq!(cpu.registers.hl(), 0x09);
}

#[test]
fn add_bc_hl() {
    let mut cpu = Cpu::new();

    cpu.registers.set_hl(0xF0);

    cpu.registers.set_bc(0xFF);

    instructions::add_rr_hl(&mut cpu, "BC");

    assert_eq!(cpu.registers.hl(), 0x01EF);
}

#[test]
fn add_de_hl() {
    let mut cpu = Cpu::new();

    cpu.registers.set_hl(0x0002);
    cpu.registers.set_de(0x0005);

    instructions::add_rr_hl(&mut cpu, "DE");
    assert_eq!(cpu.registers.hl(), 0x0007);
}

#[test]
fn add_hl_hl() {
    let mut cpu = Cpu::new();

    cpu.registers.set_hl(0xFF);

    instructions::add_rr_hl(&mut cpu, "HL");
    assert_eq!(cpu.registers.hl(), 0x01FE);
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

    cpu.f
        .update_carry_flag_sum_16bit(cpu.registers.bc(), cpu.registers.hl());

    //assert_eq!(cpu.f.carry_flag, 1);
    assert_eq!(cpu.registers.bc(), 0xFFFF);
    assert_eq!(cpu.registers.hl(), 0x0001);
}

/*************************************************************************
 * 8-bit Load Tests
 *************************************************************************/

#[test]
fn load_8bit_test() {
    let mut cpu = Cpu::new();

    cpu.registers.c = 5;
    ld_8bit(&mut (cpu.registers.b), cpu.registers.c);
}

/*************************************************************************
 * 16-bit Load Tests
 *************************************************************************/
#[test]
fn pop_rr_test() {
    let mut cpu = Cpu::new();
    let mut mmu = Mmu::new();

    cpu.sp = 0x1000;

    mmu.write_mem(cpu.sp, 0x55);
    mmu.write_mem(cpu.sp + 1, 0x33);

    instructions::pop_rr(
        &mmu,
        &mut cpu.registers.b,
        &mut cpu.registers.c,
        &mut cpu.sp,
    );

    assert_eq!(cpu.registers.bc(), 0x3355);
}

#[test]
fn push_rr_test() {
    let mut cpu = Cpu::new();
    let mut mmu = Mmu::new();

    cpu.registers.b = 0x22;
    cpu.registers.c = 0x33;

    cpu.sp = 0x1007;

    instructions::push_rr(&mut mmu, cpu.registers.b, cpu.registers.c, &mut cpu.sp);

    let check: Vec<u16> = vec![0x0022, 0x0033, 0x1005];

    assert_eq!(
        check,
        [
            mmu.read_mem(0x1006) as u16,
            mmu.read_mem(0x1005) as u16,
            cpu.sp
        ]
    );
}

/*************************************************************************
 * Jump Instructions Tests
 *************************************************************************/

#[test]
fn call_test() {
    let mut cpu: Cpu = Cpu::new();
    let mut mmu: Mmu = Mmu::new();

    cpu.pc = 0x1A47;
    cpu.sp = 0x3002;

    mmu.write_mem(cpu.pc, 0xCD);
    mmu.write_mem(cpu.pc + 1, 0x35);
    mmu.write_mem(cpu.pc + 2, 0x21);

    let nn: u16 = u16::from_be_bytes([mmu.read_mem(cpu.pc + 2), mmu.read_mem(cpu.pc + 1)]);
    instructions::call(&mut cpu, &mut mmu, nn);

    let check: Vec<u16> = vec![0x001A, 0x0047, 0x3000, 0x2135];

    assert_eq!(
        check,
        [
            mmu.read_mem(0x3001) as u16,
            mmu.read_mem(0x3000) as u16,
            cpu.sp,
            cpu.pc
        ]
    );
}

#[test]
fn ret_test() {
    let mut cpu = Cpu::new();
    let mut mmu = Mmu::new();

    cpu.pc = 0x3535;
    cpu.sp = 0x2000;
    mmu.write_mem(cpu.sp, 0xB5);
    mmu.write_mem(cpu.sp + 1, 0x18);

    instructions::ret(&mut cpu, &mmu);

    let check: Vec<u16> = vec![0x2002, 0x18B5];

    assert_eq!(check, [cpu.sp, cpu.pc]);
}

/*************************************************************************
 * Flags Tests
 *************************************************************************/
#[test]
fn set_flags() {
    let mut cpu = Cpu::new();

    cpu.registers.set_zero_flag();
    cpu.registers.set_sub_flag();
    cpu.registers.set_hc_flag();
    cpu.registers.set_carry_flag();

    assert_eq!(cpu.registers.f, 0b11110000);
}

#[test]
fn clear_flags() {
    let mut cpu = Cpu::new();

    cpu.registers.set_zero_flag();
    cpu.registers.set_sub_flag();
    cpu.registers.set_hc_flag();
    cpu.registers.set_carry_flag();

    cpu.registers.clear_zero_flag();
    cpu.registers.clear_sub_flag();
    cpu.registers.clear_hc_flag();
    cpu.registers.clear_carry_flag();

    assert_eq!(cpu.registers.f, 0b00000000);
}
