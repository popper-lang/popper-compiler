//
// Created by Antoine BarBier on 14/05/2023.
//

#ifndef POPPER_BACKEND_INSTRUCTION_H
#define POPPER_BACKEND_INSTRUCTION_H
#define DBG(x) printf("(%s, line %d) %x\n", __FILE__, __LINE__, x)
#define INSTRUCTION_MAX 100
#include <stdio.h>

typedef enum {
    OP_LOAD_CONST,
    OP_ADD,
    OP_SUB,
    OP_MUL,
    OP_DIV,
    OP_IF,
    OP_JUMP
} Opcode;

typedef struct {
    int value;
} Constant;

typedef struct {
    Opcode opcode;
    Constant arg;
} Instruction;

Instruction *new_instruction(Opcode opcode, Constant arg);
Instruction *get_instruction_from_bytes(char *bytes);
char *instruction_to_bytes(Instruction *instruction);

Opcode get_opcode_from_bytes(char *bytes);
Constant get_const_from_bytes(char *bytes);

char* opcode_to_bytes(Opcode opcode);
char* constant_to_bytes(Constant constant);

Instruction *get_instruction_from_file(FILE *file);

char* read_line(FILE* file);

#endif //POPPER_BACKEND_INSTRUCTION_H
