//
// Created by Antoine BarBier on 14/05/2023.
//


#include "instruction.h"
#include <stdio.h>

int main() {
    Instruction *instruction = new_instruction(OP_LOAD_CONST, (Constant) { .value = 42 });
    Instruction* i = get_instruction_from_bytes(instruction_to_bytes(instruction));
    printf("%d\n", i->arg.value);
    return 0;
}