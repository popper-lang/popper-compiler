; ModuleID = 'examples/helloworld.pop'
source_filename = "examples/helloworld.pop"

declare i32 @print(ptr)

define ptr @main(i32 %0) {
entry:
  %string_literal = alloca [14 x i8], align 1
  store [14 x i8] c"Hello, World!\00", ptr %string_literal, align 1
  %call_print_ = call i32 @print(ptr %string_literal)
  %string_literal1 = alloca [4 x i8], align 1
  store [4 x i8] c"Foo\00", ptr %string_literal1, align 1
  %call_print_2 = call i32 @print(ptr %string_literal1)
  %string_literal3 = alloca [4 x i8], align 1
  store [4 x i8] c"Bar\00", ptr %string_literal3, align 1
  %call_print_4 = call i32 @print(ptr %string_literal3)
  %return = alloca ptr, align 8
  store i32 0, ptr %return, align 4
  ret ptr %return
}
