window.SIDEBAR_ITEMS = {"fn":[["assign_instruction_numbers","This function assigns the instruction number to each instruction"],["build_instruction_list_from_lines","This function takes the vector of lines created by tokenize instructions and turns them into instructions assigning labels, operators, operands, and line numbers"],["confirm_operand_commas","This function goes through all but the last operands of each instruction checking that they end in a comma. If they do, the comma is removed. If they don’t a missing comma error is generated."],["create_label_map","Create_label_map builds a hashmap of addresses for labels in memory"],["expand_pseudo_instruction",""],["tokenize_instructions","This function takes the initial string of the program given by the editor and turns it into a vector of Line, a struct that holds tokens and the original line number"]]};