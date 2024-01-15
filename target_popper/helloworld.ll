; ModuleID = './examples/helloworld.pop'
source_filename = "./examples/helloworld.pop"

%Foo = type { i32, i32 }

declare i32 @println([1 x i8])

declare i32 @print([1 x i8])

define ptr @main() {
entry:
  %let_a = alloca { i32, i32 }, align 8
  store %Foo { i32 1, i32 2 }, ptr %let_a, align 4
  %return = alloca ptr, align 8
  store i32 0, ptr %return, align 4
  ret ptr %return
}
