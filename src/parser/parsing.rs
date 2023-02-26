use crate::parser::parser_structs_and_enums::instruction_tokenization::ErrorType::*;
use crate::parser::parser_structs_and_enums::instruction_tokenization::TokenType::{
    Label, Operator, Unknown,
};
use crate::parser::parser_structs_and_enums::instruction_tokenization::{
    Data, Error, Instruction, Line, MonacoLineInfo, Token,
};
use levenshtein::levenshtein;
use std::collections::HashMap;

///Takes the initial string of the program given by the editor and turns it into a vector of Line,
/// a struct that holds tokens and the original line number.
pub fn tokenize_program(program: String) -> (Vec<Line>, Vec<String>, Vec<MonacoLineInfo>) {
    let mut monaco_line_info_vec: Vec<MonacoLineInfo> = Vec::new();

    let mut line_vec: Vec<Line> = Vec::new();
    let mut token: Token = Token {
        token_name: "".to_string(),
        start_end_columns: (0, 0),
        token_type: Unknown,
    };
    let mut lines_in_monaco: Vec<String> = Vec::new();

    for (i, line_of_program) in program.lines().enumerate() {
        monaco_line_info_vec.push(MonacoLineInfo {
            mouse_hover_string: "".to_string(),
            error_start_end_columns: vec![],
        });

        lines_in_monaco.push(line_of_program.parse().unwrap());

        let mut line_of_tokens = Line {
            line_number: i as u32,

            tokens: vec![],
        };
        let mut is_string = false;
        let mut check_escape = false;
        //iterates through every character on each line of the program
        for (j, char) in line_of_program.chars().enumerate() {
            token.start_end_columns.1 = j as u32;
            if char == '#' {
                if j > 0 {
                    token.start_end_columns.1 -= 1;
                }
                break;
            };
            //is string is a flag to handle strings and read them in as a single token
            if is_string {
                if char == '\\' {
                    check_escape = true;
                    continue;
                }
                if check_escape {
                    match char {
                        'n' => {
                            token.token_name.push('\n');
                        }
                        't' => {
                            token.token_name.push('\t');
                        }
                        '\\' => {
                            token.token_name.push('\\');
                        }
                        '\"' => {
                            token.token_name.push('\"');
                        }
                        '\'' => {
                            token.token_name.push('\'');
                        }
                        _ => {
                            token.token_name.push('\\');
                            token.token_name.push(char);
                        }
                    }
                    check_escape = false;
                } else if char == '\"' {
                    token.token_name.push('\"');
                    is_string = false;
                } else {
                    token.token_name.push(char);
                }
            } else if char == '\"' {
                if !token.token_name.is_empty() {
                    line_of_tokens.tokens.push(token.clone());
                }
                token.token_name = '\"'.to_string();
                token.start_end_columns.0 = j as u32;
                is_string = true;
            } else if char != ' ' {
                if token.token_name.is_empty() {
                    token.start_end_columns.0 = j as u32;
                }
                token.token_name.push(char);
                if char == ',' {
                    if token.token_name.len() == 1 {
                        let length = line_of_tokens.tokens.len();
                        line_of_tokens.tokens[length - 1].token_name.push(char);
                    } else {
                        line_of_tokens.tokens.push(token.clone());
                    }
                    token.token_name = "".to_string();
                }
            } else if !token.token_name.is_empty() {
                token.start_end_columns.1 -= 1;
                line_of_tokens.tokens.push(token.clone());
                token.token_name = "".to_string();
            }
        }
        if !token.token_name.is_empty() {
            line_of_tokens.tokens.push(token.clone());
            token.token_name = "".to_string();
        }
        if !line_of_tokens.tokens.is_empty() {
            line_vec.push(line_of_tokens.clone());
        }
    }

    (line_vec, lines_in_monaco, monaco_line_info_vec)
}

