
# Introduction

STEP.md is a file that will show and explain the steps of my language. To explain all this, there will be a general and simple example of my language:
```
import std.io;
fun sum(a: int, b: int): int {
  return a + b;
}
func main() {
    let c: int = sum(3, 4);
    print(c);
}
```

# STEP 1: Lexer and Parser

The Lexer and the Parser are part of the creation of a language. They transform your code into an AST (Abstract Syntax Tree).

Here, using the example given above, it would look like this:
(in pseudo structure)
```
  [
    Import(
        PathSegment("std", "io"),
        None
    ),
    Function(
      Args((a, INT), (b, INT)),
      INT,
      [
        Return(
          BinOp(
            Ident("a"),
            ADD,
            Ident("b"),
          )
        )
      ]
   ),
   Function(
        Args(),
        INT,
        [
          Let(
            "c",
            Call(
              Ident("sum"),
              [
                Int(3),
                Int(4)
              ]
            )
          ),
          Call(
            Ident("print"),
            [
              Ident("c")
            ]
          )
        ]
   )

```

all made by lalrpop

# STEP 2: Semantical Analizer
this program check type and variable ( if this variable exist or not)

# STEP 3: Compile to Popper MIR

this program will transform the AST into a kind of bytecode, called MIR (Mid-level Intermediate Representation)

```
module example {
    load_module "std/io.pop";
    func sum(a: @int, b: @int) @int {
        alloc __0, @int;
        add a, b, __0;
        ret __0;
    }

    func main() @int {
        alloc __1, @int;
        call sum, [@int 3, @int 4], __1;
        call print, [@int __1], null;
        ret 0;
    }
}
```

# STEP 4: LLVM Compiler

transforms this MIR  in LLVM IR

```


```
[
  Label("sum_func", [
      Mov(EAX, EDI),
      Mov(EBX, ESI),
      Add(EAX, EBX),
      Ret()
 ]),
 Label("main_main", [
      Mov(EDI, 3),
      Mov(ESI, 4),
      Call("sum_func"),
      Add(RBP, 2)
      Mov(Ind(RNP, 2), EAX),
      Mov(EDI, Ind(RNP, 2))
      Sub(RBP, 2)
      Call("_printf")
 ])
]
```

# STEP 5: Asm Arch Compiler

transforms this AST from ASM to ASM according to its arch , here, mine is x86, so it will transform it

```

sum_func:
  mov eax, edi
  mov ebx, esi
  add eax, ebx
  ret

main:
  mov edi, 3
  mov esi, 4
  call sum_func
  add rbp, 2
  mov [rnp + 2], eax
  mov edi, [rnp + 2]
  call _printf
  sub rbp, 2

```
