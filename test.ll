; ModuleID = 'test/test.dsl'
source_filename = "test/test.dsl"

define void @main() {
  %1 = alloca i32, align 4
  store i32 5, i32* %1, align 4
  store i32 4, i32* %1, align 4
  ret void
}
