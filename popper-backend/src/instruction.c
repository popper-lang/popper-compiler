//
// Created by Antoine BarBier on 14/05/2023.
//

#include "instruction.h"

#include <stdlib.h>
#include <stdio.h>




Instruction *new_instruction(Opcode opcode, Constant arg) {
    Instruction *instruction = malloc(sizeof(Instruction));
    instruction->opcode = opcode;
    instruction->arg = arg;
    return instruction;
}

Instruction *get_instruction_from_bytes(char *bytes) {
    Opcode opcode = get_opcode_from_bytes(bytes);

    Constant arg = get_const_from_bytes(bytes);
    return new_instruction(opcode, arg);
}

char *instruction_to_bytes(Instruction *instruction) {
    char *bytes = malloc(sizeof(char) * 5);
    char *opcode_bytes = opcode_to_bytes(instruction->opcode);
    char *constant_bytes = constant_to_bytes(instruction->arg);
    bytes[0] = opcode_bytes[0];
    bytes[1] = constant_bytes[0];
    return bytes;
}

Opcode get_opcode_from_bytes(char *bytes) {
    return bytes[0];
}

Constant get_const_from_bytes(char *bytes) {
    Constant constant;
    constant.value = bytes[1];
    return constant;
}

char *opcode_to_bytes(Opcode opcode) {
    char *bytes = malloc(sizeof(char));
    bytes[0] = opcode;
    return bytes;
}

char *constant_to_bytes(Constant constant) {
    char *bytes = malloc(sizeof(char));
    bytes[0] = constant.value;
    return bytes;
}


char* read_line(FILE* file) {
    char* line = malloc(sizeof(char) * INSTRUCTION_MAX);
    char c;
    int i = 0;
    while ((c = fgetc(file)) != '\n') {
        line[i] = c;
        i++;
    }
    line[i] = '\0';
    return line;
}

Instruction *get_instruction_from_file(FILE *file) {
    char *bytes = malloc(sizeof(char) * 5);
    char *line = malloc(sizeof(char) * 500);
    Instruction *instruction = malloc(sizeof(Instruction) * INSTRUCTION_MAX);
    int index = 0;

     do {
         line = read_line(file);
         instruction[index] = *get_instruction_from_bytes(line);
     } while (feof(file) == 0);

     return instruction;
}
