; ModuleID = 'test/test.dsl'
source_filename = "test/test.dsl"

define void @main() {
  br label %1

1:                                                ; preds = %0
  %2 = alloca i32, align 4
  br label %3

3:                                                ; preds = %1
  %4 = call i32 @value1(i32 0, i32 4)
  br label %5

5:                                                ; preds = %3
  store i32 %4, i32* %2, align 4
  br label %6

6:                                                ; preds = %5
  %7 = alloca i32, align 4
  br label %8

8:                                                ; preds = %6
  %9 = call i32 @value1(i32 0, i32 4)
  br label %10

10:                                               ; preds = %8
  store i32 %9, i32* %7, align 4
  ret void
}

define i32 @value1(i32 %0, i32 %1) {
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  store i32 %0, i32* %3, align 4
  store i32 %1, i32* %4, align 4
  %5 = alloca i32, align 4
  %6 = load i32, i32* %3, align 4
  %7 = load i32, i32* %4, align 4
  %8 = add i32 %6, %7
  store i32 %8, i32* %5, align 4
  %9 = load i32, i32* %5, align 4
  ret i32 %9
}
