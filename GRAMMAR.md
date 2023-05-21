# Grammar of the language Popper

## Introduction

This document describes the grammar of the language Popper. The grammar is
written in [EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form)
notation. 

## Grammar

```
program = { statement } ;
statement = ( if | while | for | assignment | function | return | expr ) ";" ;
if = "if" expr "{" { statement } "}" [ "else" "{" { statement } "}" ] ;
while = "while" expr  "{" { statement } "}" ;

for = "for" ident "in" expr "{" { statement } "}" ;

assignment = ident "=" expr ;
function = "fun" ident "(" [ ident { "," ident } ] ")" "{" { statement } "}" ;

return = "return" expr ;

expr = assign ;

assign = equality { "=" assign } ;

equality = comparison { ( "!=" | "==" ) comparison } ;

comparison = term { ( ">" | ">=" | "<" | "<=" ) term } ;

term = factor { ( "-" | "+" ) factor } ;

factor = unary { ( "/" | "*" ) unary } ;

unary = ( "!" | "-" ) unary | call ;

call = primary { "(" [ expr { "," expr } ] ")" } ;

primary = NUMBER | STRING | "true" | "false" | "nil" | "(" expr ")" | ident ;

ident = ALPHA { ALPHA | DIGIT } ;

ALPHA = ( "a" ... "z" | "A" ... "Z" | "_" ) ;

DIGIT = "0" ... "9" ;
```
