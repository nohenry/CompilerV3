; ModuleID = 'test/test.cpp'
source_filename = "test/test.cpp"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc19.29.30140"

%rtti.CompleteObjectLocator = type { i32, i32, i32, i32, i32, i32 }
%rtti.TypeDescriptor10 = type { i8**, i8*, [11 x i8] }
%rtti.ClassHierarchyDescriptor = type { i32, i32, i32, i32 }
%rtti.BaseClassDescriptor = type { i32, i32, i32, i32, i32, i32, i32 }
%rtti.TypeDescriptor11 = type { i8**, i8*, [12 x i8] }
%class.Write = type { i32 (...)** }
%class.Data = type { %class.Write }

$"??0Data@@QEAA@XZ" = comdat any

$"??0Write@@QEAA@XZ" = comdat any

$"?write@Data@@EEAAXXZ" = comdat any

$"??_7Data@@6B@" = comdat largest

$"??_R4Data@@6B@" = comdat any

$"??_R0?AVData@@@8" = comdat any

$"??_R3Data@@8" = comdat any

$"??_R2Data@@8" = comdat any

$"??_R1A@?0A@EA@Data@@8" = comdat any

$"??_R1A@?0A@EA@Write@@8" = comdat any

$"??_R0?AVWrite@@@8" = comdat any

$"??_R3Write@@8" = comdat any

$"??_R2Write@@8" = comdat any

$"??_7Write@@6B@" = comdat largest

$"??_R4Write@@6B@" = comdat any

@0 = private unnamed_addr constant { [2 x i8*] } { [2 x i8*] [i8* bitcast (%rtti.CompleteObjectLocator* @"??_R4Data@@6B@" to i8*), i8* bitcast (void (%class.Data*)* @"?write@Data@@EEAAXXZ" to i8*)] }, comdat($"??_7Data@@6B@")
@"??_R4Data@@6B@" = linkonce_odr constant %rtti.CompleteObjectLocator { i32 1, i32 0, i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor10* @"??_R0?AVData@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3Data@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.CompleteObjectLocator* @"??_R4Data@@6B@" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_7type_info@@6B@" = external constant i8*
@"??_R0?AVData@@@8" = linkonce_odr global %rtti.TypeDescriptor10 { i8** @"??_7type_info@@6B@", i8* null, [11 x i8] c".?AVData@@\00" }, comdat
@__ImageBase = external dso_local constant i8
@"??_R3Data@@8" = linkonce_odr constant %rtti.ClassHierarchyDescriptor { i32 0, i32 0, i32 2, i32 trunc (i64 sub nuw nsw (i64 ptrtoint ([3 x i32]* @"??_R2Data@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R2Data@@8" = linkonce_odr constant [3 x i32] [i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@Data@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@Write@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0], comdat
@"??_R1A@?0A@EA@Data@@8" = linkonce_odr constant %rtti.BaseClassDescriptor { i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor10* @"??_R0?AVData@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 1, i32 0, i32 -1, i32 0, i32 64, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3Data@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R1A@?0A@EA@Write@@8" = linkonce_odr constant %rtti.BaseClassDescriptor { i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor11* @"??_R0?AVWrite@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0, i32 0, i32 -1, i32 0, i32 64, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3Write@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R0?AVWrite@@@8" = linkonce_odr global %rtti.TypeDescriptor11 { i8** @"??_7type_info@@6B@", i8* null, [12 x i8] c".?AVWrite@@\00" }, comdat
@"??_R3Write@@8" = linkonce_odr constant %rtti.ClassHierarchyDescriptor { i32 0, i32 0, i32 1, i32 trunc (i64 sub nuw nsw (i64 ptrtoint ([2 x i32]* @"??_R2Write@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R2Write@@8" = linkonce_odr constant [2 x i32] [i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@Write@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0], comdat
@1 = private unnamed_addr constant { [2 x i8*] } { [2 x i8*] [i8* bitcast (%rtti.CompleteObjectLocator* @"??_R4Write@@6B@" to i8*), i8* bitcast (void ()* @_purecall to i8*)] }, comdat($"??_7Write@@6B@")
@"??_R4Write@@6B@" = linkonce_odr constant %rtti.CompleteObjectLocator { i32 1, i32 0, i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor11* @"??_R0?AVWrite@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3Write@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.CompleteObjectLocator* @"??_R4Write@@6B@" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat

@"??_7Data@@6B@" = unnamed_addr alias i8*, getelementptr inbounds ({ [2 x i8*] }, { [2 x i8*] }* @0, i32 0, i32 0, i32 1)
@"??_7Write@@6B@" = unnamed_addr alias i8*, getelementptr inbounds ({ [2 x i8*] }, { [2 x i8*] }* @1, i32 0, i32 0, i32 1)

; Function Attrs: noinline optnone uwtable
define dso_local void @af(%class.Write* %0) #0 {
  %2 = alloca %class.Write*, align 8
  store %class.Write* %0, %class.Write** %2, align 8
  %3 = load %class.Write*, %class.Write** %2, align 8
  %4 = bitcast %class.Write* %3 to void (%class.Write*)***
  %5 = load void (%class.Write*)**, void (%class.Write*)*** %4, align 8
  %6 = getelementptr inbounds void (%class.Write*)*, void (%class.Write*)** %5, i64 0
  %7 = load void (%class.Write*)*, void (%class.Write*)** %6, align 8
  call void %7(%class.Write* %3)
  ret void
}

; Function Attrs: noinline norecurse optnone uwtable
define dso_local i32 @main() #1 {
  %1 = alloca %class.Data, align 8
  %2 = call %class.Data* @"??0Data@@QEAA@XZ"(%class.Data* %1) #3
  %3 = bitcast %class.Data* %1 to %class.Write*
  call void @af(%class.Write* %3)
  ret i32 0
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %class.Data* @"??0Data@@QEAA@XZ"(%class.Data* returned %0) unnamed_addr #2 comdat align 2 {
  %2 = alloca %class.Data*, align 8
  store %class.Data* %0, %class.Data** %2, align 8
  %3 = load %class.Data*, %class.Data** %2, align 8
  %4 = bitcast %class.Data* %3 to %class.Write*
  %5 = call %class.Write* @"??0Write@@QEAA@XZ"(%class.Write* %4) #3
  %6 = bitcast %class.Data* %3 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7Data@@6B@" to i32 (...)**), i32 (...)*** %6, align 8
  ret %class.Data* %3
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %class.Write* @"??0Write@@QEAA@XZ"(%class.Write* returned %0) unnamed_addr #2 comdat align 2 {
  %2 = alloca %class.Write*, align 8
  store %class.Write* %0, %class.Write** %2, align 8
  %3 = load %class.Write*, %class.Write** %2, align 8
  %4 = bitcast %class.Write* %3 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7Write@@6B@" to i32 (...)**), i32 (...)*** %4, align 8
  ret %class.Write* %3
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"?write@Data@@EEAAXXZ"(%class.Data* %0) unnamed_addr #2 comdat align 2 {
  %2 = alloca %class.Data*, align 8
  store %class.Data* %0, %class.Data** %2, align 8
  %3 = load %class.Data*, %class.Data** %2, align 8
  ret void
}

declare dso_local void @_purecall() unnamed_addr

attributes #0 = { noinline optnone uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { noinline norecurse optnone uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #2 = { noinline nounwind optnone uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #3 = { nounwind }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 2}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{!"clang version 11.0.0"}
