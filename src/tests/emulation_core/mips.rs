#![allow(clippy::unusual_byte_groupings)]

use crate::emulation_core::datapath::Datapath;
use crate::emulation_core::mips::datapath::MipsDatapath;
use crate::emulation_core::mips::registers::GpRegisterType;

pub mod api {
    use super::*;
    use crate::parser::parser_main::parser;

    #[test]
    fn reset_datapath() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // Add instruction into emulation core memory.
        let instruction = String::from("ori $s0, $zero, 5");
        let (_, instruction_bits) = parser(instruction);
        datapath.load_instructions(instruction_bits)?;

        datapath.execute_instruction();

        // Datapath should now have some data in it.
        assert_ne!(datapath.memory.memory[0], 0);
        assert_ne!(datapath.registers.gpr[16], 0); // $s0
        assert_ne!(datapath.registers.pc, 0);

        datapath.reset();

        // After resetting, these values should all be back to bitwise zero.
        assert_eq!(datapath.memory.memory[0], 0);
        assert_eq!(datapath.registers.gpr[16], 0); // $s0
        assert_eq!(datapath.registers.pc, 0);

        Ok(())
    }
}

pub mod add {
    use super::*;
    #[test]
    fn add_register_to_itself() {
        let mut datapath = MipsDatapath::default();

        // $t1 = $t1 + $t1
        //                       R-type  t1    t1    t1  (shamt)  ADD
        let instruction: u32 = 0b000000_01001_01001_01001_00000_100000;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        // Assume the register $t1 has the value 5.
        datapath.registers[GpRegisterType::T1] = 5;

        datapath.execute_instruction();

        // After the operation is finished, the register should be 10.
        assert_eq!(datapath.registers[GpRegisterType::T1], 10);
    }

    #[test]
    fn add_register_to_another() {
        let mut datapath = MipsDatapath::default();

        // $s2 = $s0 + $s1
        //                       R-type  s0    s1    s2  (shamt)  ADD
        let instruction: u32 = 0b000000_10000_10001_10010_00000_100000;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.registers.gpr[16] = 15; // $s0
        datapath.registers.gpr[17] = 40; // $s1

        datapath.execute_instruction();

        // Register $s2 should contain 55.
        let result = datapath.registers.gpr[18] as u32;
        assert_eq!(result, 55);
    }

    #[test]
    // This test attempts to write to register $zero. The datapath should
    // not overwrite this register, and remain with a value of 0.
    fn add_to_register_zero() {
        let mut datapath = MipsDatapath::default();

        // $zero = $t3 + $t3
        //                       R-type  t3    t3    zero (shamt) ADD
        let instruction: u32 = 0b000000_01011_01011_00000_00000_100000;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.registers.gpr[11] = 1234; // $t3

        datapath.execute_instruction();

        // $zero should still contain 0.
        assert_eq!(datapath.registers.gpr[0], 0);
    }

    #[test]
    // NOTE: This test falls under our initial project design that there are no
    // handled exceptions. Therefore, we would expect to see an updated value in
    // register T1, rather than having the register unmodified per the MIPS64v6
    // specification.
    fn add_32_bit_with_overflow() {
        let mut datapath = MipsDatapath::default();

        // $t1 = $t4 + $t4
        //                       R-type  t4    t4    t1 (shamt) ADD
        let instruction: u32 = 0b000000_01100_01100_01001_00000_100000;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        // Assume register $t4 contains 2,454,267,026, a 32-bit integer.
        datapath.registers.gpr[12] = 0b10010010_01001001_00100100_10010010;

        datapath.execute_instruction();

        // Disregarding overflow, register $t4 would contain 4,908,534,052, or
        // 1_00100100_10010010_01001001_00100100 in binary. The result
        // should be truncated. Thus, we should expect the register to
        // contain 613,566,756, or 00100100_10010010_01001001_00100100 in binary.
        assert_eq!(datapath.registers.gpr[9], 613566756);
    }

    #[test]
    // NOTE: This test falls under our initial project design that there are no
    // handled exceptions. Therefore, we would expect to see an updated value in
    // register T1, rather than having the register unmodified per the MIPS64v6
    // specification.
    fn add_32_bit_with_overflow_sign_extend() {
        let mut datapath = MipsDatapath::default();

        // $t1 = $t4 + $t4
        //                       R-type  t4    t4    t1 (shamt) ADD
        let instruction: u32 = 0b000000_01100_01100_01001_00000_100000;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        // Assume register $t4 contains 3,528,008,850, a 32-bit integer.
        datapath.registers.gpr[12] = 0b11010010_01001001_00100100_10010010;

        datapath.execute_instruction();

        // Disregarding overflow, register $t4 would contain 7,056,017,700, or
        // 1_10100100_10010010_01001001_00100100 in binary. The result
        // should be truncated. Thus, we should expect the register to
        // contain 2,761,050,404, or 10100100_10010010_01001001_00100100 in binary.
        assert_eq!(datapath.registers.gpr[9] as u32, 2761050404);
    }
}

#[test]
fn sub_positive_result() {
    let mut datapath = MipsDatapath::default();

    // $s2 = $s3 - $s2
    //                       R-type  s3    s2    s2  (shamt) SUB
    let instruction: u32 = 0b000000_10011_10010_10010_00000_100010;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    datapath.registers.gpr[19] = 7; // $s3
    datapath.registers.gpr[18] = 3; // $s2

    datapath.execute_instruction();

    // Register $s2 should contain 4, as 7 - 3 = 4.
    assert_eq!(datapath.registers.gpr[18], 4);
}

