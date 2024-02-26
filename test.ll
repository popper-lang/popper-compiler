; ModuleID = 'helloworld'
source_filename = "helloworld"

@.str = private unnamed_addr constant [15 x i8] c"hello wrld %d\0A\00", align 1

declare i32 @printf(i8* noalias nocapture, ...)

define i32 @main() {
entry:
 %c = alloca [3 x i32], align 4
 store [3 x i32] [i32 82, i32 4, i32 5], ptr %c, align 4
 %d = getelementptr i32, ptr %c, i32 2
 %0 = load i32, ptr %d, align 4

  %call = call i32 (i8*, ...) @printf(ptr @.str, i32 %0)


  ret i32 0
}
