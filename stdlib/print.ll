
@fmt = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1

define i32 @print(i8* %0) #0 {
  %2 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt, i32 0, i32 0), i8* %0)
  ret i32 %2
}

declare i32 @printf(i8*, ...) #1
