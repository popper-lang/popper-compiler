; ModuleID = './examples/helloworld.pop'
source_filename = "./examples/helloworld.pop"

@fmt = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1

define i32 @print(ptr %0) {
  %2 = call i32 (ptr, ...) @printf(ptr @fmt, ptr %0)
  ret i32 %2
}

declare i32 @printf(ptr, ...)

define void @copy_string(ptr %dest, ptr %src, i32 %length) {
  %i = alloca i32, align 4
  store i32 0, ptr %i, align 4
  br label %loop

loop:                                             ; preds = %loop, %0
  %index = load i32, ptr %i, align 4
  %ptr_src = getelementptr i8, ptr %src, i32 %index
  %char = load i8, ptr %ptr_src, align 1
  %ptr_dest = getelementptr i8, ptr %dest, i32 %index
  store i8 %char, ptr %ptr_dest, align 1
  %new_index = add i32 %index, 1
  store i32 %new_index, ptr %i, align 4
  %condition = icmp slt i32 %new_index, %length
  br i1 %condition, label %loop, label %end

end:                                              ; preds = %loop
  ret void
}

define ptr @main(i32 %0) {
entry:
  %let_d = alloca [6 x i8], align 1
  store [6 x i8] c"hello\00", ptr %let_d, align 1
  %let_k = alloca [6 x i8], align 1
  store [6 x i8] c"world\00", ptr %let_k, align 1
  %let_x = alloca [4 x i8], align 1
  store [4 x i8] c"hie\00", ptr %let_x, align 1
  %call = call i32 @print(ptr %let_d)
  %call1 = call i32 @print(ptr %let_k)
  %call2 = call i32 @print(ptr %let_x)
  %string_literal = alloca [12 x i8], align 1
  store [12 x i8] c"Hello World\00", ptr %string_literal, align 1
  %call3 = call i32 @print(ptr %string_literal)
  %return = alloca i32, align 4
  store i32 0, ptr %return, align 4
  ret ptr %return
}