#[test]
fn sub_32_bit_negative_result() {
    let mut datapath = MipsDatapath::default();

    // $s0 = $s0 - $t0
    //                       R-type  s0    t0    s0  (shamt) SUB
    let instruction: u32 = 0b000000_10000_01000_10000_00000_100010;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    datapath.registers.gpr[16] = 5; // $s0
    datapath.registers.gpr[8] = 20; // $t0

    datapath.execute_instruction();

    // Register $s0 should contain -15, as 5 - 20 = -15.
    assert_eq!(datapath.registers.gpr[16] as i32, -15);
}

#[test]
fn sub_32_bit_underflow() {
    let mut datapath = MipsDatapath::default();

    // $s0 = $s0 - $t0
    //                       R-type  s0    t0    s0  (shamt) SUB
    let instruction: u32 = 0b000000_10000_01000_10000_00000_100010;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    datapath.registers.gpr[16] = 0; // $s0
    datapath.registers.gpr[8] = 1; // $t0

    datapath.execute_instruction();

    // Register $s0 should contain the largest unsigned 32-bit integer due to underflow.
    assert_eq!(
        datapath.registers.gpr[16] as u32,
        0b11111111_11111111_11111111_11111111
    );
}

#[test]
fn mul_positive_result() {
    let mut datapath = MipsDatapath::default();

    // $s5 = $t7 * $t6
    //                       R-type  t7    t6    s5    MUL   SOP30
    let instruction: u32 = 0b000000_01111_01110_10101_00010_011000;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    datapath.registers.gpr[15] = 8; // $t7
    datapath.registers.gpr[14] = 95; // $t6

    datapath.execute_instruction();

    assert_eq!(datapath.registers.gpr[21], 760); // $s5
}

#[test]
fn mul_32_bit_negative_result() {
    let mut datapath = MipsDatapath::default();

    // $s5 = $t7 * $t6
    //                       R-type  t7    t6    s5    MUL   SOP30
    let instruction: u32 = 0b000000_01111_01110_10101_00010_011000;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    datapath.registers.gpr[15] = 5; // $t7
    datapath.registers.gpr[14] = -5_i64 as u64; // $t6

    datapath.execute_instruction();

    assert_eq!(datapath.registers.gpr[21] as i64, -25); // $s5
}

#[test]
fn mul_result_truncate() {
    let mut datapath = MipsDatapath::default();

    // $s4 = $t6 * $t5
    //                       R-type  t6    t5    s4    MUL   SOP30
    let instruction: u32 = 0b000000_01110_01101_10100_00010_011000;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    datapath.registers.gpr[14] = 731_564_544; // $t6
    datapath.registers.gpr[13] = 8; // $t5

    datapath.execute_instruction();

    // The result, 5,852,516,352, is too large for a 32-bit integer.
    // (1 01011100 11010110 01010000 00000000)
    // The result should instead truncate to the lower 32 bits.
    assert_eq!(
        datapath.registers.gpr[20],
        0b01011100_11010110_01010000_00000000
    ); // $s5
}

#[test]
fn div_positive_result() {
    let mut datapath = MipsDatapath::default();

    // $s4 = $t6 / $t5
    //                       R-type  t6    t5    s4    DIV   SOP32
    let instruction: u32 = 0b000000_01110_01101_10100_00010_011010;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    datapath.registers.gpr[14] = 20; // $t6
    datapath.registers.gpr[13] = 2; // $t5

    datapath.execute_instruction();

    assert_eq!(datapath.registers.gpr[20], 10); // $s5
}

#[test]
fn div_negative_result() {
    let mut datapath = MipsDatapath::default();

    // $s4 = $t6 / $t5
    //                       R-type  t6    t5    s4    DIV   SOP32
    let instruction: u32 = 0b000000_01110_01101_10100_00010_011010;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    datapath.registers.gpr[14] = 20; // $t6
    datapath.registers.gpr[13] = -5_i64 as u64; // $t5

    datapath.execute_instruction();

    assert_eq!(datapath.registers.gpr[20] as i64, -4); // $s5
}

pub mod and {
    use super::*;

    #[test]
    fn and_register_to_itself() {
        let mut datapath = MipsDatapath::default();

        // $t1 = $t1 & $t1
        //                       R-type  t1    t1    t1  (shamt)  AND
        let instruction: u32 = 0b000000_01001_01001_01001_00000_100100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        // Assume the register $t1 has the value 5.
        datapath.registers[GpRegisterType::T1] = 0x5;

        datapath.execute_instruction();
        assert_eq!(datapath.registers[GpRegisterType::T1], 0x5);
    }

    #[test]
    fn and_register_to_another16() {
        let mut datapath = MipsDatapath::default();

        // $s2 = $s0 & $s1
        //                       R-type  s0    s1    s2  (shamt)  AND
        let instruction: u32 = 0b000000_10000_10001_10010_00000_100100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.registers.gpr[16] = 0x1234; // $s0
        datapath.registers.gpr[17] = 0x4321; // $s1

        datapath.execute_instruction();

        // Register $s2 should contain 55.
        let result = datapath.registers.gpr[18];
        assert_eq!(result, 0x0220);
    }

    #[test]
    fn and_register_to_another32() {
        let mut datapath = MipsDatapath::default();

        // $s2 = $s0 & $s1
        //                       R-type  s0    s1    s2  (shamt)  AND
        let instruction: u32 = 0b000000_10000_10001_10010_00000_100100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.registers.gpr[16] = 0x12341234; // $s0
        datapath.registers.gpr[17] = 0x43214321; // $s1

        datapath.execute_instruction();

        // Register $s2 should contain 55.
        let result = datapath.registers.gpr[18];
        assert_eq!(result, 0x02200220);
    }

