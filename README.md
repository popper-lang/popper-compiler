# Popper
Welcome to the world of Popper!

Popper is an functional programming language designed to simplify the development process by providing a clear and concise syntax written in Rust

# Installation
To use Popper, you must first install the Popper compiler. You can do this by cloning the official Github repository and installing from the sources.

```bash
git clone https://github.com/poplang/popper-lang.git
cd popper-lang
cargo build
```
# Syntax
Here is an example Popper program that calculates the Fibonacci sequence:

```
use "io.pop" as io
use "itertool.pop" as itertool

fun fib(n) {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

io::println(fib(10))
io::println(itertool::map(fib, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]))
```
This program defines a function called fib that takes a single argument n. The function uses recursion to calculate the nth Fibonacci number. It then prints out the 10th Fibonacci number and a list of the first 10 Fibonacci numbers using the itertool::map function.

# Key Features
my langage is **SFBCF**: 
* **S**imple and clear syntax
* **F**unctional programming paradigm
* **B**uilt-in support for modules and packages
* **C**oncurrency support with lightweight threads (also known as "green threads")
* **F**ast and efficient interpreteur

# Todo
 * ✅ Lexer & Parser (with help of the library [lalrpop](https://github.com/lalrpop/lalrpop) )
 * ⬛️ Semantical Analizer (Work-in-progress)
 * ❌ SBC ( Simple Bytecode Compiler )
 * ❌ MBC ( Memory Bytecode Compiler )
 * ❌ SAC ( Simple Asm Compiler )
 * ❌ AAC ( Asm Arch Compiler ) 



