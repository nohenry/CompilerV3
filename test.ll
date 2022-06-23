; ModuleID = 'test.dslUnable to read file!'
source_filename = "test.dslUnable to read file!"

%test-Data = type { i32, i32 }

declare void @printf(i8*)

define void @test-Data-potato() {
  ret void
}

define void @test-main(i32 %0) {
  %2 = alloca i32, align 4
  store i32 %0, i32* %2, align 4
  %3 = alloca %test-Data, align 8
  %4 = getelementptr inbounds %test-Data, %test-Data* %3, i32 0, i32 0
  store i32 10, i32* %4, align 4
  %5 = getelementptr inbounds %test-Data, %test-Data* %3, i32 0, i32 1
  store i32 20, i32* %5, align 4
  call void @test-Data-potato()
  ret void
}