    #[test]
    fn and_register_to_another64() {
        let mut datapath = MipsDatapath::default();

        // $s2 = $s0 & $s1
        //                       R-type  s0    s1    s2  (shamt)  AND
        let instruction: u32 = 0b000000_10000_10001_10010_00000_100100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.registers.gpr[16] = 0x1234123412341234; // $s0
        datapath.registers.gpr[17] = 0x4321432143214321; // $s1

        datapath.execute_instruction();

        // Register $s2 should contain 55.
        let result = datapath.registers.gpr[18];
        assert_eq!(result, 0x0220022002200220);
    }

    #[test]
    // This test attempts to write to register $zero. The datapath should
    // not overwrite this register, and remain with a value of 0.
    fn and_to_register_zero() {
        let mut datapath = MipsDatapath::default();

        // $zero = $t3 & $t3
        //                       R-type  t3    t3    zero (shamt) AND
        let instruction: u32 = 0b000000_01011_01011_00000_00000_100100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.registers.gpr[11] = 1234; // $t3

        datapath.execute_instruction();

        // $zero should still contain 0.
        assert_eq!(datapath.registers.gpr[0], 0);
    }
}

pub mod andi {
    use super::*;
    #[test]
    fn and_immediate_with_zero() {
        let mut datapath = MipsDatapath::default();

        // $s0 = $zero & 12345
        //                       andi    $zero  $s0   12345
        let instruction: u32 = 0b001100_00000_10000_0011000000111001;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[16], 0); // $s0
    }

    #[test]
    fn andi_immediate_with_value() {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 & 12345
        //                       andi     $t0   $s0   12345
        let instruction: u32 = 0b001100_01000_10000_0011000000111001;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        // In binary: 00111010 11011110 01101000 10110001
        datapath.registers.gpr[8] = 987654321; // $t0

        datapath.execute_instruction();

        // The result should be as follows:
        //         $t0:  00111010 11011110 01101000 10110001
        // AND  12,345:                    00110000 00111001
        // =================================================
        // 987,658,425:  00000000 00000000 00100000 00110001

        assert_eq!(datapath.registers.gpr[16], 0x2031); // $s0
    }
}

pub mod addi_addiu {
    use super::*;
    #[test]
    fn addi_simple_test() {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x1
        //                       addi    $t0   $s0          4
        let instruction: u32 = 0b001000_01000_10000_0000000000000100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store addi");
        datapath.registers[GpRegisterType::T0] = 1;
        datapath.registers[GpRegisterType::S0] = 123;
        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S0], 5);
    }

    #[test]
    fn addi_overflow_test() {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x1
        //                       addi    $t0   $s0          1
        let instruction: u32 = 0b001000_01000_10000_0000000000000100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store addi");
        datapath.registers[GpRegisterType::T0] = 0xffffffff;
        datapath.registers[GpRegisterType::S0] = 123;
        datapath.execute_instruction();

        // if there is an overflow, $s0 should not change.
        // For the addiu instruction, $s0 would change on overflow, it would become 3.
        assert_eq!(datapath.registers[GpRegisterType::S0], 123);
    }

    #[test]
    fn addi_sign_extend_test() {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x1
        //                       addi    $t0   $s0          1
        let instruction: u32 = 0b001000_01000_10000_0000000000000001;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store addi");
        datapath.registers[GpRegisterType::T0] = 0xfffffff1;
        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S0], 0xfffffffffffffff2);
    }

    #[test]
    fn addi_sign_extend_test2() {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x1
        //                       addi    $t0   $s0          1
        let instruction: u32 = 0b001000_01000_10000_0000000000000001;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store addi");
        datapath.registers[GpRegisterType::T0] = 0xfffffffe;
        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S0], 0xffffffffffffffff);
    }

    #[test]
    fn addiu_simple_test() {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x1
        //                       addiu    $t0   $s0          4
        let instruction: u32 = 0b001001_01000_10000_0000000000000100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store addi");
        datapath.registers[GpRegisterType::T0] = 1;
        datapath.registers[GpRegisterType::S0] = 123;
        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S0], 5);
    }

    #[test]
    fn addiu_overflow_test() {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x1
        //                       addiu    $t0   $s0          1
        let instruction: u32 = 0b001001_01000_10000_0000000000000100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store addi");
        datapath.registers[GpRegisterType::T0] = 0xffffffff;
        datapath.registers[GpRegisterType::S0] = 123;
        datapath.execute_instruction();

        // if there is an overflow, $s0 should not change.
        // For the addiu instruction, $s0 would change on overflow, it would become 3.
        assert_eq!(datapath.registers[GpRegisterType::S0], 3);
    }

    #[test]
    fn addiu_sign_extend_test() {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x1
        //                       addi    $t0   $s0          1
        let instruction: u32 = 0b001000_01000_10000_0000000000000001;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store addi");
        datapath.registers[GpRegisterType::T0] = 0xfffffff1;
        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S0], 0xfffffffffffffff2);
    }
}

