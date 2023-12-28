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