///This function takes the vector of lines created by tokenize program and turns them into instructions
///assigning labels, operators, operands, and line numbers and data assigning labels, data types, and values
pub fn separate_data_and_text(mut lines: Vec<Line>) -> (Vec<Instruction>, Vec<Data>) {
    let mut instruction_list: Vec<Instruction> = Vec::new();
    let mut instruction = Instruction::default();
    let mut data_list: Vec<Data> = Vec::new();
    let mut data = Data::default();
    let mut is_text = true;

    let mut i = 0;
    //goes through each line of the line vector and builds instructions as it goes
    while i < lines.len() {
        if lines[i].tokens[0].token_name == ".text" {
            is_text = true;
            i += 1;
            continue;
        } else if lines[i].tokens[0].token_name == ".data" {
            is_text = false;
            i += 1;
            continue;
        }

        if is_text {
            let mut operand_iterator = 1;

            if lines[i].tokens[0].token_name.ends_with(':') {
                //if the instruction already has a label at this point, that means that the user wrote a label on a line on its
                //own and then wrote another label on the next line without ever finishing the first
                if instruction.label.is_some() {
                    instruction.errors.push(Error {
                        error_name: LabelAssignmentError,
                        operand_number: None,
                        message: "".to_string(),
                    })
                    //if the above error doesn't occur, we can push the label to the instruction struct.
                } else {
                    lines[i].tokens[0].token_name.pop();
                    lines[i].tokens[0].token_type = Label;
                    instruction.label = Some((lines[i].tokens[0].clone(), lines[i].line_number));
                }

                if lines[i].tokens.len() == 1 {
                    //if the only token on the last line of the program is a label, the user never finished assigning a value to the label
                    if i == (lines.len() - 1) {
                        instruction.errors.push(Error {
                            error_name: LabelAssignmentError,
                            operand_number: None,
                            message: "".to_string(),
                        });
                        instruction_list.push(instruction.clone());
                    }

                    i += 1;
                    continue;
                }
                //since token[0] was a label, the operator will be token[1] and operands start at token[2]
                lines[i].tokens[1].token_type = Operator;
                instruction.operator = lines[i].tokens[1].clone();
                operand_iterator = 2;
            } else {
                lines[i].tokens[0].token_type = Operator;
                instruction.operator = lines[i].tokens[0].clone();
            }

            let first_operand_index = operand_iterator;

            //push all operands to the instruction operand vec that will have commas
            while operand_iterator < (lines[i].tokens.len() - 1) {
                if lines[i].tokens[operand_iterator].token_name.ends_with(',') {
                    lines[i].tokens[operand_iterator].token_name.pop();
                } else {
                    instruction.errors.push(Error {
                        error_name: MissingComma,
                        operand_number: Some((operand_iterator - first_operand_index) as u8),
                        message: "".to_string(),
                    })
                }
                instruction
                    .operands
                    .push(lines[i].tokens[operand_iterator].clone());
                operand_iterator += 1;
            }

            //simple statement to handle cases where the user doesn't finish instructions
            if operand_iterator >= lines[i].tokens.len() {
                instruction_list.push(instruction.clone());
                i += 1;
                continue;
            }

            //push last operand that will not have a comma
            instruction
                .operands
                .push(lines[i].tokens[operand_iterator].clone());

            instruction.line_number = lines[i].line_number;

            //push completed instruction to the instruction vec
            instruction_list.push(instruction.clone());
            instruction = Instruction::default();
        }
        //if not text, it must be data
        else {
            data.line_number = lines[i].line_number;

            //the first token should be the label name
            if lines[i].tokens[0].token_name.ends_with(':') {
                lines[i].tokens[0].token_name.pop();
                lines[i].tokens[0].token_type = Label;
                data.label = lines[i].tokens[0].clone();
            } else {
                data.errors.push(Error {
                    error_name: ImproperlyFormattedLabel,
                    operand_number: Some(0),
                    message: "".to_string(),
                });
                lines[i].tokens[0].token_type = Label;
                data.label = lines[i].tokens[0].clone();
            }

            //just a simple check in case the user didn't complete a line
            if lines[i].tokens.len() < 2 {
                data.errors.push(Error {
                    error_name: ImproperlyFormattedData,
                    operand_number: None,
                    message: "".to_string(),
                });
                i += 1;
                continue;
            }

            //the second token on the line is the data type
            data.data_type = lines[i].tokens[1].clone();

            let mut value_iterator = 2;
            let first_value_index = value_iterator;

            //push all values to the data vec that will have commas
            while value_iterator < (lines[i].tokens.len() - 1) {
                if lines[i].tokens[value_iterator].token_name.ends_with(',') {
                    lines[i].tokens[value_iterator].token_name.pop();
                } else {
                    instruction.errors.push(Error {
                        error_name: MissingComma,
                        operand_number: Some((value_iterator - first_value_index) as u8),
                        message: "".to_string(),
                    })
                }
                data.data_entries_and_values
                    .push((lines[i].tokens[value_iterator].clone(), 0));
                value_iterator += 1;
            }

            //push last operand that will not have a comma
            data.data_entries_and_values
                .push((lines[i].tokens[value_iterator].clone(), 0));

            data_list.push(data.clone());
            data = Data::default();
        }
        i += 1;
    }

    (instruction_list, data_list)
}

