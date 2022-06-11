; ModuleID = 'test/test.dsl'
source_filename = "test/test.dsl"

define void @main() {
  br label %1

1:                                                ; preds = %0
  %2 = alloca i32, align 4
  br label %3

3:                                                ; preds = %1
  br label %4

4:                                                ; preds = %3
  store i32 10, i32* %2, align 4
  ret void
}
