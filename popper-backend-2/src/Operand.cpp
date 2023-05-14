//
// Created by Antoine BarBier on 11/05/2023.
//

#include <Operand.h>
#include <iostream>
#include <cstring>

char* OperandInt::to_bytes() {
    char bytes[sizeof(int)];

    std::memcpy(bytes, &m_value, sizeof(int));

    return bytes;
}

int OperandInt::to_value(std::vector<char>) {
    int result = 0;
    for (size_t i = 0; i < sizeof(int); ++i) {
        result |= static_cast<int>(bytes[i]) << (8 * i);
    }
    return result;
}

void OperandInt::debug() {
    char* bytes = to_bytes();
    for (int i = sizeof(int) - 1; i >= 0; --i) {
        std::cout << std::hex << static_cast<int>(bytes[i]) << " ";
    }
}
