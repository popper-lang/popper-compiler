; ModuleID = 'examples/helloworld.pop'
source_filename = "examples/helloworld.pop"

@format_string = private constant [4 x i8] c"%d\0A\00"

declare i32 @printf(i8*, ...)

define i32 @print(i32 %0) {
entry:
  %printf_call = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @format_string, i32 0, i32 0), i32 %0)
  
  ret i32 0
}

define i32 @main(i32 %0) {
entry:
  %call = call i32 @print(i32 99)
  ret i32 0
}

