//! Register structure and API.

use std::ops::{Index, IndexMut};
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Default, PartialEq)]
pub struct Registers {
    pub pc: u64,
    pub gpr: [u64; 32],
    pub fpr: [u64; 32],
    pub cc: u64,
}

#[derive(EnumString)]
#[strum(ascii_case_insensitive)]
pub enum RegisterType {
    Zero,
    At,
    V0,
    V1,
    A0,
    A1,
    A2,
    A3,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    T8,
    T9,
    K0,
    K1,
    Gp,
    Sp,
    Fp,
    Ra,
    Cc,
    Pc,
    F0,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
}

<<<<<<< Updated upstream
impl Index<&str> for Registers {
=======
impl ToString for GpRegisters {
    fn to_string(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("PC = {}\n", self.pc));

        let gpr_registers = self
            .gpr
            .iter()
            .enumerate()
            .map(|(i, inst)| format!("gpr[{}] = {}", i, inst))
            .collect::<Vec<String>>()
            .join("\n");
        output.push_str(&gpr_registers);

        output
    }
}

impl Index<&str> for GpRegisters {
>>>>>>> Stashed changes
    type Output = u64;

    // Convert string to the corresponding RegistersEnum value and use this to index.
    // If this is an invalid string, no enum will be returned, causing a panic as desired.
    fn index(&self, index: &str) -> &Self::Output {
        match RegisterType::from_str(index) {
            Ok(register) => &self[register],
            _ => panic!("{} is not a valid register", index),
        }
    }
}

impl IndexMut<&str> for Registers {
    // Convert string to the corresponding RegistersEnum value and use this to index.
    // If this is an invalid string, no enum will be returned, causing a panic as desired.
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        match RegisterType::from_str(index) {
            Ok(register) => &mut self[register],
            _ => panic!("{} is not a valid register", index),
        }
    }
}

impl Index<RegisterType> for Registers {
    type Output = u64;

    fn index(&self, index: RegisterType) -> &Self::Output {
        match index {
            RegisterType::Pc => &self.pc,
            RegisterType::Zero => &self.gpr[0],
            RegisterType::At => &self.gpr[1],
            RegisterType::V0 => &self.gpr[2],
            RegisterType::V1 => &self.gpr[3],
            RegisterType::A0 => &self.gpr[4],
            RegisterType::A1 => &self.gpr[5],
            RegisterType::A2 => &self.gpr[6],
            RegisterType::A3 => &self.gpr[7],
            RegisterType::T0 => &self.gpr[8],
            RegisterType::T1 => &self.gpr[9],
            RegisterType::T2 => &self.gpr[10],
            RegisterType::T3 => &self.gpr[11],
            RegisterType::T4 => &self.gpr[12],
            RegisterType::T5 => &self.gpr[13],
            RegisterType::T6 => &self.gpr[14],
            RegisterType::T7 => &self.gpr[15],
            RegisterType::S0 => &self.gpr[16],
            RegisterType::S1 => &self.gpr[17],
            RegisterType::S2 => &self.gpr[18],
            RegisterType::S3 => &self.gpr[19],
            RegisterType::S4 => &self.gpr[20],
            RegisterType::S5 => &self.gpr[21],
            RegisterType::S6 => &self.gpr[22],
            RegisterType::S7 => &self.gpr[23],
            RegisterType::T8 => &self.gpr[24],
            RegisterType::T9 => &self.gpr[25],
            RegisterType::K0 => &self.gpr[26],
            RegisterType::K1 => &self.gpr[27],
            RegisterType::Gp => &self.gpr[28],
            RegisterType::Sp => &self.gpr[29],
            RegisterType::Fp => &self.gpr[30],
            RegisterType::Ra => &self.gpr[31],
            RegisterType::Cc => &self.cc,
            RegisterType::F0 => &self.fpr[0],
            RegisterType::F1 => &self.fpr[1],
            RegisterType::F2 => &self.fpr[2],
            RegisterType::F3 => &self.fpr[3],
            RegisterType::F4 => &self.fpr[4],
            RegisterType::F5 => &self.fpr[5],
            RegisterType::F6 => &self.fpr[6],
            RegisterType::F7 => &self.fpr[7],
            RegisterType::F8 => &self.fpr[8],
            RegisterType::F9 => &self.fpr[9],
            RegisterType::F10 => &self.fpr[10],
            RegisterType::F11 => &self.fpr[11],
            RegisterType::F12 => &self.fpr[12],
            RegisterType::F13 => &self.fpr[13],
            RegisterType::F14 => &self.fpr[14],
            RegisterType::F15 => &self.fpr[15],
            RegisterType::F16 => &self.fpr[16],
            RegisterType::F17 => &self.fpr[17],
            RegisterType::F18 => &self.fpr[18],
            RegisterType::F19 => &self.fpr[19],
            RegisterType::F20 => &self.fpr[20],
            RegisterType::F21 => &self.fpr[21],
            RegisterType::F22 => &self.fpr[22],
            RegisterType::F23 => &self.fpr[23],
            RegisterType::F24 => &self.fpr[24],
            RegisterType::F25 => &self.fpr[25],
            RegisterType::F26 => &self.fpr[26],
            RegisterType::F27 => &self.fpr[27],
            RegisterType::F28 => &self.fpr[28],
            RegisterType::F29 => &self.fpr[29],
            RegisterType::F30 => &self.fpr[30],
            RegisterType::F31 => &self.fpr[31],
        }
    }
}

