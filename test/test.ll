; ModuleID = 'test/test.c'
source_filename = "test/test.c"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc19.29.30140"

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main(i32 %0) #0 !dbg !8 {
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  store i32 0, i32* %2, align 4
  store i32 %0, i32* %3, align 4
  call void @llvm.dbg.declare(metadata i32* %3, metadata !13, metadata !DIExpression()), !dbg !14
  call void @llvm.dbg.declare(metadata i32* %4, metadata !15, metadata !DIExpression()), !dbg !16
  store i32 5, i32* %4, align 4, !dbg !16
  %5 = load i32, i32* %4, align 4, !dbg !17
  %6 = icmp eq i32 %5, 5, !dbg !17
  br i1 %6, label %7, label %8, !dbg !17

7:                                                ; preds = %1
  store i32 6, i32* %4, align 4, !dbg !18
  br label %14, !dbg !21

8:                                                ; preds = %1
  %9 = load i32, i32* %4, align 4, !dbg !22
  %10 = icmp eq i32 %9, 4, !dbg !22
  br i1 %10, label %11, label %12, !dbg !22

11:                                               ; preds = %8
  store i32 0, i32* %4, align 4, !dbg !23
  br label %13, !dbg !26

12:                                               ; preds = %8
  store i32 7, i32* %4, align 4, !dbg !27
  br label %13, !dbg !29

13:                                               ; preds = %12, %11
  br label %14, !dbg !30

14:                                               ; preds = %13, %7
  %15 = load i32, i32* %2, align 4, !dbg !31
  ret i32 %15, !dbg !31
}

; Function Attrs: nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

attributes #0 = { noinline nounwind optnone uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { nounwind readnone speculatable willreturn }

!llvm.dbg.cu = !{!0}
!llvm.module.flags = !{!3, !4, !5, !6}
!llvm.ident = !{!7}

!0 = distinct !DICompileUnit(language: DW_LANG_C99, file: !1, producer: "clang version 11.0.0", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !2, nameTableKind: None)
!1 = !DIFile(filename: "test\\test.c", directory: "D:\\Developement\\Projects\\DSL\\CompilerV3", checksumkind: CSK_MD5, checksum: "c26cbd38e8d4aca2862a3ca2fd945b49")
!2 = !{}
!3 = !{i32 2, !"CodeView", i32 1}
!4 = !{i32 2, !"Debug Info Version", i32 3}
!5 = !{i32 1, !"wchar_size", i32 2}
!6 = !{i32 7, !"PIC Level", i32 2}
!7 = !{!"clang version 11.0.0"}
!8 = distinct !DISubprogram(name: "main", scope: !9, file: !9, line: 2, type: !10, scopeLine: 3, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0, retainedNodes: !2)
!9 = !DIFile(filename: "test/test.c", directory: "D:\\Developement\\Projects\\DSL\\CompilerV3", checksumkind: CSK_MD5, checksum: "c26cbd38e8d4aca2862a3ca2fd945b49")
!10 = !DISubroutineType(types: !11)
!11 = !{!12, !12}
!12 = !DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)
!13 = !DILocalVariable(name: "argc", arg: 1, scope: !8, file: !9, line: 2, type: !12)
!14 = !DILocation(line: 2, scope: !8)
!15 = !DILocalVariable(name: "d", scope: !8, file: !9, line: 4, type: !12)
!16 = !DILocation(line: 4, scope: !8)
!17 = !DILocation(line: 5, scope: !8)
!18 = !DILocation(line: 7, scope: !19)
!19 = distinct !DILexicalBlock(scope: !20, file: !9, line: 6)
!20 = distinct !DILexicalBlock(scope: !8, file: !9, line: 5)
!21 = !DILocation(line: 8, scope: !19)
!22 = !DILocation(line: 9, scope: !20)
!23 = !DILocation(line: 11, scope: !24)
!24 = distinct !DILexicalBlock(scope: !25, file: !9, line: 10)
!25 = distinct !DILexicalBlock(scope: !20, file: !9, line: 9)
!26 = !DILocation(line: 12, scope: !24)
!27 = !DILocation(line: 15, scope: !28)
!28 = distinct !DILexicalBlock(scope: !25, file: !9, line: 14)
!29 = !DILocation(line: 16, scope: !28)
!30 = !DILocation(line: 9, scope: !25)
!31 = !DILocation(line: 17, scope: !8)
