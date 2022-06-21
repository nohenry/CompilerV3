; ModuleID = 'test/test.dsl'
source_filename = "test/test.dsl"

define void @main(i32 %0) {
  %2 = alloca i32, align 4
  store i32 %0, i32* %2, align 4
  %3 = alloca [3 x i32], align 4
  store [3 x i32] [i32 5, i32 6, i32 7], [3 x i32] %3, align 4
  ret void
}