pub mod daddi_and_daddiu {
    use super::*;
    #[test]
    fn daddi_simple_test() {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x1
        //                       daddi    $t0   $s0          4
        let instruction: u32 = 0b011000_01000_10000_0000000000000100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store addi");
        datapath.registers[GpRegisterType::T0] = 1;
        datapath.registers[GpRegisterType::S0] = 123;
        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S0], 5);
    }

    #[test]
    fn daddi_overflow_test() {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x1
        //                       daddi    $t0   $s0          1
        let instruction: u32 = 0b011000_01000_10000_0000000000000100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store addi");
        datapath.registers[GpRegisterType::T0] = 0xffffffffffffffff;
        datapath.registers[GpRegisterType::S0] = 123;
        datapath.execute_instruction();

        // if there is an overflow, $s0 should not change.
        // For the addiu instruction, $s0 would change on overflow, it would become 3.
        assert_eq!(datapath.registers[GpRegisterType::S0], 123);
    }

    #[test]
    fn daddi_sign_extend_test() {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x1
        //                       daddi    $t0   $s0          1
        let instruction: u32 = 0b011000_01000_10000_0000000000000001;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store addi");
        datapath.registers[GpRegisterType::T0] = 0xfffffffffffffff1;
        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S0], 0xfffffffffffffff2);
    }

    #[test]
    fn daddi_sign_extend_test2() {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x1
        //                       daddi    $t0   $s0          1
        let instruction: u32 = 0b011000_01000_10000_0000000000000001;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store addi");
        datapath.registers[GpRegisterType::T0] = 0xfffffffffffffffe;
        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S0], 0xffffffffffffffff);
    }

    #[test]
    fn daddiu_simple_test() {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x1
        //                       daddiu    $t0   $s0          4
        let instruction: u32 = 0b011001_01000_10000_0000000000000100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store addi");
        datapath.registers[GpRegisterType::T0] = 1;
        datapath.registers[GpRegisterType::S0] = 123;
        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S0], 5);
    }

    #[test]
    fn daddiu_overflow_test() {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x1
        //                       daddiu    $t0   $s0          1
        let instruction: u32 = 0b011001_01000_10000_0000000000000100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store addi");
        datapath.registers[GpRegisterType::T0] = 0xffffffffffffffff;
        datapath.registers[GpRegisterType::S0] = 123;
        datapath.execute_instruction();

        // if there is an overflow, $s0 should not change.
        // For the addiu instruction, $s0 would change on overflow, it would become 3.
        assert_eq!(datapath.registers[GpRegisterType::S0], 3);
    }

    #[test]
    fn daddiu_sign_extend_test() {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x1
        //                       daddiu    $t0   $s0          1
        let instruction: u32 = 0b011001_01000_10000_0000000000000001;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store addi");
        datapath.registers[GpRegisterType::T0] = 0xfffffffffffffff1;
        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S0], 0xfffffffffffffff2);
    }
}

pub mod ori {
    use super::*;
    #[test]
    fn or_immediate_with_zero() {
        let mut datapath = MipsDatapath::default();

        // $s0 = $zero | 12345
        //                       ori    $zero  $s0   12345
        let instruction: u32 = 0b001101_00000_10000_0011000000111001;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[16], 12345); // $s0
    }

    #[test]
    fn or_immediate_with_value() {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 | 12345
        //                       ori     $t0   $s0   12345
        let instruction: u32 = 0b001101_01000_10000_0011000000111001;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        // In binary: 00111010 11011110 01101000 10110001
        datapath.registers.gpr[8] = 987654321; // $t0

        datapath.execute_instruction();

        // The result should be as follows:
        //         $t0:  00111010 11011110 01101000 10110001
        // OR   12,345:                    00110000 00111001
        // =================================================
        // 987,658,425:  00111010 11011110 01111000 10111001

        assert_eq!(datapath.registers.gpr[16], 987658425); // $s0
    }
}

#[test]
fn dadd_register_to_itself() {
    let mut datapath = MipsDatapath::default();

    // dadd rd, rs, rt
    // dadd $v0, $t5, $t5
    // GPR[2] <- GPR[13] + GPR[13]
    //                      SPECIAL rs    rt    rd    0     DADD
    //                              13    13    2
    let instruction: u32 = 0b000000_01101_01101_00010_00000_101100;

    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    // Assume register $t5 contains 969,093,589,304, which is an integer
    // that takes up 39 bits.
    datapath.registers.gpr[13] = 969_093_589_304; // $t5

    datapath.execute_instruction();

    assert_eq!(datapath.registers.gpr[2], 1_938_187_178_608); // $v0
}

#[test]
fn dsub_registers_positive_result() {
    let mut datapath = MipsDatapath::default();

    // dsub rd, rs, rt
    // dsub $s5, $s4, $s3
    // GPR[rd] <- GPR[rs] - GPR[rt]
    // GPR[$s5] <- GPR[$s4] - GPR[$s3]
    // GPR[19] <- GPR[18] - GPR[17]
    //                      SPECIAL rs    rt    rd    0     funct
    //                              $s4   $s3   $s5         DSUB
    //                              18    17    19
    let instruction: u32 = 0b000000_10010_10001_10011_00000_101110;

    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    // Assume registers $s3 and $s4 contain numbers larger than 32 bits,
    // but smaller than 64 bits.
    datapath.registers.gpr[18] = 4_833_323_886_298_794; // $s4
    datapath.registers.gpr[17] = 163_643_849_115_304; // $s3

    datapath.execute_instruction();

    assert_eq!(datapath.registers.gpr[19], 4_669_680_037_183_490); // $s5
}

#[test]
fn dmul_positive_result() {
    let mut datapath = MipsDatapath::default();

    // dmul rd, rs, rt
    // dmul $a0, $t8, $t9
    // dmul 4, 24, 25
    // GPR[rd] <- lo_doubleword(multiply.signed(GPR[rs] * GPR[rt]))
    //                      opcode  rs    rt    rd          funct
    //                      SPECIAL $t8   $t9   $a0   DMUL  SOP34
    //                              24    25    4
    let instruction: u32 = 0b000000_11000_11001_00100_00010_011100;

    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    // Assume register $t8 contains a number larger than 32 bits,
    // but smaller than 64 bits.
    datapath.registers.gpr[24] = 5_861_036_283_017; // $t8
    datapath.registers.gpr[25] = 5; // $t9

    datapath.execute_instruction();

    assert_eq!(datapath.registers.gpr[4], 29_305_181_415_085); // $a0
}

