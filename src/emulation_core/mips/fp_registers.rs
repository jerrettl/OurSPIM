//! Register structure and API.

use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

/// Collection of general-purpose registers used by the datapath.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FpRegisters {
    pub fpr: [u64; 32],
}

/// Specifies all of the valid registers accessible in an instance
/// of [`FpRegisters`].
#[derive(Clone, Copy, Debug, Display, EnumIter, EnumString, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "lowercase")]
pub enum FpRegisterType {
    F0 = 0,
    F1 = 1,
    F2 = 2,
    F3 = 3,
    F4 = 4,
    F5 = 5,
    F6 = 6,
    F7 = 7,
    F8 = 8,
    F9 = 9,
    F10 = 10,
    F11 = 11,
    F12 = 12,
    F13 = 13,
    F14 = 14,
    F15 = 15,
    F16 = 16,
    F17 = 17,
    F18 = 18,
    F19 = 19,
    F20 = 20,
    F21 = 21,
    F22 = 22,
    F23 = 23,
    F24 = 24,
    F25 = 25,
    F26 = 26,
    F27 = 27,
    F28 = 28,
    F29 = 29,
    F30 = 30,
    F31 = 31,
}

impl FpRegisterType {
    pub fn get_fpr_name(&self) -> String {
        format!("f{}", *self as u32)
    }
    pub fn is_valid_register_value(&self, _value: u64) -> bool {
        true
    }
}

impl ToString for FpRegisters {
    fn to_string(&self) -> String {
        let mut output = String::new();

        let fpr_registers = self
            .fpr
            .iter()
            .enumerate()
            .map(|(i, inst)| format!("fpr[{i}] = {inst}"))
            .collect::<Vec<String>>()
            .join("\n");
        output.push_str(&fpr_registers);

        output
    }
}

impl Index<&str> for FpRegisters {
    type Output = u64;

    // Convert string to the corresponding RegistersEnum value and use this to index.
    // If this is an invalid string, no enum will be returned, causing a panic as desired.
    fn index(&self, index: &str) -> &Self::Output {
        match FpRegisterType::from_str(index) {
            Ok(register) => &self[register],
            _ => panic!("{index} is not a valid register"),
        }
    }
}

impl IndexMut<&str> for FpRegisters {
    // Convert string to the corresponding RegistersEnum value and use this to index.
    // If this is an invalid string, no enum will be returned, causing a panic as desired.
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        match FpRegisterType::from_str(index) {
            Ok(register) => &mut self[register],
            _ => panic!("{index} is not a valid register"),
        }
    }
}

impl Index<FpRegisterType> for FpRegisters {
    type Output = u64;

    fn index(&self, index: FpRegisterType) -> &Self::Output {
        &self.fpr[index as usize]
    }
}

impl IndexMut<FpRegisterType> for FpRegisters {
    fn index_mut(&mut self, index: FpRegisterType) -> &mut Self::Output {
        &mut self.fpr[index as usize]
    }
}

/// Iterator that is used to view each register in the register file.
///
/// This contains a copy of all the registers and their values, and a [`FpRegisterTypeIter`],
/// as generated by [`strum::IntoEnumIterator`]. In other iterator implementations,
/// the internal state might be data like a [`FpRegisterType`]. However, since we can't
/// normally just "add 1" to get to the next register, we use an internal iterator
/// that can track the progression of one [`FpRegisterType`] to the next.
pub struct FpRegistersIter {
    registers: FpRegisters,
    register_iter: FpRegisterTypeIter,
}

/// This implementation of the [`Iterator`] trait essentially wraps the existing
/// [`FpRegisterTypeIter`] so that the register type can be paired with register data.
impl Iterator for FpRegistersIter {
    type Item = (FpRegisterType, u64);

    fn next(&mut self) -> Option<Self::Item> {
        match self.register_iter.next() {
            Some(register_type) => Some((register_type, self.registers[register_type])),
            None => None,
        }
    }
}

/// [`IntoIterator`] is a standard library trait that can convert any type into
/// an [`Iterator`]. In this case, this is an instance of [`FpRegistersIter`] with all the
/// data in the registers and a new [`FpRegisterTypeIter`].
impl IntoIterator for FpRegisters {
    type Item = (FpRegisterType, u64);
    type IntoIter = FpRegistersIter;

    /// Consumes the [`FpRegisters`] struct to create a new [`FpRegistersIter`] that can
    /// be iterated over.
    fn into_iter(self) -> Self::IntoIter {
        FpRegistersIter {
            registers: self,
            register_iter: FpRegisterType::iter(),
        }
    }
}
