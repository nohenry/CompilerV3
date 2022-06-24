; ModuleID = 'test.dslUnable to read file!'
source_filename = "test.dslUnable to read file!"

%test-Data1 = type { i32 }

declare void @printf(i8*)

define void @test-main(i32 %0) {
  %2 = alloca i32, align 4
  store i32 %0, i32* %2, align 4
  %3 = alloca %test-Data1, align 8
  %4 = getelementptr inbounds %test-Data1, %test-Data1* %3, i32 0, i32 0
  store i32 0, i32* %4, align 4
  ret void
}