#[test]
fn dmul_negative_result() {
    let mut datapath = MipsDatapath::default();

    // dmul rd, rs, rt
    // dmul $s7, $t7, $t6
    // dmul 23, 15, 14
    // GPR[rd] <- lo_doubleword(multiply.signed(GPR[rs] * GPR[rt]))
    //                      opcode  rs    rt    rd          funct
    //                      SPECIAL $t7   $t6   $s7   DMUL  SOP34
    //                              15    14    23
    let instruction: u32 = 0b000000_01111_01110_10111_00010_011100;

    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    // Assume register $t7 contains a number larger than 32 bits,
    // but smaller than 64 bits.
    datapath.registers.gpr[15] = 363_251_152_978_005; // $t7
    datapath.registers.gpr[14] = -19_i64 as u64; // $t6

    datapath.execute_instruction();

    assert_eq!(datapath.registers.gpr[23] as i64, -6_901_771_906_582_095); // $s7
}

#[test]
fn dmul_result_truncate() {
    let mut datapath = MipsDatapath::default();

    // dmul rd, rs, rt
    // dmul $s2, $s4, $s3
    // dmul 18, 20, 19
    // GPR[rd] <- lo_doubleword(multiply.signed(GPR[rs] * GPR[rt]))
    //                      opcode  rs    rt    rd          funct
    //                      SPECIAL $s4   $s3   $s2   DMUL  SOP34
    //                              20    19    18
    let instruction: u32 = 0b000000_10100_10011_10010_00010_011100;

    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    // Assume registers $s4 and $s3 contain numbers larger than 32 bits,
    // but smaller than 64 bits.
    datapath.registers.gpr[20] = 191_893_548_893_556_856; // $s4
    datapath.registers.gpr[19] = 2_799_316_838_897; // $s3

    datapath.execute_instruction();

    // The result, 537,170,842,693,438,490,068,661,827,832, is too large for
    // a 64-bit integer.
    // (110 11000111 10110001 01001110 10000100 [00110100 01101011 00001011 00010110 11011010 00010011 11111000 11111000])
    // The result should instead truncate to the lower 64 bits.
    assert_eq!(datapath.registers.gpr[18], 3_777_124_905_256_220_920); // $s2
}

#[test]
fn ddiv_positive_result() {
    let mut datapath = MipsDatapath::default();

    // ddiv rd, rs, rt
    // ddiv $s0, $s1, $s2
    // ddiv 16, 17, 18
    // GPR[rd] <- divide.signed(GPR[rs], GPR[rt])
    //                      opcode  rs    rt    rd          funct
    //                      SPECIAL $s1   $s2   $s0   DDIV  SOP36
    //                              17    18    16
    let instruction: u32 = 0b000000_10001_10010_10000_00010_011110;

    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    // Assume register $s1 contains a number larger than 32 bits,
    // but smaller than 64 bits.
    datapath.registers.gpr[17] = 1_284_064_531_192; // $s1
    datapath.registers.gpr[18] = 7; // $s2

    datapath.execute_instruction();

    // While the actual result is 183,437,790,170.285714....
    // the decimal portion is truncated.
    assert_eq!(datapath.registers.gpr[16], 183_437_790_170); // $s0
}

#[test]
fn ddiv_negative_result() {
    let mut datapath = MipsDatapath::default();

    // ddiv rd, rs, rt
    // ddiv $a3, $a2, $a1
    // ddiv 7, 6, 5
    // GPR[rd] <- divide.signed(GPR[rs], GPR[rt])
    //                      opcode  rs    rt    rd          funct
    //                      SPECIAL $a2   $a1   $a3   DDIV  SOP36
    //                              6     5     7
    let instruction: u32 = 0b000000_00110_00101_00111_00010_011110;

    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    // Assume register $a2 contains a number larger than 32 bits,
    // but smaller than 64 bits.
    datapath.registers.gpr[6] = -6_245_352_518_120_328_878_i64 as u64; // $a2
    datapath.registers.gpr[5] = 123; // $a1

    datapath.execute_instruction();

    // While the actual result is -50,775,223,724,555,519.333333....
    // the decimal portion is truncated.
    assert_eq!(datapath.registers.gpr[7] as i64, -50_775_223_724_555_519); // $a3
}

pub mod dahi_dati {
    use super::*;

    #[test]
    fn dahi_basic_add() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // dahi rs, immediate
        // dahi $a0, 1
        // GPR[rs] <- GPR[rs] + sign_extend(immediate << 32)
        // GPR[4] <- GPR[4] + sign_extend(1 << 32)
        //                       op     rs    rt     immediate
        //                       REGIMM $a0   DAHI   1
        let instruction: u32 = 0b000001_00100_00110_0000000000000001;
        datapath.memory.store_word(0, instruction)?;

        datapath.registers.gpr[4] = 0xABCD; // $a0

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[4], 0x0000_0001_0000_ABCD);

        Ok(())
    }
}

pub mod load_word {
    use super::*;
    #[test]
    fn lw_zero_offset_test() {
        // for this test the lw instruction will load itself from
        // memory
        let mut datapath = MipsDatapath::default();

        //                        lw     $t0   $s0      offset = 0
        let instruction: u32 = 0b100011_01000_10000_0000000000000000;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");
        datapath.execute_instruction();
        assert_eq!(datapath.registers.gpr[16], instruction as u64);
    }

