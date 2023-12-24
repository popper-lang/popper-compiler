@format_string = constant [4 x i8] c"%s\0"

declare i32 @printf(i8*, ...) ; external( C function )

define i32 @print(i8* %str) {
  %fmt = bitcast [4 x i8]* @format_string to i8* ; cast to i8*
  %res = call i32 (i8*, ...) @printf(i8* %fmt, i8* %str) ; call printf
  ret i32 %res
}
