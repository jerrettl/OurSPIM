//! Abstract representation of an instruction.

use super::constants::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RType {
    pub op: u8,
    pub rs: u8,
    pub rt: u8,
    pub rd: u8,
    pub shamt: u8,
    pub funct: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RTypeSpecial {
    pub op: u8,
    pub rs: u8,
    pub rt: u8,
    pub rd: u8,
    pub shamt: u8, // Maybe should be called "Hint"
    pub funct: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IType {
    pub op: u8,
    pub rs: u8,
    pub rt: u8,
    pub immediate: u16,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JType {
    pub op: u8,
    pub addr: u32,
}

/// Syscall ("System Call") Instruction
///
/// ```text
/// 31              26   25                                           6   5              0
/// ┌──────────────────┬────────────────────────────────────────────────┬──────────────────┐
/// │ opcode = SPECIAL │                     code                       │  funct = SYSCALL │
/// │      000000      │                                                │      001100      │
/// └──────────────────┴────────────────────────────────────────────────┴──────────────────┘
///         6                                 20                                  6
/// ```
///
/// - opcode: SPECIAL (`000000`)
/// - code: Available for use as software parameters.
/// - funct: SYSCALL (`001100`)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SyscallType {
    pub op: u8,
    pub code: u32,
    pub funct: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FpuRType {
    pub op: u8,
    pub fmt: u8,
    pub ft: u8,
    pub fs: u8,
    pub fd: u8,
    pub function: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FpuIType {
    pub op: u8,
    pub base: u8,
    pub ft: u8,
    pub offset: u16,
}

/// Register-Immediate FPU Instruction
///
/// Used for instructions that transfer data between the main processor
/// and the floating-point coprocessor.
///
/// ```text
/// 31           26   25       21   20       16   15       11   10                    0
/// ┌───────────────┬─────────────┬─────────────┬─────────────┬─────────────────────────┐
/// │ opcode = COP1 │     sub     │     rt      │     fs      │            0            │
/// │    010001     │             │             │             │                         │
/// └───────────────┴─────────────┴─────────────┴─────────────┴─────────────────────────┘
///         6              5             5             5                   11
/// ```
///
/// - opcode: COP1 (`010001`)
/// - sub: Operation subcode field for COP1 register immediate-mode instructions.
/// - rt: CPU register - can be either source or destination.
/// - fs: FPU register - can be either source or destination.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FpuRegImmType {
    pub op: u8,
    pub sub: u8,
    pub rt: u8,
    pub fs: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FpuCompareType {
    pub op: u8,
    pub fmt: u8,
    pub ft: u8,
    pub fs: u8,
    pub cc: u8,
    pub function: u8,
}

/// Branching FPU Instruction
///
/// Used for instructions that branch based on the floating-point coprocessor.
///
/// ```text
/// 31           26   25       21   20    18   17   16   15                              0
/// ┌───────────────┬─────────────┬──────────┬────┬────┬──────────────────────────────────┐
/// │ opcode = COP1 │    BCC1     │    cc    │ nd │ tf │              offset              │
/// │    010001     │             │          │    │    │                                  │
/// └───────────────┴─────────────┴──────────┴────┴────┴──────────────────────────────────┘
///        6              5            3       1    1                   16
/// ```
///
/// - opcode: COP1 (`010001`)
/// - BCC1: "Branch conditional coprocessor 1" subcode.
/// - cc: Branch instruction condition code.
/// - nd: Nullify delay. If set, the branch is Likely, and the delay slot instruction is not executed. (Not necessary for this project.)
/// - tf: True/False. The type of condition for a comparison.
/// - offset: Signed offset field used in address calculations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FpuBranchType {
    pub op: u8,
    pub bcc1: u8,
    pub cc: u8,
    pub nd: u8,
    pub tf: u8,
    pub offset: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    RType(RType),
    IType(IType),
    JType(JType),
    SyscallType(SyscallType),
    RTypeSpecial(RTypeSpecial),
    FpuRType(FpuRType),
    FpuIType(FpuIType),
    FpuRegImmType(FpuRegImmType),
    FpuCompareType(FpuCompareType),
    FpuBranchType(FpuBranchType),
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction::RType(RType::default())
    }
}

impl TryFrom<u32> for Instruction {
    type Error = String;

    /// Based on the opcode, convert a binary instruction into a struct representation.
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let op = (value >> 26) as u8;
        match op {
            // R-type instructions:
            // add, sub, mul, div
            // dadd, dsub, dmul, ddiv
            // daddu, dsubu, dmulu, ddivu
            //
            // R-type-like instructions:
            // These are instructions that are very R-type-like, but require
            // some custom flag options that make them unique.
            // jalr
            //
            // Syscall instructions:
            // Any syscall
            //
            OPCODE_SPECIAL => {
                let funct = (value & 0x3F) as u8;

                match funct {
                    FUNCT_SYSCALL => Ok(Instruction::SyscallType(SyscallType {
                        op: ((value >> 26) & 0x3F) as u8,
                        code: ((value >> 6) & 0xFFFFF),
                        funct: (value & 0x3F) as u8,
                    })),
                    FUNCT_JALR => Ok(Instruction::RTypeSpecial(RTypeSpecial {
                        op: ((value >> 26) & 0x3F) as u8,
                        rs: ((value >> 21) & 0x1F) as u8,
                        rt: ((value >> 16) & 0x1F) as u8,
                        rd: ((value >> 11) & 0x1F) as u8,
                        shamt: ((value >> 6) & 0x1F) as u8,
                        funct: (value & 0x3F) as u8,
                    })),
                    // Simple R-Type instructions:
                    _ => Ok(Instruction::RType(RType {
                        op: ((value >> 26) & 0x3F) as u8,
                        rs: ((value >> 21) & 0x1F) as u8,
                        rt: ((value >> 16) & 0x1F) as u8,
                        rd: ((value >> 11) & 0x1F) as u8,
                        shamt: ((value >> 6) & 0x1F) as u8,
                        funct: (value & 0x3F) as u8,
                    })),
                }
            }

            // COP1 (coprocessor 1)
            OPCODE_COP1 => {
                // First break down the instruction by its `fmt`/`rs`/`bcc1` field.
                // Also called `sub` (operation subcode) field.
                let sub = ((value >> 21) & 0x1F) as u8;

                match sub {
                    // If it is the "s" or "d" fmts, use the `function` field.
                    FMT_SINGLE | FMT_DOUBLE => {
                        let function = (value & 0x3F) as u8;
                        match function {
                            // add.fmt, sub.fmt, mul.fmt, div.fmt
                            FUNCTION_ADD | FUNCTION_SUB | FUNCTION_MUL | FUNCTION_DIV => {
                                Ok(Instruction::FpuRType(FpuRType {
                                    op: ((value >> 26) & 0x3F) as u8,
                                    fmt: ((value >> 21) & 0x1F) as u8,
                                    ft: ((value >> 16) & 0x1F) as u8,
                                    fs: ((value >> 11) & 0x1F) as u8,
                                    fd: ((value >> 6) & 0x1F) as u8,
                                    function: (value & 0x3F) as u8,
                                }))
                            }
                            // Comparison instructions:
                            // c.eq.fmt, c.lt.fmt, c.le.fmt, c.ngt.fmt, c.nge.fmt
                            FUNCTION_C_EQ | FUNCTION_C_LT | FUNCTION_C_NGE | FUNCTION_C_LE
                            | FUNCTION_C_NGT => Ok(Instruction::FpuCompareType(FpuCompareType {
                                op: ((value >> 26) & 0x3F) as u8,
                                fmt: ((value >> 21) & 0x1F) as u8,
                                ft: ((value >> 16) & 0x1F) as u8,
                                fs: ((value >> 11) & 0x1F) as u8,
                                cc: ((value >> 8) & 0x7) as u8,
                                function: (value & 0x3F) as u8,
                            })),
                            _ => Err(format!(
                                "function `{function}` not supported for opcode {op}"
                            )),
                        }
                    }

                    // Move word to coprocessor 1 (mtc1)
                    // Move doubleword to coprocessor 1 (dmtc1)
                    // Move word from coprocessor 1 (mfc1)
                    // Move doubleword from coprocessor 1 (dmfc1)
                    SUB_MT | SUB_DMT | SUB_MF | SUB_DMF => {
                        Ok(Instruction::FpuRegImmType(FpuRegImmType {
                            op: ((value >> 26) & 0x3F) as u8,
                            sub: ((value >> 21) & 0x1F) as u8,
                            rt: ((value >> 16) & 0x1F) as u8,
                            fs: ((value >> 11) & 0x1F) as u8,
                        }))
                    }

                    _ => Err(format!("sub code `{sub}` not supported for opcode {op}")),
                }
            }

            // I-Type instructions:
            OPCODE_ADDI | OPCODE_ADDIU | OPCODE_DADDI | OPCODE_DADDIU | OPCODE_LW | OPCODE_SW
            | OPCODE_LUI | OPCODE_ORI | OPCODE_ANDI | OPCODE_REGIMM | OPCODE_BEQ | OPCODE_BNE => {
                Ok(Instruction::IType(IType {
                    op: ((value >> 26) & 0x3F) as u8,
                    rs: ((value >> 21) & 0x1F) as u8,
                    rt: ((value >> 16) & 0x1F) as u8,
                    immediate: (value & 0xFFFF) as u16,
                }))
            }

            // Store/load word to Coprocessor 1
            OPCODE_SWC1 | OPCODE_LWC1 => Ok(Instruction::FpuIType(FpuIType {
                op: ((value >> 26) & 0x3F) as u8,
                base: ((value >> 21) & 0x1F) as u8,
                ft: ((value >> 16) & 0x1F) as u8,
                offset: (value & 0xFFFF) as u16,
            })),

            OPCODE_J | OPCODE_JAL => Ok(Instruction::JType(JType {
                op: ((value >> 26) & 0x3F) as u8,
                addr: value & 0x03ffffff,
            })),

            _ => Err(format!("opcode `{op}` not supported")),
        }
    }
}
