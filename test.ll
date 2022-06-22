; ModuleID = 'test/test.dsl'
source_filename = "test/test.dsl"

declare void @printf(i8*)

define void @main(i32 %0) {
  %2 = alloca i32, align 4
  store i32 %0, i32* %2, align 4
  %3 = alloca i32, align 4
  %4 = load i32, i32* %2, align 4
  %5 = icmp sgt i32 %4, 5
  br i1 %5, label %6, label %7

6:                                                ; preds = %1
  br label %10

7:                                                ; preds = %1
  %8 = load i32, i32* %2, align 4
  %9 = add i32 %8, 4
  br label %10

10:                                               ; preds = %7, %6
  %11 = phi i32 [ 4, %6 ], [ %9, %7 ]
  store i32 %11, i32* %3, align 4
  ret void
}
