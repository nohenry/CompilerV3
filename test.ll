; ModuleID = 'test/test.dsl'
source_filename = "test/test.dsl"

%Data = type { i32, %Data1 }
%Data1 = type { i32, i32 }

declare void @printf(i8*)

define void @main(i32 %0) {
  %2 = alloca i32, align 4
  store i32 %0, i32* %2, align 4
  %3 = alloca %Data, align 8
  %4 = getelementptr inbounds %Data, %Data* %3, i32 0, i32 1
  %5 = getelementptr inbounds %Data1, %Data1* %4, i32 0, i32 0
  store i32 15, i32* %5, align 4
  %6 = getelementptr inbounds %Data1, %Data1* %4, i32 0, i32 1
  store i32 20, i32* %6, align 4
  %7 = getelementptr inbounds %Data, %Data* %3, i32 0, i32 0
  store i32 10, i32* %7, align 4
  %8 = alloca %Data1, align 8
  %9 = getelementptr inbounds %Data, %Data* %3, i32 0, i32 1
  %10 = load %Data1, %Data1* %9, align 8
  store %Data1 %10, %Data1* %8, align 4
  ret void
}
