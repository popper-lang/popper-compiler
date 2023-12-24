; ModuleID = 'examples/helloworld.pop'
source_filename = "examples/helloworld.pop"

define i32 @add(i32 %0, i32 %1) {
entry:
  %alloca = alloca i32, align 4
  store i32 %0, ptr %alloca, align 4
  %alloca1 = alloca i32, align 4
  store i32 %1, ptr %alloca1, align 4
  %load = load ptr, ptr %alloca, align 8
  %load2 = load ptr, ptr %alloca1, align 8
  %add = add ptr %load, %load2
  %let_c = alloca i32, align 4
  store ptr %add, ptr %let_c, align 8
}

define i32 @main(i32 %0) {
entry:
  %alloca = alloca i32, align 4
  store i32 %0, ptr %alloca, align 4
  %call = call i32 @add(i32 4, i32 1)
  ret i32 0
}