impl IndexMut<RegisterType> for Registers {
    fn index_mut(&mut self, index: RegisterType) -> &mut Self::Output {
        match index {
            RegisterType::Pc => &mut self.pc,
            RegisterType::Zero => &mut self.gpr[0],
            RegisterType::At => &mut self.gpr[1],
            RegisterType::V0 => &mut self.gpr[2],
            RegisterType::V1 => &mut self.gpr[3],
            RegisterType::A0 => &mut self.gpr[4],
            RegisterType::A1 => &mut self.gpr[5],
            RegisterType::A2 => &mut self.gpr[6],
            RegisterType::A3 => &mut self.gpr[7],
            RegisterType::T0 => &mut self.gpr[8],
            RegisterType::T1 => &mut self.gpr[9],
            RegisterType::T2 => &mut self.gpr[10],
            RegisterType::T3 => &mut self.gpr[11],
            RegisterType::T4 => &mut self.gpr[12],
            RegisterType::T5 => &mut self.gpr[13],
            RegisterType::T6 => &mut self.gpr[14],
            RegisterType::T7 => &mut self.gpr[15],
            RegisterType::S0 => &mut self.gpr[16],
            RegisterType::S1 => &mut self.gpr[17],
            RegisterType::S2 => &mut self.gpr[18],
            RegisterType::S3 => &mut self.gpr[19],
            RegisterType::S4 => &mut self.gpr[20],
            RegisterType::S5 => &mut self.gpr[21],
            RegisterType::S6 => &mut self.gpr[22],
            RegisterType::S7 => &mut self.gpr[23],
            RegisterType::T8 => &mut self.gpr[24],
            RegisterType::T9 => &mut self.gpr[25],
            RegisterType::K0 => &mut self.gpr[26],
            RegisterType::K1 => &mut self.gpr[27],
            RegisterType::Gp => &mut self.gpr[28],
            RegisterType::Sp => &mut self.gpr[29],
            RegisterType::Fp => &mut self.gpr[30],
            RegisterType::Ra => &mut self.gpr[31],
            RegisterType::Cc => &mut self.cc,
            RegisterType::F0 => &mut self.fpr[0],
            RegisterType::F1 => &mut self.fpr[1],
            RegisterType::F2 => &mut self.fpr[2],
            RegisterType::F3 => &mut self.fpr[3],
            RegisterType::F4 => &mut self.fpr[4],
            RegisterType::F5 => &mut self.fpr[5],
            RegisterType::F6 => &mut self.fpr[6],
            RegisterType::F7 => &mut self.fpr[7],
            RegisterType::F8 => &mut self.fpr[8],
            RegisterType::F9 => &mut self.fpr[9],
            RegisterType::F10 => &mut self.fpr[10],
            RegisterType::F11 => &mut self.fpr[11],
            RegisterType::F12 => &mut self.fpr[12],
            RegisterType::F13 => &mut self.fpr[13],
            RegisterType::F14 => &mut self.fpr[14],
            RegisterType::F15 => &mut self.fpr[15],
            RegisterType::F16 => &mut self.fpr[16],
            RegisterType::F17 => &mut self.fpr[17],
            RegisterType::F18 => &mut self.fpr[18],
            RegisterType::F19 => &mut self.fpr[19],
            RegisterType::F20 => &mut self.fpr[20],
            RegisterType::F21 => &mut self.fpr[21],
            RegisterType::F22 => &mut self.fpr[22],
            RegisterType::F23 => &mut self.fpr[23],
            RegisterType::F24 => &mut self.fpr[24],
            RegisterType::F25 => &mut self.fpr[25],
            RegisterType::F26 => &mut self.fpr[26],
            RegisterType::F27 => &mut self.fpr[27],
            RegisterType::F28 => &mut self.fpr[28],
            RegisterType::F29 => &mut self.fpr[29],
            RegisterType::F30 => &mut self.fpr[30],
            RegisterType::F31 => &mut self.fpr[31],
        }
    }
}