///Create_label_map builds a hashmap of addresses for labels in memory
pub fn create_label_map(
    instruction_list: &mut Vec<Instruction>,
    data_list: &mut [Data],
) -> HashMap<String, u32> {
    let mut labels: HashMap<String, u32> = HashMap::new();
    for instruction in &mut *instruction_list {
        if instruction.label.is_some() {
            //if the given label name is already used, an error is generated
            if labels.contains_key(&*instruction.label.clone().unwrap().0.token_name) {
                instruction.errors.push(Error {
                    error_name: LabelMultipleDefinition,
                    operand_number: None,
                    message: "".to_string(),
                });
                //otherwise, it is inserted
            } else {
                labels.insert(
                    instruction.clone().label.unwrap().0.token_name,
                    instruction.clone().instruction_number << 2,
                );
            }
        }
    }

    let last_instruction = instruction_list.last();

    let offset_for_instructions: u32 = if let Some(..) = last_instruction {
        (last_instruction.unwrap().instruction_number + 1) << 2
    } else {
        0
    };

    for (i, data) in data_list.iter_mut().enumerate() {
        //if the given label name is already used, an error is generated
        if labels.contains_key(&*data.label.clone().token_name) {
            data.errors.push(Error {
                error_name: LabelMultipleDefinition,
                operand_number: Some(i as u8),
                message: "".to_string(),
            });
            //otherwise, it is inserted
        } else {
            labels.insert(
                data.label.token_name.clone(),
                data.data_number + offset_for_instructions,
            );
        }
    }

    labels
}

