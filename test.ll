; ModuleID = 'test/test.dsl'
source_filename = "test/test.dsl"

define void @main() {
  %1 = alloca i32, align 4
  store i32 5, i32* %1, align 4
  %2 = load i32, i32* %1, align 4
  %3 = icmp eq i32 %2, 5
  br i1 %3, label %4, label %5

4:                                                ; preds = %0
  store i32 6, i32* %1, align 4
  br label %10

5:                                                ; preds = %0
  %6 = load i32, i32* %1, align 4
  %7 = icmp eq i32 %6, 4
  br i1 %7, label %8, label %9

8:                                                ; preds = %5
  store i32 0, i32* %1, align 4
  br label %10

9:                                                ; preds = %5
  store i32 7, i32* %1, align 4
  br label %10, !Potato !0

10:                                               ; preds = %10, %9, %8, %4
  br label %10, !Potato !0
  ret void
}

!0 = !{!"Hello"}