    #[test]
    fn lw_offset_at_4_test() {
        // For this test the lw instruction will load 0x4 from memory
        // by using the offset address plus zero
        let mut datapath = MipsDatapath::default();

        //                        lw     $t0   $s0      offset = 4
        let instruction: u32 = 0b100011_01000_10000_0000000000000100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        // place data at address
        datapath
            .memory
            .store_word(0b100, 0x10000)
            .expect("failed to store test data");

        datapath.registers.gpr[8] = 0;
        datapath.execute_instruction();
        assert_eq!(datapath.registers.gpr[16], 0x10000);
    }

    #[test]
    fn lw_gpr_8_at_4_offset_at_0_test() {
        // for this test the lw instruction will load 0x4 from memory
        // by using (offset = 0) + (gpr[8] = 4)
        let mut datapath = MipsDatapath::default();

        //                        lw     $t0   $s0      offset = 0
        let instruction: u32 = 0b100011_01000_10000_0000000000000000;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        // place data at address
        datapath
            .memory
            .store_word(0b100, 0x10000)
            .expect("failed to store test data");

        datapath.registers.gpr[8] = 4;
        datapath.execute_instruction();
        assert_eq!(datapath.registers.gpr[16], 0x10000);
    }

    #[test]
    fn lw_gpr_8_at_4_offset_at_4_test() {
        // for this test the lw instruction will load 0x8 from memory
        // by adding the offset to gpr[8]
        let mut datapath = MipsDatapath::default();

        //                        lw     $t0   $s0      offset = 0
        let instruction: u32 = 0b100011_01000_10000_0000000000000100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        // place data at address
        datapath
            .memory
            .store_word(0b1000, 0x10000)
            .expect("failed to store test data");

        datapath.registers.gpr[8] = 4;
        datapath.execute_instruction();
        assert_eq!(datapath.registers.gpr[16], 0x10000);
    }

    #[test]
    fn lw_gpr_8_at_12_offset_at_neg_4_test() {
        // for this test the lw instruction will load 0x8 from memory
        // by adding the offset to gpr[8]
        let mut datapath = MipsDatapath::default();

        //                        lw     $t0   $s0      offset = 0
        let instruction: u32 = 0b100011_01000_10000_1111111111111100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        // place data at address
        datapath
            .memory
            .store_word(0b1000, 0x10000)
            .expect("failed to store test data");

        datapath.registers.gpr[8] = 12;
        datapath.execute_instruction();
        assert_eq!(datapath.registers.gpr[16], 0x10000);
    }
}

pub mod load_upper_imm {
    use super::*;

    #[test]
    fn basic_load_upper_imm_test() {
        let mut datapath = MipsDatapath::default();

        //                        lui    $t0   $s0      offset = 42
        let instruction: u32 = 0b001111_01000_10000_0010101010101010;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");
        datapath.execute_instruction();

        let t = datapath.registers[GpRegisterType::S0];
        assert_eq!(t, 0x2aaa_0000);
    }

    #[test]
    fn sign_extend_load_upper_imm_test() {
        let mut datapath = MipsDatapath::default();

        //                        lui    $t0   $s0      offset = 42
        let instruction: u32 = 0b001111_01000_10000_1010101010101010;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");
        datapath.execute_instruction();

        let t = datapath.registers[GpRegisterType::S0];
        assert_eq!(t, 0xffff_ffff_aaaa_0000);
    }
}
pub mod store_word {
    use super::*;
    #[test]
    fn sw_zero_offset_test() {
        let mut datapath = MipsDatapath::default();

        //                        lw     $t0   $s0      offset = 0
        let instruction: u32 = 0b101011_01000_10000_0000000000000000;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");
        datapath.execute_instruction();

        let t = datapath
            .memory
            .load_word(0)
            .expect("Could not load from memory");
        assert_eq!(t, 0);
    }

    #[test]
    fn sw_offset_at_4_test() {
        let mut datapath = MipsDatapath::default();

        //                        sw     $t0   $s0      offset = 4
        let instruction: u32 = 0b101011_01000_10000_0000000000000100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.registers.gpr[8] = 0;
        datapath.registers.gpr[16] = 0xff;
        datapath.execute_instruction();

        let t = datapath
            .memory
            .load_word(4)
            .expect("Could not load from memory");
        assert_eq!(t, 0xff);
    }

    #[test]
    fn lw_gpr_8_at_4_offset_at_4_test() {
        let mut datapath = MipsDatapath::default();

        //                        sw     $t0   $s0      offset = 4
        let instruction: u32 = 0b101011_01000_10000_0000000000000100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.registers.gpr[8] = 4;
        datapath.registers.gpr[16] = 0xff;
        datapath.execute_instruction();

        let t = datapath
            .memory
            .load_word(8)
            .expect("Could not load from memory");
        assert_eq!(t, 0xff);
    }

    #[test]
    fn lw_gpr_8_at_4_offset_at_neg_4_test() {
        let mut datapath = MipsDatapath::default();

        //                        sw     $t0   $s0      offset = -4
        let instruction: u32 = 0b101011_01000_10000_1111111111111100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.registers.gpr[8] = 12;
        datapath.registers.gpr[16] = 0xff;
        datapath.execute_instruction();

        let t = datapath
            .memory
            .load_word(8)
            .expect("Could not load from memory");
        assert_eq!(t, 0xff);
    }
}

pub mod coprocessor {
    use crate::emulation_core::datapath::Datapath;
    use crate::emulation_core::mips::datapath::MipsDatapath;

