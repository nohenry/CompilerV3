; ModuleID = 'test/test.c'
source_filename = "test/test.c"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc19.29.30140"

%struct.Data = type { i32, %struct.Data1 }
%struct.Data1 = type { i32, i32 }

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main(i32 %0) #0 {
  %2 = alloca i32, align 4
  %3 = alloca %struct.Data, align 4
  %4 = alloca i32, align 4
  store i32 %0, i32* %2, align 4
  %5 = getelementptr inbounds %struct.Data, %struct.Data* %3, i32 0, i32 0
  %6 = load i32, i32* %2, align 4
  store i32 %6, i32* %5, align 4
  %7 = getelementptr inbounds %struct.Data, %struct.Data* %3, i32 0, i32 1
  %8 = getelementptr inbounds %struct.Data1, %struct.Data1* %7, i32 0, i32 0
  store i32 2, i32* %8, align 4
  %9 = getelementptr inbounds %struct.Data1, %struct.Data1* %7, i32 0, i32 1
  store i32 8, i32* %9, align 4
  call void asm sideeffect "nop", "~{dirflag},~{fpsr},~{flags}"() #1, !srcloc !3
  %10 = getelementptr inbounds %struct.Data, %struct.Data* %3, i32 0, i32 1
  %11 = getelementptr inbounds %struct.Data1, %struct.Data1* %10, i32 0, i32 0
  %12 = load i32, i32* %11, align 4
  store i32 %12, i32* %4, align 4
  ret i32 0
}

attributes #0 = { noinline nounwind optnone uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { nounwind }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 2}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{!"clang version 11.0.0"}
!3 = !{i32 200}
