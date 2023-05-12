//
// Created by Antoine BarBier on 11/05/2023.
//

#ifndef POPPER_BACKEND_OPERAND_H
#define POPPER_BACKEND_OPERAND_H
#include <vector>


template<class T>
class Operand {
public:
    enum OperandType {
        Int,
        Float,
        Boolean
    };

    Operand(OperandType op, T value )
    : m_operand_type(op), m_value(value) {}

    virtual char* to_bytes();
    static  T to_value(char bytes[]);
    OperandType get_type() { return m_operand_type; }
    T get_value() { return m_value; }
    virtual void debug();

protected:
    OperandType m_operand_type;
    T m_value;

};


class OperandInt : public Operand<int> {
public:
    OperandInt(int value)
    : Operand(OperandType::Int, value) {}

    std::vector<char*>() to_bytes();
    static int to_value(std::vector<char> bytes);
    void debug() override;
};







#endif //POPPER_BACKEND_OPERAND_H
