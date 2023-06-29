
# Introduction

STEP.md is a file that will show and explain the steps of my language. To explain all this, there will be a general and simple example of my language:
```
fun sum(a: int, b: int) -> int {
  return a + b;
}

let c = sum(3, 4);
print(c);
``` 

# STEP 1: Lexer and Parser

The Lexer and the Parser are part of the creation of a language. They transform your code into an AST (Abstract Syntax Tree).

Here, using the example given above, it would look like this:
(in pseudo structure)
```
  [
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
```

all made by lalrpop

# STEP 2: Semantical Analizer
this program check type and variable ( if this variable exist or not)

# STEP 3: Simple Bytecode Compiler  

this program will transform the AST into "simple" bytecode:
```
0  ARG ("a", INT)
1  ARG ("b", INT)
2  LOAD_STR "sum"
3  DEF_FUNC 5
4  JMP 9
5  LOAD_IDENT "a"
6  LOAD_IDENT "b"
7  ADD 
8  RETURN
9  LOAD_INT 3
10 LOAD_INT 4
11 CALL "sum"
12 LET "c"
13 LOAD_IDENT "c"
14 CALL "print"
```

# STEP 4: Simple Asm Compiler

it will take the bytecode and transform it into a kind of ASM AST:

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

