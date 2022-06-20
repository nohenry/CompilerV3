; ModuleID = 'test/test.dsl'
source_filename = "test/test.dsl"

define void @main() {
  %1 = call i1 @value2(i1 true, i32 4)
  %2 = call i1 @value2(i1 false, i32 0)
  ret void
}

define i1 @value2(i1 %0, i32 %1) {
  %3 = alloca i1, align 1
  %4 = alloca i32, align 4
  store i1 %0, i1* %3, align 1
  store i32 %1, i32* %4, align 4
  %5 = alloca i1, align 1
  %6 = load i1, i1* %3, align 1
  %7 = icmp eq i1 %6, true
  br i1 %7, label %8, label %9

8:                                                ; preds = %2
  br label %10

9:                                                ; preds = %2
  br label %10

10:                                               ; preds = %9, %8
  %11 = phi i1 [ false, %8 ], [ true, %9 ]
  store i1 %11, i1* %5, align 1
  %12 = load i1, i1* %5, align 1
  ret i1 %12
}
