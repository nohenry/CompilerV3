; ModuleID = 'test'
source_filename = "test"

%core-List = type { i32 }

@0 = private unnamed_addr constant [12 x i8] c"Poop Potato\00", align 1

declare void @printf(i8*)

define void @test-main() {
  %1 = alloca %core-List, align 8
  %2 = getelementptr inbounds %core-List, %core-List* %1, i32 0, i32 0
  store i32 7, i32* %2, align 4
  call void @printf(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @0, i32 0, i32 0))
  ret void
}

define void @main() {
  call void @test-main()
  ret void
}
