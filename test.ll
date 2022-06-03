; ModuleID = 'test/test.dsl'
source_filename = "test/test.dsl"

define void @main() {
  %1 = alloca i32, align 4
  store i32 5, i32* %1, align 4
  br i1 true, label %2, label %3

2:                                                ; preds = %0
  store i32 6, i32* %1, align 4
  br label %3

3:                                                ; preds = %2, %0
  ret void
}
