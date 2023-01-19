use crate::parser::parser_instruction_tokenization::instruction_tokenization::ErrorType::{
    LabelAssignmentError, MissingComma,
};
use crate::parser::parser_instruction_tokenization::instruction_tokenization::TokenType::Unknown;
use crate::parser::parser_instruction_tokenization::instruction_tokenization::{
    Error, Instruction, Line, Token,
};

///This function takes the vector of lines created by tokenize instructions and turns them into instructions
///assigning labels, operators, operands, and line numbers
pub fn build_instruction_list_from_lines(mut lines: Vec<Line>) -> Vec<Instruction> {
    let mut instruction_list: Vec<Instruction> = Vec::new();
    let mut instruction = Instruction::default();

    let mut i = 0;
    //goes through each line of the line vector and builds instructions as it goes
    while i < lines.len() {
        let mut operand_iterator = 1;

        if lines[i].tokens[0].token_name.ends_with(':') {
            //if the instruction already has a label at this point, that means that the user wrote a label on a line on its
            //own and then wrote another label on the next line without ever finishing the first
            if instruction.label.is_some() {
                instruction.errors.push(Error {
                    error_name: LabelAssignmentError,
                    token_number_giving_error: 0,
                })
                //if the above error doesn't occur, we can push the label to the instruction struct.
            } else {
                lines[i].tokens[0].token_name.pop();
                instruction.label = Some((lines[i].tokens[0].clone(), lines[i].line_number));
            }

            if lines[i].tokens.len() == 1 {
                //if the only token on the last line of the program is a label, the user never finished assigning a value to the label
                if i == (lines.len() - 1) {
                    instruction.errors.push(Error {
                        error_name: LabelAssignmentError,
                        token_number_giving_error: 0,
                    });
                    instruction_list.push(instruction.clone());
                }

                i = i + 1;
                continue;
            }
            //since token[0] was a label, the operator will be token[1] and operands start at token[2]
            instruction.operator = lines[i].tokens[1].clone();
            operand_iterator = 2;
        } else {
            instruction.operator = lines[i].tokens[0].clone();
        }
        //push all operands to the instruction operand vec
        while operand_iterator < lines[i].tokens.len() {
            instruction
                .operands
                .push(lines[i].tokens[operand_iterator].clone());
            operand_iterator += 1;
        }
        instruction.line_number = lines[i].line_number as u32;

        //push completed instruction to the instruction vec
        instruction_list.push(instruction.clone());
        instruction = Instruction::default();

        i = i + 1;
    }

    return instruction_list;
}

///This function goes through all but the last operands of each instruction checking that they end in a comma.
/// If they do, the comma is removed. If they don't a missing comma error is generated.
pub fn confirm_operand_commas(instructions: &mut Vec<Instruction>) {
    for instruction in instructions {
        for i in 0..(instruction.operands.len() - 1) {
            if instruction.operands[i].token_name.ends_with(',') {
                instruction.operands[i].token_name.pop();
            } else {
                instruction.errors.push(Error {
                    error_name: MissingComma,
                    token_number_giving_error: i as u8,
                })
            }
        }
    }
}

//TODO Add more pseudo instructions. Especially ones that are converted into more than a single instruction to make sure this method works
pub fn convert_pseudo_instruction_into_real_instruction(instruction_list: &mut Vec<Instruction>) {
    for (_i, mut instruction) in instruction_list.clone().into_iter().enumerate(){
        match &*instruction.operator.token_name {
            "li" => {
                instruction.operator.token_name = "ori".to_string();

                instruction.operands.push(Token {
                    token_name: "$zero".to_string(),
                    starting_column: 0,
                    token_type: Default::default(),
                });
            }

            _ => {}
        }
    }
}

///This function takes the initial string of the program given by the editor and turns it into a vector of Line,
/// a struct that holds tokens and the original line number
pub fn tokenize_instructions(program: String) -> Vec<Line> {
    let mut line_vec: Vec<Line> = Vec::new();
    let mut token: Token = Token {
        token_name: "".to_string(),
        starting_column: 0,
        token_type: Unknown,
    };

    for (i, line_of_program) in program.lines().enumerate() {
        let mut line_of_tokens = Line {
            line_number: i as i32,

            tokens: vec![],
        };

        for (j, char) in line_of_program.chars().enumerate() {
            if char == '#' {
                break;
            };
            if char != ' ' {
                if token.token_name.is_empty() {
                    token.starting_column = j as i32;
                }
                token.token_name.push(char);
            } else if !token.token_name.is_empty() {
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

    line_vec
}
