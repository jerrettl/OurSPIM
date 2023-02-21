//! Module for mapping lines in the visual datapath to information
//! and variables in the coded datapath.

use super::super::datapath::VisualDatapath;
use super::datapath::MipsDatapath;

/// A collection of data surrounding a line in the visual datapath.
pub struct LineInformation {
    pub title: String,
    pub description: String,

    /// The value stored in a line. This may not be a 64-bit value, but should
    /// refer to the `bits` field to determine how many bits on the line are
    /// relevant to be displayed.
    pub value: u64,

    /// The number of bits on a given line.
    pub bits: u64,
}

impl VisualDatapath for MipsDatapath {
    type LineInformation = LineInformation;

    fn visual_line_to_data(&self, variable: &str) -> LineInformation {
        match variable {
            "instruction" => LineInformation {
                title: String::from("The Instruction"),
                description: String::from("This block holds the bits of the currently executing instruction"),
                value: self.state.instruction as u64,
                bits: 32,
            },

            "pc" => LineInformation {
                title: String::from("Program Counter"),
                description: String::from("The address of the instruction to be executed next."),
                value: self.registers.pc,
                bits: 64,
            },

            "register_write_data" => LineInformation {
                title: String::from("Register Write Data"),
                description: String::from("Data that will be written to a general-purpose register."),
                value: self.state.register_write_data,
                bits: 64,
            },

            "rs" => LineInformation {
                title: String::from("Instruction [20-16] (rs)"),
                description: String::from("The rs field"),
                value: self.state.rs as u64,
                bits: 5,
            },

            "rt" => LineInformation {
                title: String::from("Instruction [20-16] (rt)"),
                description: String::from("The rt field. Contains the second register to be read for an R-type instruction."),
                value: self.state.rt as u64,
                bits: 5,
            },

            "rd" => LineInformation {
                title: String::from("Instruction [20-16] (rd)"),
                description: String::from("The rd field"),
                value: self.state.rd as u64,
                bits: 5,
            },
            
            "imm" => LineInformation {
                title: String::from("Instruction [15-0] (imm)"),
                description: String::from("The imm field"),
                value: self.state.imm as u64,
                bits: 16,
            },

            "coprocessor.fs" => LineInformation {
                title: String::from("Instruction [15-11]"),
                description: String::from("The fs field. Contains the first register to be read in a floating-point instruction."),
                value: self.coprocessor.state.fs as u64,
                bits: 5,
            },
            
            "read_data_1" => LineInformation {
                title: String::from("Read Data 1"),
                description: String::from("The rs register data"),
                value: self.state.read_data_1,
                bits: 5,
            },
            
            "read_data_2" => LineInformation {
                title: String::from("Read Data 2"),
                description: String::from("The rt register data"),
                value: self.state.read_data_2,
                bits: 5,
            },
            
            "sign_extend" => LineInformation {
                title: String::from("Sign extended imm"),
                description: String::from("The immediate low 16 bits sign extended to a 32/64bit value"),
                value: self.state.sign_extend,
                bits: 64,
            },
            
            "sign_extend_shift_left_by_2" => LineInformation {
                title: String::from("Sign extended imm << 2"),
                description: String::from("The sign extended immediate low 16 bits shifted left by 2"),
                value: self.state.sign_extend_shift_left_by_2,
                bits: 64,
            },
            
            "alu_result" => LineInformation {
                title: String::from("ALU Result"),
                description: String::from("The main output line of the ALU"),
                value: self.state.alu_result,
                bits: 64,
            },
            
            "memory_data" => LineInformation {
                title: String::from("Memory Data"),
                description: String::from("The data retrieved from memory"),
                value: self.state.memory_data,
                bits: 64,
            },
            
            "data_result" => LineInformation {
                title: String::from("Data Result"),
                description: String::from("This data maybe possible get wirtten to a register"),
                value: self.state.data_result,
                bits: 64,
            },
            
            "lower_26" => LineInformation {
                title: String::from("Lower 26"),
                description: String::from("The lower 26 bits of instruction"),
                value: self.state.lower_26 as u64,
                bits: 26,
            },
            
            "lower_26_shifted_left_by_2" => LineInformation {
                title: String::from("Lower 26 Shifted Left By 2"),
                description: String::from("The lower 26 bits of instruction shifted left by 2"),
                value: self.state.lower_26_shifted_left_by_2 as u64,
                bits: 28,
            },
            
            "jump_address" => LineInformation {
                title: String::from("Jump Address"),
                description: String::from("The combination the high (32 + 4) bits of PC and the low 26 instruction bits shifted left by 2"),
                value: self.state.jump_address,
                bits: 64,
            },
            
            "pc_plus_4" => LineInformation {
                title: String::from("PC + 4"),
                description: String::from("The address of the currectly executing instruction + 4"),
                value: self.state.pc_plus_4,
                bits: 64,
            },
            
            "new_pc" => LineInformation {
                title: String::from("New PC"),
                description: String::from("The address of the next instruction to execute"),
                value: self.state.new_pc,
                bits: 64,
            },
            
            "relative_pc_branch" => LineInformation {
                title: String::from("Relative PC Branch"),
                description: String::from("The PC relative branch address"),
                value: self.state.relative_pc_branch,
                bits: 64,
            },
            
            "mem_mux1_to_mem_mux2" => LineInformation {
                title: String::from("Mux to Mux"),
                description: String::from("This line holder either PC+4, or the relative branch address"),
                value: self.state.mem_mux1_to_mem_mux2,
                bits: 64,
            },
            
            "write_data" => LineInformation {
                title: String::from("Write Data"),
                description: String::from("This data will be written to memory if the memWrite flag is set"),
                value: self.state.new_pc,
                bits: 64,
            },


            _ => LineInformation {
                title: String::from("[Title]"),
                description: String::from("[Description]"),
                value: 0,
                bits: 0,
            },
        }
    }
}
