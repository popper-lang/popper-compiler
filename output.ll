; ModuleID = './examples/helloworld.pop'
source_filename = "./examples/helloworld.pop"

@fmt = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
@.str = private unnamed_addr constant [12 x i8] c"Hello World\00", align 1

define i32 @print(ptr %0) {
  %2 = call i32 (ptr, ...) @printf(ptr @fmt, ptr %0)
  ret i32 %2
}

declare i32 @printf(ptr, ...)

define ptr @add(i32 %0, i32 %1) {
entry:
  %add = add i32 %0, %1
  %let_c = alloca i32, align 4
  store i32 %add, ptr %let_c, align 4
  %load = load ptr, ptr %let_c, align 8
  %return = alloca ptr, align 8
  store ptr %load, ptr %return, align 8
  ret ptr %return
}

define ptr @main(i32 %0) {
entry:
  %call = call ptr @add(i32 4, i32 1)
  %call1 = call i32 @print(ptr @.str)
  %return = alloca i32, align 4
  store i32 0, ptr %return, align 4
  ret ptr %return
}

