; ModuleID = 'test/test.dsl'
source_filename = "test/test.dsl"

define void @main() {
  %1 = alloca i32, align 4
  store i32 5, i32* %1, align 4
  br label %2

2:                                                ; preds = %5, %0
  %3 = load i32, i32* %1, align 4
  %4 = icmp ult i32 %3, 5
  br i1 %4, label %5, label %8

5:                                                ; preds = %2
  %6 = load i32, i32* %1, align 4
  %7 = add i32 %6, 1
  store i32 %7, i32* %1, align 4
  br label %2

8:                                                ; preds = %2
  ret void
}
