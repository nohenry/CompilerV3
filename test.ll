; ModuleID = 'test.dslUnable to read file!'
source_filename = "test.dslUnable to read file!"

%test-Data2 = type { i32, i1 }
%test-Data3 = type { i1 }

declare void @printf(i8*)

define void @test-main(i32 %0) {
  %2 = alloca i32, align 4
  store i32 %0, i32* %2, align 4
  %3 = alloca %test-Data2, align 8
  %4 = getelementptr inbounds %test-Data2, %test-Data2* %3, i32 0, i32 1
  store i1 true, i1* %4, align 1
  %5 = getelementptr inbounds %test-Data2, %test-Data2* %3, i32 0, i32 0
  store i32 0, i32* %5, align 4
  %6 = alloca %test-Data3, align 8
  %7 = getelementptr inbounds %test-Data3, %test-Data3* %6, i32 0, i32 0
  store i1 false, i1* %7, align 1
  ret void
}
