; ModuleID = 'test/test.dsl'
source_filename = "test/test.dsl"

define void @main() {
  %1 = alloca [3 x i32], align 4
  store [3 x i32] [i32 5, i32 6, i32 9], [3 x i32]* %1, align 4
  %2 = alloca i32, align 4
  %3 = getelementptr inbounds [3 x i32], [3 x i32]* %1, i64 0, i32 2
  %4 = load i32, i32* %3, align 4
  store i32 %4, i32* %2, align 4
  ret void
}