///Goes through each error found in the parsing & assembling process and suggests to the user a way of
/// correcting the error.
pub fn suggest_error_corrections(
    instructions: &mut [Instruction],
    data: &mut [Data],
    labels: &HashMap<String, u32>,
    monaco_line_info: &mut [MonacoLineInfo],
) {
    //go through each error in the instructions and suggest a correction
    for instruction in instructions {
        if instruction.errors.is_empty() {
            monaco_line_info[instruction.line_number as usize]
                .mouse_hover_string
                .push_str("Binary: ");
            monaco_line_info[instruction.line_number as usize]
                .mouse_hover_string
                .push_str(&format!("{:032b}", instruction.binary));
            monaco_line_info[instruction.line_number as usize]
                .mouse_hover_string
                .push('\n');
        } else {
            for error in &mut instruction.errors {
                match error.error_name {
                    UnsupportedInstruction => {
                        error.message =
                             "While this is a valid instruction, it is not currently supported by SWIM\n"
                                 .to_string();
                    }
                    UnrecognizedGPRegister => {
                        let gp_registers = [
                            "$zero", "$at", "$v0", "$v1", "$a0", "$a1", "$a2", "$a3", "$t0", "$t1",
                            "$t2", "$t3", "$t4", "$t5", "$t6", "$t7", "$s0", "$s1", "$s2", "$s3",
                            "$s4", "$s5", "$s6", "$s7", "$t8", "$t9", "$k0", "$k1", "$gp", "$sp",
                            "$fp", "$ra", "r0", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8",
                            "r9", "r10", "r11", "r12", "r13", "r14", "r15", "r16", "r17", "r18",
                            "r19", "r20", "r21", "r22", "r23", "r24", "r25", "r26", "r27", "r28",
                            "r29", "r30", "r31",
                        ];

                        let given_string = &instruction.operands
                            [error.operand_number.unwrap() as usize]
                            .token_name;
                        let mut closest: (usize, String) = (usize::MAX, "".to_string());

                        for register in gp_registers {
                            if levenshtein(given_string, register) < closest.0 {
                                closest.0 = levenshtein(given_string, register);
                                closest.1 = register.to_string();
                            }
                        }

                        let mut suggestion = "A valid, similar register is: ".to_string();
                        suggestion.push_str(&closest.1);
                        suggestion.push_str(".\n");
                        error.message = suggestion;
                    }
                    UnrecognizedFPRegister => {
                        let fp_registers = [
                            "$f0", "$f1", "$f2", "$f3", "$f4", "$f5", "$f6", "$f7", "$f8", "$f9",
                            "$f10", "$f11", "$f12", "$f13", "$f14", "$f15", "$f16", "$f17", "$f18",
                            "$f19", "$f20", "$f21", "$f22", "$f23", "$f24", "$f25", "$f26", "$f27",
                            "$f28", "$f29", "$f30", "$f31",
                        ];

                        let given_string = &instruction.operands
                            [error.operand_number.unwrap() as usize]
                            .token_name;
                        let mut closest: (usize, String) = (usize::MAX, "".to_string());

                        for register in fp_registers {
                            if levenshtein(given_string, register) < closest.0 {
                                closest.0 = levenshtein(given_string, register);
                                closest.1 = register.to_string();
                            }
                        }

                        let mut suggestion = "A valid, similar register is: ".to_string();
                        suggestion.push_str(&closest.1);
                        suggestion.push_str(".\n");
                        error.message = suggestion;
                    }
                    UnrecognizedInstruction => {
                        let recognized_instructions = [
                            "add", "sub", "mul", "div", "lw", "sw", "lui", "aui", "andi", "ori",
                            "addi", "dadd", "dsub", "dmul", "ddiv", "or", "and", "add.s", "add.d",
                            "sub.s", "sub.d", "mul.s", "mul.d", "div.s", "div.d", "dahi", "dati",
                            "daddiu", "slt", "sltu", "swc1", "lwc1", "mtc1", "dmtc1", "mfc1",
                            "dmfc1", "j", "beq", "bne", "c.eq.s", "c.eq.d", "c.lt.s", "c.le.s",
                            "c.le.d", "c.ngt.s", "c.ngt.d", "c.nge.s", "c.nge.d", "bc1t", "bc1f",
                        ];

                        let given_string = &instruction.operator.token_name;
                        let mut closest: (usize, String) = (usize::MAX, "".to_string());

                        for instruction in recognized_instructions {
                            if levenshtein(given_string, instruction) < closest.0 {
                                closest.0 = levenshtein(given_string, instruction);
                                closest.1 = instruction.to_string();
                            }
                        }

                        let mut suggestion = "A valid, similar instruction is: ".to_string();
                        suggestion.push_str(&closest.1);
                        suggestion.push_str(".\n");
                        error.message = suggestion;
                    }
                    IncorrectRegisterTypeGP => {
                        error.message =
                            "Expected FP register but received GP register.\n".to_string();
                    }
                    IncorrectRegisterTypeFP => {
                        error.message =
                            "Expected GP register but received FP register.\n".to_string();
                    }
                    MissingComma => {
                        error.message =
                            "Operand expected to end with a comma but it does not.\n".to_string()
                    }
                    ImmediateOutOfBounds => {
                        error.message = "Immediate value given cannot be expressed in the available number of bits.\n".to_string();
                    }
                    NonIntImmediate => {
                        error.message =
                            "The given string cannot be recognized as an integer.\n".to_string();
                    }
                    NonFloatImmediate => {
                        error.message =
                            "The given string cannot be recognized as a float.\n".to_string();
                    }
                    InvalidMemorySyntax => {
                        error.message = "The given string for memory does not match syntax of \"offset(base)\" or \"label\".\n".to_string();
                    }
                    IncorrectNumberOfOperands => {
                        error.message = "The given number of operands does not match the number expected for the given instruction.\n".to_string();
                    }
                    LabelMultipleDefinition => {
                        error.message =
                            "The given label name is already used elsewhere in the project.\n"
                                .to_string();
                    }
                    LabelNotFound => {
                        if labels.is_empty() {
                            error.message = "There is no recognized labelled memory.\n".to_string();
                            continue;
                        }

                        let given_string = &instruction.operands
                            [error.operand_number.unwrap() as usize]
                            .token_name;
                        let mut closest: (usize, String) = (usize::MAX, "".to_string());

                        for label in labels {
                            if levenshtein(given_string, label.0) < closest.0 {
                                closest.0 = levenshtein(given_string, label.0);
                                closest.1 = label.0.to_string();
                            }
                        }

                        let mut suggestion = "A valid, similar label is: ".to_string();
                        suggestion.push_str(&closest.1);
                        suggestion.push_str(".\n");
                        error.message = suggestion;
                    }
                    ImproperlyFormattedASCII => {
                        error.message =
                            "Token recognized as ASCII does not start and or end with \".\n"
                                .to_string();
                    }
                    ImproperlyFormattedChar => {
                        error.message = "Token recognized as a char does not end with ' or is larger than a single char.\n".to_string();
                    }
                    _ => {
                        error.message = "PARSER/ASSEMBLER ERROR. THIS ERROR TYPE SHOULD NOT BE ABLE TO BE ASSOCIATED WITH AN INSTRUCTION.\n".to_string();
                    }
                }
                monaco_line_info[instruction.line_number as usize]
                    .mouse_hover_string
                    .push_str(&error.message.clone());
            }
        }
    }

    //go through each error in the data and suggest a correction
    for datum in data {
        for error in &mut datum.errors {
            match &error.error_name {
                UnrecognizedDataType => {
                    let recognized_data_types = [
                        ".ascii", ".asciiz", ".byte", ".double", ".float", ".half", ".space",
                        ".word",
                    ];

                    let given_string = &datum.data_type.token_name.to_string();
                    let mut closest: (usize, String) = (usize::MAX, "".to_string());

                    for data_type in recognized_data_types {
                        if levenshtein(given_string, data_type) < closest.0 {
                            closest.0 = levenshtein(given_string, data_type);
                            closest.1 = data_type.to_string();
                        }
                    }

                    let mut suggestion = "A valid, similar data type is: ".to_string();
                    suggestion.push_str(&closest.1);
                    suggestion.push_str(".\n");
                    error.message = suggestion;
                }
                LabelAssignmentError => {
                    error.message = "A label is specified but it is not followed by data or an instruction committed to memory.\n".to_string();
                }
                LabelMultipleDefinition => {
                    error.message =
                        "The given label name is already used elsewhere in the project.\n"
                            .to_string();
                }
                _ => {
                    error.message = "PARSER/ASSEMBLER ERROR. THIS ERROR TYPE SHOULD NOT BE ABLE TO BE ASSOCIATED WITH DATA.\n".to_string();
                }
            }
            monaco_line_info[datum.line_number as usize]
                .mouse_hover_string
                .push_str(&error.message.clone());
        }
    }
}