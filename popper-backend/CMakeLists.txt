cmake_minimum_required(VERSION 3.20)
project(popper_backend C)

set(CMAKE_C_STANDARD 11)

add_library(popper_backend src/library.c src/instruction.h src/main.c)
