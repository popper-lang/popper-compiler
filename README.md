# Popper
Welcome to the world of Popper!

Popper is an functional programming language designed to simplify the development process by providing a clear and concise syntax written in Rust

# Installation
To use Popper, you must first install the Popper compiler. You can do this by cloning the official Github repository and installing from the sources.

```bash
git clone https://github.com/popper-lang/popper-lang.git
cd popper-lang
cargo build --release
sh INSTALL.sh
```
# Syntax
Here is an example Popper program that calculates the Fibonacci sequence:

```
use "itertool.pop" as itertool

fun fib(n: int) : int {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

print(fib(10))
println(itertool::map(fib, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]))
```
This program defines a function called fib that takes a single argument n. The function uses recursion to calculate the nth Fibonacci number. It then prints out the 10th Fibonacci number and a list of the first 10 Fibonacci numbers using the itertool::map function.

# Key Features
my langage is **SFBF**: 
* **S**imple and clear syntax
* **F**unctional programming paradigm
* **B**uilt-in support for modules and packages
* **F**ast and efficient compiler

# Todo
 * ✅ Lexer & Parser (with help of the library [lalrpop](https://github.com/lalrpop/lalrpop) )
 * ✅ Semantical Analizer (Work-in-progress)
 * ✅ SBC ( Simple Bytecode Compiler )
 * ✅ SAC ( Simple Asm Compiler )
 * ✅ AAC ( Asm Arch Compiler ) 
Actually all step are not really complet

# Info

if you want information how this langage work you can read [STEP.md](STEP.md)

