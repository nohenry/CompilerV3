; ModuleID = 'test/test.dsl'
source_filename = "test/test.dsl"

@0 = private unnamed_addr constant [13 x i8] c"Hello World!\00", align 1

declare void @printf(i8*)

define void @main(i32 %0) {
  %2 = alloca i32, align 4
  store i32 %0, i32* %2, align 4
  call void @printf(i8* getelementptr inbounds ([13 x i8], [13 x i8]* @0, i32 0, i32 0))
  ret void
}
