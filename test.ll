; ModuleID = 'test/test.dsl'
source_filename = "test/test.dsl"

define void @main() {
  %1 = alloca i1, align 1
  br i1 true, label %2, label %3

2:                                                ; preds = %0
  store i32 5, i1* %1, align 4
  br label %4

3:                                                ; preds = %0
  store i32 7, i1* %1, align 4
  br label %4

4:                                                ; preds = %3, %2
  ret void
}
