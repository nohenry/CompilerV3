; ModuleID = 'test.dslUnable to read file!'
source_filename = "test.dslUnable to read file!"

%test-Data1 = type { i32 }
%test-Data2 = type { i32 }

declare void @printf(i8*)

define void @test-Data1-act(%test-Data1* %0) {
  %2 = alloca %test-Data1*, align 8
  store %test-Data1* %0, %test-Data1** %2, align 8
  ret void
}

define void @test-main(i32 %0) {
  %2 = alloca i32, align 4
  store i32 %0, i32* %2, align 4
  %3 = alloca %test-Data2, align 8
  %4 = getelementptr inbounds %test-Data2, %test-Data2* %3, i32 0, i32 0
  store i32 9, i32* %4, align 4
  ret void
}