    #[test]
    pub fn add_float_single_precision() {
        let mut datapath = MipsDatapath::default();

        // add.s fd, fs, ft
        // add.s $f2, $f1, $f0
        // FPR[2] = FPR[1] + FPR[0]
        //                       COP1   fmt   ft    fs    fd    function
        //                              s     $f0   $f1   $f2   ADD
        let instruction: u32 = 0b010001_10000_00000_00001_00010_000000;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.coprocessor.fpr[0] = f32::to_bits(0.25f32) as u64;
        datapath.coprocessor.fpr[1] = f32::to_bits(0.5f32) as u64;

        datapath.execute_instruction();

        // The result should be 0.75, represented in a 32-bit value as per the
        // IEEE 754 single-precision floating-point specification.
        assert_eq!(f32::from_bits(datapath.coprocessor.fpr[2] as u32), 0.75);
    }

    #[test]
    pub fn add_float_double_precision() {
        let mut datapath = MipsDatapath::default();

        // add.d fd, fs, ft
        // add.d $f2, $f1, $f0
        // FPR[2] = FPR[1] + FPR[0]
        //                       COP1   fmt   ft    fs    fd    function
        //                              d     $f0   $f1   $f2   ADD
        let instruction: u32 = 0b010001_10001_00000_00001_00010_000000;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.coprocessor.fpr[0] = f64::to_bits(123.125);
        datapath.coprocessor.fpr[1] = f64::to_bits(0.5);

        datapath.execute_instruction();

        // The result should be 123.625, represented in a 64-bit value as per the
        // IEEE 754 double-precision floating-point specification.
        assert_eq!(f64::from_bits(datapath.coprocessor.fpr[2]), 123.625);
    }

    #[test]
    pub fn sub_float_single_precision() {
        let mut datapath = MipsDatapath::default();

        // sub.s fd, fs, ft
        // sub.s $f2, $f1, $f0
        // FPR[fd] = FPR[fs] - FPR[ft]
        // FPR[2] = FPR[1] - FPR[0]
        //                       COP1   fmt   ft    fs    fd    function
        //                              s     $f0   $f1   $f2   SUB
        let instruction: u32 = 0b010001_10000_00000_00001_00010_000001;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.coprocessor.fpr[0] = f32::to_bits(5.625f32) as u64;
        datapath.coprocessor.fpr[1] = f32::to_bits(3.125f32) as u64;

        datapath.execute_instruction();

        assert_eq!(f32::from_bits(datapath.coprocessor.fpr[2] as u32), -2.5);
    }

    #[test]
    pub fn sub_float_double_precision() {
        let mut datapath = MipsDatapath::default();

        // sub.d fd, fs, ft
        // sub.d $f2, $f1, $f0
        // FPR[fd] = FPR[fs] - FPR[ft]
        // FPR[2] = FPR[1] - FPR[0]
        //                       COP1   fmt   ft    fs    fd    function
        //                              d     $f0   $f1   $f2   SUB
        let instruction: u32 = 0b010001_10001_00000_00001_00010_000001;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.coprocessor.fpr[0] = f64::to_bits(438.125);
        datapath.coprocessor.fpr[1] = f64::to_bits(98765.5);

        datapath.execute_instruction();

        assert_eq!(f64::from_bits(datapath.coprocessor.fpr[2]), 98327.375);
    }

    #[test]
    pub fn mul_float_single_precision() {
        let mut datapath = MipsDatapath::default();

        // mul.s fd, fs, ft
        // mul.s $f9, $f5, $f4
        // FPR[fd] = FPR[fs] * FPR[ft]
        // FPR[9] = FPR[5] * FPR[4]
        //                       COP1   fmt   ft    fs    fd    function
        //                              s     $f4   $f5   $f9   MUL
        let instruction: u32 = 0b010001_10000_00100_00101_01001_000010;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.coprocessor.fpr[5] = f32::to_bits(24.5f32) as u64;
        datapath.coprocessor.fpr[4] = f32::to_bits(0.5f32) as u64;

        datapath.execute_instruction();

        assert_eq!(f32::from_bits(datapath.coprocessor.fpr[9] as u32), 12.25f32);
    }

    #[test]
    pub fn mul_float_double_precision() {
        let mut datapath = MipsDatapath::default();

        // mul.d fd, fs, ft
        // mul.d $f4, $f6, $f9
        // FPR[fd] = FPR[fs] * FPR[ft]
        // FPR[4] = FPR[6] * FPR[9]
        //                       COP1   fmt   ft    fs    fd    function
        //                              d     $f9   $f6   $f4   MUL
        let instruction: u32 = 0b010001_10001_01001_00110_00100_000010;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.coprocessor.fpr[6] = f64::to_bits(-150.0625);
        datapath.coprocessor.fpr[9] = f64::to_bits(9.5);

        datapath.execute_instruction();

        assert_eq!(f64::from_bits(datapath.coprocessor.fpr[4]), -1425.59375);
    }

    #[test]
    pub fn div_float_single_precision() {
        let mut datapath = MipsDatapath::default();

        // div.s fd, fs, ft
        // div.s $f15, $f16, $f17
        // FPR[fd] = FPR[fs] / FPR[ft]
        // FPR[15] = FPR[16] / FPR[17]
        //                       COP1   fmt   ft    fs    fd    function
        //                              s     $f17  $f16  $f15  DIV
        let instruction: u32 = 0b010001_10000_10001_10000_01111_000011;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.coprocessor.fpr[16] = f32::to_bits(901f32) as u64;
        datapath.coprocessor.fpr[17] = f32::to_bits(2f32) as u64;

        datapath.execute_instruction();

        assert_eq!(
            f32::from_bits(datapath.coprocessor.fpr[15] as u32),
            450.5f32
        );
    }

