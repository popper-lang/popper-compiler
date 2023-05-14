#include <iostream>
#include <Bytecode.h>
#include <Operand.h>


int main(int argc, char** argv)
{
    OperandInt op(3);

    int n = OperandInt::to_value(op.to_bytes());

    std::cout << n << std::endl;


    std::cout << std::endl;
    return 0;
}
