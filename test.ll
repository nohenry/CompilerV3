; ModuleID = 'test.dslUnable to read file!'
source_filename = "test.dslUnable to read file!"

%test-Data = type { i32 }

declare void @printf(i8*)

define void @core-uint32-inc(i32* %0) {
  %2 = alloca i32*, align 8
  store i32* %0, i32** %2, align 8
  ret void
}

define void @test-Data-act(%test-Data* %0) {
  %2 = alloca %test-Data*, align 8
  store %test-Data* %0, %test-Data** %2, align 8
  %3 = load %test-Data*, %test-Data** %2, align 8
  %4 = getelementptr inbounds %test-Data, %test-Data* %3, i32 0, i32 0
  store i32 6, i32* %4, align 4
  ret void
}

define void @test-main(i32 %0) {
  %2 = alloca i32, align 4
  store i32 %0, i32* %2, align 4
  %3 = alloca %test-Data, align 8
  %4 = getelementptr inbounds %test-Data, %test-Data* %3, i32 0, i32 0
  store i32 9, i32* %4, align 4
  %5 = getelementptr inbounds %test-Data, %test-Data* %3, i32 0, i32 0
  store i32 8, i32* %5, align 4
  ret void
}
