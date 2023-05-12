//
// Created by Antoine BarBier on 10/05/2023.
//

#ifndef POPPER_BACKEND_BYTECODE_H
#define POPPER_BACKEND_BYTECODE_H
#include <iostream>

class Bytecode {
public:
    enum OpCode {
        LOAD_CONST,
        ADD,
        SUB,
        MUL,
        DIV,
        IF,
        JUMP
    };





    OpCode opCode;

};





#endif //POPPER_BACKEND_BYTECODE_H