    #[test]
    pub fn div_float_double_precision() {
        let mut datapath = MipsDatapath::default();

        // div.d fd, fs, ft
        // div.d $f1, $f10, $f20
        // FPR[fd] = FPR[fs] / FPR[ft]
        // FPR[1] = FPR[10] / FPR[20]
        //                       COP1   fmt   ft    fs    fd    function
        //                              d     $f20  $f10  $f1   DIV
        let instruction: u32 = 0b010001_10001_10100_01010_00001_000011;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.coprocessor.fpr[10] = f64::to_bits(95405.375);
        datapath.coprocessor.fpr[20] = f64::to_bits(2.0);

        datapath.execute_instruction();

        assert_eq!(f64::from_bits(datapath.coprocessor.fpr[1]), 47702.6875);
    }

    #[test]
    pub fn swc1_basic_store_no_offset() {
        let mut datapath = MipsDatapath::default();

        // swc1 ft, offset(base)
        // swc1 $f3, 0($s1)
        // memory[GPR[base] + offset] <- FPR[ft]
        // memory[GPR[17] + 0] <- FPR[3]
        //                       SWC1   base  ft    offset
        //                              $s1   $f3   0
        let instruction: u32 = 0b111001_10001_00011_0000000000000000;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.registers.gpr[17] = 1028; // $s1
        datapath.coprocessor.fpr[3] = f32::to_bits(1.0625f32) as u64;

        datapath.execute_instruction();

        // The single-precision float 1.0625 should be stored at address 1028.
        assert_eq!(
            f32::from_bits(datapath.memory.load_word(1028).unwrap()),
            1.0625f32
        );
    }

    #[test]
    pub fn swc1_basic_store_with_offset() {
        let mut datapath = MipsDatapath::default();

        // swc1 ft, offset(base)
        // swc1 $f5, 32($s0)
        // memory[GPR[base] + offset] <- FPR[ft]
        // memory[GPR[16] + 32] <- FPR[5]
        //                       SWC1   base  ft    offset
        //                              $s0   $f5   32
        let instruction: u32 = 0b111001_10000_00101_0000000000100000;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.registers.gpr[16] = 2000; // $s0
        datapath.coprocessor.fpr[5] = f32::to_bits(3.5f32) as u64;

        datapath.execute_instruction();

        // The single-precision float 3.5 should be stored at address 2032.
        assert_eq!(
            f32::from_bits(datapath.memory.load_word(2032).unwrap()),
            3.5f32
        );
    }

    #[test]
    pub fn swc1_basic_store_64_bit_cutoff() {
        // This test ensures that if there is 64-bit data in a floating-point
        // register, only the bottom 32 bits are stored in memory with this
        // instruction.

        let mut datapath = MipsDatapath::default();

        // swc1 ft, offset(base)
        // swc1 $f0, 0($s2)
        // memory[GPR[base] + offset] <- FPR[ft]
        // memory[GPR[18] + 0] <- FPR[0]
        //                       SWC1   base  ft    offset
        //                              $s2   $f0   0
        let instruction: u32 = 0b111001_10010_00000_0000000000000000;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.registers.gpr[18] = 1000; // $s2
        datapath.coprocessor.fpr[0] = f64::to_bits(9853114.625);

        datapath.execute_instruction();

        // The double-precision float 9853114.625 is represented in hexadecimal as
        // 4162 CB17 5400 0000. When storing the 32-bit word, the bottom 32 bits
        // should be stored, in this case meaning 5400 0000 in hexadecimal.
        assert_eq!(datapath.memory.load_word(1000).unwrap(), 0x5400_0000);
    }

    #[test]
    fn lwc1_basic_load_no_offset() {
        let mut datapath = MipsDatapath::default();

        // lwc1 ft, offset(base)
        // lwc1 $f10, 0($t0)
        // FPR[ft] <- memory[GPR[base] + offset]
        // FPR[10] <- memory[GPR[8] + 0]
        //                       LWC1   base  ft    offset
        //                              $t0   $f10  0
        let instruction: u32 = 0b110001_01000_01010_0000000000000000;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.registers.gpr[8] = 500; // $t0

        // Data is put into memory this way (rather than using load_word()) to
        // demonstrate no reliance on API calls.
        let data = f32::to_bits(413.125f32).to_be_bytes();
        for (i, byte) in data.iter().enumerate() {
            datapath.memory.memory[500 + i] = *byte;
        }

        datapath.execute_instruction();

        assert_eq!(
            f32::from_bits(datapath.coprocessor.fpr[10] as u32),
            413.125f32
        );
    }

    #[test]
    fn lwc1_basic_load_with_offset() {
        let mut datapath = MipsDatapath::default();

        // lwc1 ft, offset(base)
        // lwc1 $f11, 200($t1)
        // FPR[ft] <- memory[GPR[base] + offset]
        // FPR[11] <- memory[GPR[9] + 200]
        //                       LWC1   base  ft    offset
        //                              $t1   $f11  200
        let instruction: u32 = 0b110001_01001_01011_0000000011001000;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.registers.gpr[9] = 1000; // $t1

        // Data is put into memory this way (rather than using load_word()) to
        // demonstrate no reliance on API calls.
        let data = f32::to_bits(6.1875f32).to_be_bytes();
        for (i, byte) in data.iter().enumerate() {
            datapath.memory.memory[1200 + i] = *byte;
        }

        datapath.execute_instruction();

        assert_eq!(
            f32::from_bits(datapath.coprocessor.fpr[11] as u32),
            6.1875f32
        );
    }
}
