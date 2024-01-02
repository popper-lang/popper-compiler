; ModuleID = 'examples/helloworld.pop'
source_filename = "examples/helloworld.pop"

declare i32 @println([1 x i8])

declare i32 @print([1 x i8])

define ptr @main() {
entry:
  %string_literal = alloca [14 x i8], align 1
  store [14 x i8] c"Hello, world!\00", ptr %string_literal, align 1
  %call_println_ = call i32 @println(ptr %string_literal)
  %return = alloca ptr, align 8
  store i32 0, ptr %return, align 4
  ret ptr %return
}
