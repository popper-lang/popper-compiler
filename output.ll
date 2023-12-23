; ModuleID = 'examples/helloworld.pop'
source_filename = "examples/helloworld.pop"

declare i32 @printf(i8, ...)

define i32 @print(i32 %0) {
entry:
  %printf_call = call i32 (i8, ...) @printf([4 x i8] c"%d\0A\00", i32 %0)
  ret void
}

define i32 @main(i32 %0) {
entry:
  %call = call i32 @print(i32 3)
}

