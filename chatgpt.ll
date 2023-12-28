; ModuleID = 'examples/helloworld.pop'
source_filename = "examples/helloworld.pop"

@fmt = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
@.str = private unnamed_addr constant [4 x i8] c"POP\00", align 1
@.str.1 = private unnamed_addr constant [13 x i8] c"Hello, World\00", align 1

declare i32 @printf(i8*, ...)

define i32 @print(i8* %str) {
  %1 = call i32 (i8*, ...) @printf(ptr @fmt, i8* %str)
  ret i32 %1
}

define i32 @main(i32 %0) {
  entry:
    %let_d = alloca [13 x i8], align 1
    %ptr_str = getelementptr [13 x i8], ptr %let_d, i32 0, i32 0
    call void @copy_string(ptr %ptr_str, ptr getelementptr inbounds ([4 x i8], ptr @.str, i32 0, i32 0), i32 4)
    %call = call i32 @print(ptr getelementptr inbounds ([13 x i8], ptr @.str.1, i32 0, i32 0))
    %call1 = call i32 @print(ptr %ptr_str)
    %return = alloca i32, align 4
    store i32 0, ptr %return, align 4
    ret i32 0
}

define void @copy_string(ptr %dest, ptr %src, i32 %length) {
  %i = alloca i32, align 4
  store i32 0, ptr %i, align 4
  br label %loop

loop:
  %index = load i32, ptr %i, align 4
  %ptr_src = getelementptr i8, ptr %src, i32 %index
  %char = load i8, i8* %ptr_src, align 1
  %ptr_dest = getelementptr i8, ptr %dest, i32 %index
  store i8 %char, ptr %ptr_dest, align 1
  %new_index = add i32 %index, 1
  store i32 %new_index, i32* %i, align 4

  %condition = icmp slt i32 %new_index, %length
  br i1 %condition, label %loop, label %end

end:
  ret void
}
