; ModuleID = 'test.36762699-cgu.0'
source_filename = "test.36762699-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

%"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>" = type { %"core::marker::PhantomData<std::ffi::os_str::OsString>", %"alloc::alloc::Global", i64*, i64, %"std::ffi::os_str::OsString"*, %"std::ffi::os_str::OsString"* }
%"core::marker::PhantomData<std::ffi::os_str::OsString>" = type {}
%"alloc::alloc::Global" = type {}
%"std::ffi::os_str::OsString" = type { %"std::sys::windows::os_str::Buf" }
%"std::sys::windows::os_str::Buf" = type { %"std::sys_common::wtf8::Wtf8Buf" }
%"std::sys_common::wtf8::Wtf8Buf" = type { %"alloc::vec::Vec<u8>" }
%"alloc::vec::Vec<u8>" = type { { i8*, i64 }, i64 }
%"core::option::Option<usize>::Some" = type { [1 x i64], i64 }
%"core::panic::location::Location" = type { { [0 x i8]*, i64 }, i32, i32 }
%"core::result::Result<core::alloc::layout::Layout, core::alloc::layout::LayoutError>::Err" = type { %"core::alloc::layout::LayoutError" }
%"core::alloc::layout::LayoutError" = type {}
%"core::fmt::Formatter" = type { { i64, i64 }, { i64, i64 }, { {}*, [3 x i64]* }, i32, i32, i8, [7 x i8] }
%"std::env::Args" = type { %"std::env::ArgsOs" }
%"std::env::ArgsOs" = type { %"std::sys::windows::args::Args" }
%"std::sys::windows::args::Args" = type { %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>" }
%"core::mem::maybe_uninit::MaybeUninit<alloc::alloc::Global>" = type { [0 x i8] }
%"core::ptr::metadata::PtrRepr<[u8]>" = type { [2 x i64] }
%"core::ptr::metadata::PtrRepr<[std::ffi::os_str::OsString]>" = type { [2 x i64] }
%"core::option::Option<core::fmt::Arguments>" = type { {}*, [5 x i64] }
%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Continue" = type { [1 x i64], i64 }
%"core::result::Result<usize, core::alloc::layout::LayoutError>::Err" = type { [8 x i8], %"core::alloc::layout::LayoutError" }
%"core::result::Result<usize, core::alloc::layout::LayoutError>::Ok" = type { [1 x i64], i64 }
%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>" = type { {}*, [2 x i64] }
%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some" = type { { i8*, { i64, i64 } } }
%"core::fmt::builders::DebugTuple" = type { %"core::fmt::Formatter"*, i64, i8, i8, [6 x i8] }
%"core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>::Err" = type { %"core::alloc::layout::LayoutError" }
%"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Break" = type { [8 x i8], %"core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>::Err" }

@vtable.0 = private unnamed_addr constant <{ i8*, [16 x i8], i8*, i8*, i8* }> <{ i8* bitcast (void (i64**)* @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h474366d8b931e683E" to i8*), [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", i8* bitcast (i32 (i64**)* @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hc4726b8faa2f60f5E" to i8*), i8* bitcast (i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h87c5ae68f7928da5E" to i8*), i8* bitcast (i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h87c5ae68f7928da5E" to i8*) }>, align 8
@alloc14 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/335ffbfa547df94ac236f5c56130cecf99c8d82b\\library\\core\\src\\ptr\\mod.rs" }>, align 1
@alloc15 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [75 x i8] }>, <{ [75 x i8] }>* @alloc14, i32 0, i32 0, i32 0), [16 x i8] c"K\00\00\00\00\00\00\00\95\02\00\00\0D\00\00\00" }>, align 8
@alloc16 = private unnamed_addr constant <{ [73 x i8] }> <{ [73 x i8] c"assertion failed: 0 < pointee_size && pointee_size <= isize::MAX as usize" }>, align 1
@alloc17 = private unnamed_addr constant <{ [81 x i8] }> <{ [81 x i8] c"/rustc/335ffbfa547df94ac236f5c56130cecf99c8d82b\\library\\core\\src\\ptr\\const_ptr.rs" }>, align 1
@alloc18 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [81 x i8] }>, <{ [81 x i8] }>* @alloc17, i32 0, i32 0, i32 0), [16 x i8] c"Q\00\00\00\00\00\00\00\C1\01\00\00\09\00\00\00" }>, align 8
@alloc19 = private unnamed_addr constant <{ [90 x i8] }> <{ [90 x i8] c"/rustc/335ffbfa547df94ac236f5c56130cecf99c8d82b\\library\\core\\src\\iter\\traits\\exact_size.rs" }>, align 1
@alloc20 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [90 x i8] }>, <{ [90 x i8] }>* @alloc19, i32 0, i32 0, i32 0), [16 x i8] c"Z\00\00\00\00\00\00\00l\00\00\00\09\00\00\00" }>, align 8
@alloc21 = private unnamed_addr constant <{ [80 x i8] }> <{ [80 x i8] c"/rustc/335ffbfa547df94ac236f5c56130cecf99c8d82b\\library\\core\\src\\alloc\\layout.rs" }>, align 1
@alloc22 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [80 x i8] }>, <{ [80 x i8] }>* @alloc21, i32 0, i32 0, i32 0), [16 x i8] c"P\00\00\00\00\00\00\00\98\01\00\00\1A\00\00\00" }>, align 8
@vtable.1 = private unnamed_addr constant <{ i8*, [16 x i8], i8* }> <{ i8* bitcast (void ({ i64, i64 }**)* @"_ZN4core3ptr58drop_in_place$LT$$RF$core..option..Option$LT$usize$GT$$GT$17h19ecb1043aee27f1E" to i8*), [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", i8* bitcast (i1 ({ i64, i64 }**, %"core::fmt::Formatter"*)* @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hae5318a5530776d7E" to i8*) }>, align 8
@alloc26 = private unnamed_addr constant <{ [76 x i8] }> <{ [76 x i8] c"/rustc/335ffbfa547df94ac236f5c56130cecf99c8d82b\\library\\alloc\\src\\raw_vec.rs" }>, align 1
@alloc27 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [76 x i8] }>, <{ [76 x i8] }>* @alloc26, i32 0, i32 0, i32 0), [16 x i8] c"L\00\00\00\00\00\00\00\F7\00\00\00;\00\00\00" }>, align 8
@alloc28 = private unnamed_addr constant <{ [4 x i8] }> <{ [4 x i8] c"Some" }>, align 1
@vtable.2 = private unnamed_addr constant <{ i8*, [16 x i8], i8* }> <{ i8* bitcast (void (i64**)* @"_ZN4core3ptr30drop_in_place$LT$$RF$usize$GT$17hdaf709f8f2433fa4E" to i8*), [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", i8* bitcast (i1 (i64**, %"core::fmt::Formatter"*)* @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hb270a7a7e9077700E" to i8*) }>, align 8
@alloc32 = private unnamed_addr constant <{ [4 x i8] }> <{ [4 x i8] c"None" }>, align 1

; <alloc::vec::into_iter::IntoIter<T,A> as core::iter::traits::iterator::Iterator>::size_hint
; Function Attrs: inlinehint uwtable
define internal void @"_ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9size_hint17h8c861d489f5ed3e2E"({ i64, { i64, i64 } }* sret({ i64, { i64, i64 } }) %0, %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"* align 8 %self) unnamed_addr #0 {
start:
  %_12 = alloca { i64, i64 }, align 8
  %exact = alloca i64, align 8
  br label %bb1

bb1:                                              ; preds = %start
  %1 = icmp eq i64 24, 0
  br i1 %1, label %bb2, label %bb4

bb2:                                              ; preds = %bb1
  %2 = getelementptr inbounds %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>", %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"* %self, i32 0, i32 5
  %_5 = load %"std::ffi::os_str::OsString"*, %"std::ffi::os_str::OsString"** %2, align 8
  %_4 = ptrtoint %"std::ffi::os_str::OsString"* %_5 to i64
  %3 = getelementptr inbounds %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>", %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"* %self, i32 0, i32 4
  %_7 = load %"std::ffi::os_str::OsString"*, %"std::ffi::os_str::OsString"** %3, align 8
  %_6 = ptrtoint %"std::ffi::os_str::OsString"* %_7 to i64
  %4 = sub i64 %_4, %_6
  store i64 %4, i64* %exact, align 8
  br label %bb3

bb4:                                              ; preds = %bb1
  %5 = getelementptr inbounds %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>", %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"* %self, i32 0, i32 5
  %_9 = load %"std::ffi::os_str::OsString"*, %"std::ffi::os_str::OsString"** %5, align 8
  %6 = getelementptr inbounds %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>", %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"* %self, i32 0, i32 4
  %_10 = load %"std::ffi::os_str::OsString"*, %"std::ffi::os_str::OsString"** %6, align 8
; call core::ptr::const_ptr::<impl *const T>::offset_from
  %_8 = call i64 @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$11offset_from17h8b15a4e317183e25E"(%"std::ffi::os_str::OsString"* %_9, %"std::ffi::os_str::OsString"* %_10)
  br label %bb5

bb5:                                              ; preds = %bb4
  store i64 %_8, i64* %exact, align 8
  br label %bb6

bb6:                                              ; preds = %bb3, %bb5
  %_11 = load i64, i64* %exact, align 8
  %_13 = load i64, i64* %exact, align 8
  %7 = bitcast { i64, i64 }* %_12 to %"core::option::Option<usize>::Some"*
  %8 = getelementptr inbounds %"core::option::Option<usize>::Some", %"core::option::Option<usize>::Some"* %7, i32 0, i32 1
  store i64 %_13, i64* %8, align 8
  %9 = bitcast { i64, i64 }* %_12 to i64*
  store i64 1, i64* %9, align 8
  %10 = bitcast { i64, { i64, i64 } }* %0 to i64*
  store i64 %_11, i64* %10, align 8
  %11 = getelementptr inbounds { i64, { i64, i64 } }, { i64, { i64, i64 } }* %0, i32 0, i32 1
  %12 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_12, i32 0, i32 0
  %13 = load i64, i64* %12, align 8, !range !2, !noundef !3
  %14 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_12, i32 0, i32 1
  %15 = load i64, i64* %14, align 8
  %16 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %11, i32 0, i32 0
  store i64 %13, i64* %16, align 8
  %17 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %11, i32 0, i32 1
  store i64 %15, i64* %17, align 8
  ret void

bb3:                                              ; preds = %bb2
  br label %bb6
}

; <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
; Function Attrs: inlinehint uwtable
define internal i8* @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17h2eb096bfec32d0f7E"(i8* %unique) unnamed_addr #0 {
start:
; call core::ptr::unique::Unique<T>::as_ptr
  %_2 = call i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h9206e7cf0f3ae9e0E"(i8* %unique)
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::non_null::NonNull<T>::new_unchecked
  %0 = call i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17heb024f450d5f02e1E"(i8* %_2)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i8* %0
}

; <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
; Function Attrs: inlinehint uwtable
define internal { i64, i64 } @"_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17hc7c150c0a4246803E"(%"core::panic::location::Location"* align 8 %0) unnamed_addr #0 {
start:
  %1 = alloca { i64, i64 }, align 8
; call <T as core::convert::From<T>>::from
  call void @"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17hbda7aff5bc07100dE"()
  br label %bb1

bb1:                                              ; preds = %start
  %2 = bitcast { i64, i64 }* %1 to %"core::result::Result<core::alloc::layout::Layout, core::alloc::layout::LayoutError>::Err"*
  %3 = bitcast %"core::result::Result<core::alloc::layout::Layout, core::alloc::layout::LayoutError>::Err"* %2 to %"core::alloc::layout::LayoutError"*
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %1, i32 0, i32 1
  store i64 0, i64* %4, align 8
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %1, i32 0, i32 0
  %6 = load i64, i64* %5, align 8
  %7 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %1, i32 0, i32 1
  %8 = load i64, i64* %7, align 8
  %9 = insertvalue { i64, i64 } undef, i64 %6, 0
  %10 = insertvalue { i64, i64 } %9, i64 %8, 1
  ret { i64, i64 } %10
}

; <<alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop::DropGuard<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define internal void @"_ZN157_$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hff8f95d3da5d9bebE"(i64** align 8 %self) unnamed_addr #1 personality i32 (...)* @__CxxFrameHandler3 {
start:
  %_10 = alloca i8, align 1
  %_5 = alloca { i64*, i64 }, align 8
  store i8 0, i8* %_10, align 1
  %0 = bitcast i64** %self to %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"**
  %1 = load %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"*, %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"** %0, align 8, !nonnull !3, !align !4, !noundef !3
  %_4 = bitcast %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"* %1 to %"alloc::alloc::Global"*
  store i8 1, i8* %_10, align 1
; call core::ptr::read
  call void @_ZN4core3ptr4read17h24ab8ebef5d9cb58E(%"alloc::alloc::Global"* %_4)
  br label %bb1

bb1:                                              ; preds = %start
  %2 = bitcast i64** %self to %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"**
  %3 = load %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"*, %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"** %2, align 8, !nonnull !3, !align !4, !noundef !3
  %4 = bitcast %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"* %3 to i64**
  %_7 = load i64*, i64** %4, align 8, !nonnull !3, !noundef !3
; invoke core::ptr::non_null::NonNull<T>::as_ptr
  %_6 = invoke %"std::ffi::os_str::OsString"* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h7ef62d35e080de5eE"(i64* %_7)
          to label %bb2 unwind label %funclet_bb7

bb7:                                              ; preds = %funclet_bb7
  %5 = load i8, i8* %_10, align 1, !range !5, !noundef !3
  %6 = trunc i8 %5 to i1
  br i1 %6, label %bb6, label %bb5

funclet_bb7:                                      ; preds = %bb3, %bb2, %bb1
  %cleanuppad = cleanuppad within none []
  br label %bb7

bb2:                                              ; preds = %bb1
  %7 = bitcast i64** %self to %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"**
  %8 = load %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"*, %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"** %7, align 8, !nonnull !3, !align !4, !noundef !3
  %9 = getelementptr inbounds %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>", %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"* %8, i32 0, i32 3
  %_8 = load i64, i64* %9, align 8
  store i8 0, i8* %_10, align 1
; invoke alloc::raw_vec::RawVec<T,A>::from_raw_parts_in
  %10 = invoke { i64*, i64 } @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$17from_raw_parts_in17h548863d6577571bdE"(%"std::ffi::os_str::OsString"* %_6, i64 %_8)
          to label %bb3 unwind label %funclet_bb7

bb3:                                              ; preds = %bb2
  store { i64*, i64 } %10, { i64*, i64 }* %_5, align 8
; invoke core::ptr::drop_in_place<alloc::raw_vec::RawVec<std::ffi::os_str::OsString>>
  invoke void @"_ZN4core3ptr77drop_in_place$LT$alloc..raw_vec..RawVec$LT$std..ffi..os_str..OsString$GT$$GT$17h790dc7b886675322E"({ i64*, i64 }* %_5)
          to label %bb4 unwind label %funclet_bb7

bb4:                                              ; preds = %bb3
  store i8 0, i8* %_10, align 1
  ret void

bb5:                                              ; preds = %bb6, %bb7
  cleanupret from %cleanuppad unwind to caller

bb6:                                              ; preds = %bb7
  br label %bb5
}

; std::sys_common::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline uwtable
define internal void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hda34cccff0e9eda2E(void ()* %f) unnamed_addr #2 personality i32 (...)* @__CxxFrameHandler3 {
start:
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17hcfdb2b2b7772fec2E(void ()* %f)
  br label %bb1

bb1:                                              ; preds = %start
; invoke core::hint::black_box
  invoke void @_ZN4core4hint9black_box17he4759ebeb9a4deceE()
          to label %bb2 unwind label %funclet_bb3

bb3:                                              ; preds = %funclet_bb3
  br label %bb4

funclet_bb3:                                      ; preds = %bb1
  %cleanuppad = cleanuppad within none []
  br label %bb3

bb2:                                              ; preds = %bb1
  ret void

bb4:                                              ; preds = %bb3
  cleanupret from %cleanuppad unwind to caller
}

; std::rt::lang_start
; Function Attrs: uwtable
define hidden i64 @_ZN3std2rt10lang_start17h8acd9567097365c6E(void ()* %main, i64 %argc, i8** %argv) unnamed_addr #1 {
start:
  %_8 = alloca i64*, align 8
  %_4 = alloca i64, align 8
  %0 = bitcast i64** %_8 to void ()**
  store void ()* %main, void ()** %0, align 8
  %_5.0 = bitcast i64** %_8 to {}*
; call std::rt::lang_start_internal
  %1 = call i64 @_ZN3std2rt19lang_start_internal17hafda6cb4b2d28b93E({}* align 1 %_5.0, [3 x i64]* align 8 bitcast (<{ i8*, [16 x i8], i8*, i8*, i8* }>* @vtable.0 to [3 x i64]*), i64 %argc, i8** %argv)
  store i64 %1, i64* %_4, align 8
  br label %bb1

bb1:                                              ; preds = %start
  %v = load i64, i64* %_4, align 8
  ret i64 %v
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h87c5ae68f7928da5E"(i64** align 8 %_1) unnamed_addr #0 {
start:
  %0 = bitcast i64** %_1 to void ()**
  %_4 = load void ()*, void ()** %0, align 8, !nonnull !3, !noundef !3
; call std::sys_common::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hda34cccff0e9eda2E(void ()* %_4)
  br label %bb1

bb1:                                              ; preds = %start
; call <() as std::process::Termination>::report
  %_2 = call i32 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hce51499ee67d1f1bE"()
  br label %bb2

bb2:                                              ; preds = %bb1
; call std::process::ExitCode::to_i32
  %1 = call i32 @_ZN3std7process8ExitCode6to_i3217h93fe0efc60928f08E(i32 %_2)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret i32 %1
}

; std::sys::windows::process::ExitCode::as_i32
; Function Attrs: inlinehint uwtable
define internal i32 @_ZN3std3sys7windows7process8ExitCode6as_i3217h9d5aeb09dc7eda2fE(i32* align 4 %self) unnamed_addr #0 {
start:
  %_2 = load i32, i32* %self, align 4
  ret i32 %_2
}

; std::process::ExitCode::to_i32
; Function Attrs: inlinehint uwtable
define internal i32 @_ZN3std7process8ExitCode6to_i3217h93fe0efc60928f08E(i32 %0) unnamed_addr #0 {
start:
  %self = alloca i32, align 4
  store i32 %0, i32* %self, align 4
; call std::sys::windows::process::ExitCode::as_i32
  %1 = call i32 @_ZN3std3sys7windows7process8ExitCode6as_i3217h9d5aeb09dc7eda2fE(i32* align 4 %self)
  br label %bb1

bb1:                                              ; preds = %start
  ret i32 %1
}

; <&T as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal zeroext i1 @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hae5318a5530776d7E"({ i64, i64 }** align 8 %self, %"core::fmt::Formatter"* align 8 %f) unnamed_addr #1 {
start:
  %_4 = load { i64, i64 }*, { i64, i64 }** %self, align 8, !nonnull !3, !align !4, !noundef !3
; call <core::option::Option<T> as core::fmt::Debug>::fmt
  %0 = call zeroext i1 @"_ZN66_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h82beae0e53d79ef7E"({ i64, i64 }* align 8 %_4, %"core::fmt::Formatter"* align 8 %f)
  br label %bb1

bb1:                                              ; preds = %start
  ret i1 %0
}

; <&T as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal zeroext i1 @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hb270a7a7e9077700E"(i64** align 8 %self, %"core::fmt::Formatter"* align 8 %f) unnamed_addr #1 {
start:
  %_4 = load i64*, i64** %self, align 8, !nonnull !3, !align !4, !noundef !3
; call core::fmt::num::<impl core::fmt::Debug for usize>::fmt
  %0 = call zeroext i1 @"_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h030e037742f1c7b7E"(i64* align 8 %_4, %"core::fmt::Formatter"* align 8 %f)
  br label %bb1

bb1:                                              ; preds = %start
  ret i1 %0
}

; core::cmp::impls::<impl core::cmp::PartialEq for usize>::eq
; Function Attrs: inlinehint uwtable
define internal zeroext i1 @"_ZN4core3cmp5impls56_$LT$impl$u20$core..cmp..PartialEq$u20$for$u20$usize$GT$2eq17h4bbeac1807bdf9dfE"(i64* align 8 %self, i64* align 8 %other) unnamed_addr #0 {
start:
  %_3 = load i64, i64* %self, align 8
  %_4 = load i64, i64* %other, align 8
  %0 = icmp eq i64 %_3, %_4
  ret i1 %0
}

; core::fmt::num::<impl core::fmt::Debug for usize>::fmt
; Function Attrs: inlinehint uwtable
define internal zeroext i1 @"_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h030e037742f1c7b7E"(i64* align 8 %self, %"core::fmt::Formatter"* align 8 %f) unnamed_addr #0 {
start:
  %0 = alloca i8, align 1
; call core::fmt::Formatter::debug_lower_hex
  %_3 = call zeroext i1 @_ZN4core3fmt9Formatter15debug_lower_hex17h705fc45d79f414f0E(%"core::fmt::Formatter"* align 8 %f)
  br label %bb1

bb1:                                              ; preds = %start
  br i1 %_3, label %bb2, label %bb4

bb4:                                              ; preds = %bb1
; call core::fmt::Formatter::debug_upper_hex
  %_7 = call zeroext i1 @_ZN4core3fmt9Formatter15debug_upper_hex17h8b753905378ff302E(%"core::fmt::Formatter"* align 8 %f)
  br label %bb5

bb2:                                              ; preds = %bb1
; call core::fmt::num::<impl core::fmt::LowerHex for usize>::fmt
  %1 = call zeroext i1 @"_ZN4core3fmt3num55_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$usize$GT$3fmt17h9eb247cc005c0d76E"(i64* align 8 %self, %"core::fmt::Formatter"* align 8 %f)
  %2 = zext i1 %1 to i8
  store i8 %2, i8* %0, align 1
  br label %bb3

bb3:                                              ; preds = %bb2
  br label %bb11

bb11:                                             ; preds = %bb10, %bb3
  %3 = load i8, i8* %0, align 1, !range !5, !noundef !3
  %4 = trunc i8 %3 to i1
  ret i1 %4

bb5:                                              ; preds = %bb4
  br i1 %_7, label %bb6, label %bb8

bb8:                                              ; preds = %bb5
; call core::fmt::num::imp::<impl core::fmt::Display for usize>::fmt
  %5 = call zeroext i1 @"_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17hc8b6f8f88d6483acE"(i64* align 8 %self, %"core::fmt::Formatter"* align 8 %f)
  %6 = zext i1 %5 to i8
  store i8 %6, i8* %0, align 1
  br label %bb9

bb6:                                              ; preds = %bb5
; call core::fmt::num::<impl core::fmt::UpperHex for usize>::fmt
  %7 = call zeroext i1 @"_ZN4core3fmt3num55_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$usize$GT$3fmt17h946be138df9b7392E"(i64* align 8 %self, %"core::fmt::Formatter"* align 8 %f)
  %8 = zext i1 %7 to i8
  store i8 %8, i8* %0, align 1
  br label %bb7

bb7:                                              ; preds = %bb6
  br label %bb10

bb10:                                             ; preds = %bb9, %bb7
  br label %bb11

bb9:                                              ; preds = %bb8
  br label %bb10
}

; core::num::<impl usize>::checked_mul
; Function Attrs: inlinehint uwtable
define internal { i64, i64 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_mul17hd8a29a9a7afade75E"(i64 %self, i64 %rhs) unnamed_addr #0 {
start:
  %0 = alloca { i64, i8 }, align 8
  %1 = alloca { i64, i8 }, align 8
  %2 = alloca i8, align 1
  %3 = alloca { i64, i64 }, align 8
  %4 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %self, i64 %rhs)
  %5 = extractvalue { i64, i1 } %4, 0
  %6 = extractvalue { i64, i1 } %4, 1
  %7 = zext i1 %6 to i8
  %8 = bitcast { i64, i8 }* %0 to i64*
  store i64 %5, i64* %8, align 8
  %9 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %0, i32 0, i32 1
  store i8 %7, i8* %9, align 8
  %10 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %0, i32 0, i32 0
  %_5.0.i = load i64, i64* %10, align 8
  %11 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %0, i32 0, i32 1
  %12 = load i8, i8* %11, align 8, !range !5, !noundef !3
  %_5.1.i = trunc i8 %12 to i1
  %13 = bitcast { i64, i8 }* %1 to i64*
  store i64 %_5.0.i, i64* %13, align 8
  %14 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %1, i32 0, i32 1
  %15 = zext i1 %_5.1.i to i8
  store i8 %15, i8* %14, align 8
  %16 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %1, i32 0, i32 0
  %17 = load i64, i64* %16, align 8
  %18 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %1, i32 0, i32 1
  %19 = load i8, i8* %18, align 8, !range !5, !noundef !3
  %20 = trunc i8 %19 to i1
  %21 = zext i1 %20 to i8
  %22 = insertvalue { i64, i8 } undef, i64 %17, 0
  %23 = insertvalue { i64, i8 } %22, i8 %21, 1
  %_5.0 = extractvalue { i64, i8 } %23, 0
  %24 = extractvalue { i64, i8 } %23, 1
  %_5.1 = trunc i8 %24 to i1
  br label %bb1

bb1:                                              ; preds = %start
  %25 = call i1 @llvm.expect.i1(i1 %_5.1, i1 false)
  %26 = zext i1 %25 to i8
  store i8 %26, i8* %2, align 1
  %27 = load i8, i8* %2, align 1, !range !5, !noundef !3
  %_8 = trunc i8 %27 to i1
  br label %bb2

bb2:                                              ; preds = %bb1
  br i1 %_8, label %bb3, label %bb4

bb4:                                              ; preds = %bb2
  %28 = bitcast { i64, i64 }* %3 to %"core::option::Option<usize>::Some"*
  %29 = getelementptr inbounds %"core::option::Option<usize>::Some", %"core::option::Option<usize>::Some"* %28, i32 0, i32 1
  store i64 %_5.0, i64* %29, align 8
  %30 = bitcast { i64, i64 }* %3 to i64*
  store i64 1, i64* %30, align 8
  br label %bb5

bb3:                                              ; preds = %bb2
  %31 = bitcast { i64, i64 }* %3 to i64*
  store i64 0, i64* %31, align 8
  br label %bb5

bb5:                                              ; preds = %bb4, %bb3
  %32 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %3, i32 0, i32 0
  %33 = load i64, i64* %32, align 8, !range !2, !noundef !3
  %34 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %3, i32 0, i32 1
  %35 = load i64, i64* %34, align 8
  %36 = insertvalue { i64, i64 } undef, i64 %33, 0
  %37 = insertvalue { i64, i64 } %36, i64 %35, 1
  ret { i64, i64 } %37
}

; core::num::nonzero::NonZeroUsize::new_unchecked
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core3num7nonzero12NonZeroUsize13new_unchecked17hf0411f9840ebd1edE(i64 %n) unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  store i64 %n, i64* %0, align 8
  %1 = load i64, i64* %0, align 8, !range !6, !noundef !3
  ret i64 %1
}

; core::num::nonzero::NonZeroUsize::get
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core3num7nonzero12NonZeroUsize3get17h9afe58b26c3b3ebfE(i64 %self) unnamed_addr #0 {
start:
  ret i64 %self
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hc4726b8faa2f60f5E"(i64** %_1) unnamed_addr #0 {
start:
  %_2 = alloca {}, align 1
  %0 = load i64*, i64** %_1, align 8, !nonnull !3, !noundef !3
; call core::ops::function::FnOnce::call_once
  %1 = call i32 @_ZN4core3ops8function6FnOnce9call_once17h468585c26c7fa3a6E(i64* %0)
  br label %bb1

bb1:                                              ; preds = %start
  ret i32 %1
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17h468585c26c7fa3a6E(i64* %0) unnamed_addr #0 personality i32 (...)* @__CxxFrameHandler3 {
start:
  %_2 = alloca {}, align 1
  %_1 = alloca i64*, align 8
  store i64* %0, i64** %_1, align 8
; invoke std::rt::lang_start::{{closure}}
  %1 = invoke i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h87c5ae68f7928da5E"(i64** align 8 %_1)
          to label %bb1 unwind label %funclet_bb3

bb3:                                              ; preds = %funclet_bb3
  br label %bb4

funclet_bb3:                                      ; preds = %start
  %cleanuppad = cleanuppad within none []
  br label %bb3

bb1:                                              ; preds = %start
  br label %bb2

bb4:                                              ; preds = %bb3
  cleanupret from %cleanuppad unwind to caller

bb2:                                              ; preds = %bb1
  ret i32 %1
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17hcfdb2b2b7772fec2E(void ()* %_1) unnamed_addr #0 {
start:
  %_2 = alloca {}, align 1
  call void %_1()
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::drop_in_place<<alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop::DropGuard<std::ffi::os_str::OsString,alloc::alloc::Global>>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr180drop_in_place$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$std..ffi..os_str..OsString$C$alloc..alloc..Global$GT$$GT$17hed087f7f6a8908f6E"(i64** %_1) unnamed_addr #1 {
start:
; call <<alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop::DropGuard<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN157_$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hff8f95d3da5d9bebE"(i64** align 8 %_1)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::slice_from_raw_parts_mut
; Function Attrs: inlinehint uwtable
define internal { [0 x i8]*, i64 } @_ZN4core3ptr24slice_from_raw_parts_mut17h589593876c82e883E(i8* %data, i64 %len) unnamed_addr #0 {
start:
  %0 = bitcast i8* %data to {}*
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::metadata::from_raw_parts_mut
  %1 = call { [0 x i8]*, i64 } @_ZN4core3ptr8metadata18from_raw_parts_mut17h54650e67c1e66d16E({}* %0, i64 %len)
  %2 = extractvalue { [0 x i8]*, i64 } %1, 0
  %3 = extractvalue { [0 x i8]*, i64 } %1, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  %4 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %2, 0
  %5 = insertvalue { [0 x i8]*, i64 } %4, i64 %3, 1
  ret { [0 x i8]*, i64 } %5
}

; core::ptr::slice_from_raw_parts_mut
; Function Attrs: inlinehint uwtable
define internal { [0 x %"std::ffi::os_str::OsString"]*, i64 } @_ZN4core3ptr24slice_from_raw_parts_mut17h8ebef298811a14e5E(%"std::ffi::os_str::OsString"* %data, i64 %len) unnamed_addr #0 {
start:
  %0 = bitcast %"std::ffi::os_str::OsString"* %data to {}*
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::metadata::from_raw_parts_mut
  %1 = call { [0 x %"std::ffi::os_str::OsString"]*, i64 } @_ZN4core3ptr8metadata18from_raw_parts_mut17h89113da6e0a491d0E({}* %0, i64 %len)
  %2 = extractvalue { [0 x %"std::ffi::os_str::OsString"]*, i64 } %1, 0
  %3 = extractvalue { [0 x %"std::ffi::os_str::OsString"]*, i64 } %1, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  %4 = insertvalue { [0 x %"std::ffi::os_str::OsString"]*, i64 } undef, [0 x %"std::ffi::os_str::OsString"]* %2, 0
  %5 = insertvalue { [0 x %"std::ffi::os_str::OsString"]*, i64 } %4, i64 %3, 1
  ret { [0 x %"std::ffi::os_str::OsString"]*, i64 } %5
}

; core::ptr::drop_in_place<&usize>
; Function Attrs: inlinehint uwtable
define internal void @"_ZN4core3ptr30drop_in_place$LT$$RF$usize$GT$17hdaf709f8f2433fa4E"(i64** %_1) unnamed_addr #0 {
start:
  ret void
}

; core::ptr::drop_in_place<std::env::Args>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17h8bb3a0fa3e904967E"(%"std::env::Args"* %_1) unnamed_addr #1 {
start:
  %0 = bitcast %"std::env::Args"* %_1 to %"std::env::ArgsOs"*
; call core::ptr::drop_in_place<std::env::ArgsOs>
  call void @"_ZN4core3ptr37drop_in_place$LT$std..env..ArgsOs$GT$17h58b45318ab724083E"(%"std::env::ArgsOs"* %0)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::drop_in_place<std::env::ArgsOs>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr37drop_in_place$LT$std..env..ArgsOs$GT$17h58b45318ab724083E"(%"std::env::ArgsOs"* %_1) unnamed_addr #1 {
start:
  %0 = bitcast %"std::env::ArgsOs"* %_1 to %"std::sys::windows::args::Args"*
; call core::ptr::drop_in_place<std::sys::windows::args::Args>
  call void @"_ZN4core3ptr50drop_in_place$LT$std..sys..windows..args..Args$GT$17hb82581b439629b5bE"(%"std::sys::windows::args::Args"* %0)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::drop_in_place<alloc::vec::Vec<u8>>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h34819694c0b3ae4bE"(%"alloc::vec::Vec<u8>"* %_1) unnamed_addr #1 personality i32 (...)* @__CxxFrameHandler3 {
start:
; invoke <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
  invoke void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h05ab1c2b38a1fc9dE"(%"alloc::vec::Vec<u8>"* align 8 %_1)
          to label %bb4 unwind label %funclet_bb3

bb3:                                              ; preds = %funclet_bb3
  %0 = bitcast %"alloc::vec::Vec<u8>"* %_1 to { i8*, i64 }*
; call core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
  call void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h674aef848a82a113E"({ i8*, i64 }* %0) #13 [ "funclet"(token %cleanuppad) ]
  br label %bb1

funclet_bb3:                                      ; preds = %start
  %cleanuppad = cleanuppad within none []
  br label %bb3

bb4:                                              ; preds = %start
  %1 = bitcast %"alloc::vec::Vec<u8>"* %_1 to { i8*, i64 }*
; call core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
  call void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h674aef848a82a113E"({ i8*, i64 }* %1)
  br label %bb2

bb1:                                              ; preds = %bb3
  cleanupret from %cleanuppad unwind to caller

bb2:                                              ; preds = %bb4
  ret void
}

; core::ptr::drop_in_place<std::ffi::os_str::OsString>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hf6c23f463ebcea10E"(%"std::ffi::os_str::OsString"* %_1) unnamed_addr #1 {
start:
  %0 = bitcast %"std::ffi::os_str::OsString"* %_1 to %"std::sys::windows::os_str::Buf"*
; call core::ptr::drop_in_place<std::sys::windows::os_str::Buf>
  call void @"_ZN4core3ptr51drop_in_place$LT$std..sys..windows..os_str..Buf$GT$17hb5dddd6372780ae9E"(%"std::sys::windows::os_str::Buf"* %0)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::read
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3ptr4read17h24ab8ebef5d9cb58E(%"alloc::alloc::Global"* %src) unnamed_addr #0 {
start:
  %0 = alloca %"core::mem::maybe_uninit::MaybeUninit<alloc::alloc::Global>", align 1
  %tmp = alloca %"core::mem::maybe_uninit::MaybeUninit<alloc::alloc::Global>", align 1
  %1 = bitcast %"core::mem::maybe_uninit::MaybeUninit<alloc::alloc::Global>"* %0 to {}*
  br label %bb1

bb1:                                              ; preds = %start
  %2 = bitcast %"core::mem::maybe_uninit::MaybeUninit<alloc::alloc::Global>"* %tmp to %"alloc::alloc::Global"*
  br label %bb2

bb2:                                              ; preds = %bb1
  %3 = bitcast %"alloc::alloc::Global"* %2 to i8*
  %4 = bitcast %"alloc::alloc::Global"* %src to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %3, i8* align 1 %4, i64 0, i1 false)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret void
}

; core::ptr::drop_in_place<std::sys::windows::args::Args>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr50drop_in_place$LT$std..sys..windows..args..Args$GT$17hb82581b439629b5bE"(%"std::sys::windows::args::Args"* %_1) unnamed_addr #1 {
start:
  %0 = bitcast %"std::sys::windows::args::Args"* %_1 to %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"*
; call core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>>
  call void @"_ZN4core3ptr86drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$std..ffi..os_str..OsString$GT$$GT$17h5123e688c8a2a11cE"(%"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"* %0)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::drop_in_place<std::sys::windows::os_str::Buf>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr51drop_in_place$LT$std..sys..windows..os_str..Buf$GT$17hb5dddd6372780ae9E"(%"std::sys::windows::os_str::Buf"* %_1) unnamed_addr #1 {
start:
  %0 = bitcast %"std::sys::windows::os_str::Buf"* %_1 to %"std::sys_common::wtf8::Wtf8Buf"*
; call core::ptr::drop_in_place<std::sys_common::wtf8::Wtf8Buf>
  call void @"_ZN4core3ptr51drop_in_place$LT$std..sys_common..wtf8..Wtf8Buf$GT$17h624a2b6240aac6e8E"(%"std::sys_common::wtf8::Wtf8Buf"* %0)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::drop_in_place<std::sys_common::wtf8::Wtf8Buf>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr51drop_in_place$LT$std..sys_common..wtf8..Wtf8Buf$GT$17h624a2b6240aac6e8E"(%"std::sys_common::wtf8::Wtf8Buf"* %_1) unnamed_addr #1 {
start:
  %0 = bitcast %"std::sys_common::wtf8::Wtf8Buf"* %_1 to %"alloc::vec::Vec<u8>"*
; call core::ptr::drop_in_place<alloc::vec::Vec<u8>>
  call void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h34819694c0b3ae4bE"(%"alloc::vec::Vec<u8>"* %0)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h674aef848a82a113E"({ i8*, i64 }* %_1) unnamed_addr #1 {
start:
; call <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8dfb863328ab7775E"({ i8*, i64 }* align 8 %_1)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::drop_in_place<[std::ffi::os_str::OsString]>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr57drop_in_place$LT$$u5b$std..ffi..os_str..OsString$u5d$$GT$17h1515d6f4ee4cd1d6E"([0 x %"std::ffi::os_str::OsString"]* %_1.0, i64 %_1.1) unnamed_addr #1 personality i32 (...)* @__CxxFrameHandler3 {
start:
  %_9 = alloca %"std::ffi::os_str::OsString"*, align 8
  %_4 = alloca i64, align 8
  br i1 false, label %bb7, label %bb12

bb7:                                              ; preds = %start
  store i64 0, i64* %_4, align 8
  br label %bb6

bb12:                                             ; preds = %start
  %0 = bitcast [0 x %"std::ffi::os_str::OsString"]* %_1.0 to %"std::ffi::os_str::OsString"*
  store %"std::ffi::os_str::OsString"* %0, %"std::ffi::os_str::OsString"** %_9, align 8
  %1 = load %"std::ffi::os_str::OsString"*, %"std::ffi::os_str::OsString"** %_9, align 8
  %_10 = getelementptr inbounds %"std::ffi::os_str::OsString", %"std::ffi::os_str::OsString"* %1, i64 %_1.1
  br label %bb11

bb11:                                             ; preds = %bb10, %bb12
  %2 = load %"std::ffi::os_str::OsString"*, %"std::ffi::os_str::OsString"** %_9, align 8
  %_14 = icmp eq %"std::ffi::os_str::OsString"* %2, %_10
  br i1 %_14, label %bb1, label %bb10

bb10:                                             ; preds = %bb11
  %_13 = load %"std::ffi::os_str::OsString"*, %"std::ffi::os_str::OsString"** %_9, align 8
  %3 = load %"std::ffi::os_str::OsString"*, %"std::ffi::os_str::OsString"** %_9, align 8
  %4 = getelementptr inbounds %"std::ffi::os_str::OsString", %"std::ffi::os_str::OsString"* %3, i64 1
  store %"std::ffi::os_str::OsString"* %4, %"std::ffi::os_str::OsString"** %_9, align 8
; invoke core::ptr::drop_in_place<std::ffi::os_str::OsString>
  invoke void @"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hf6c23f463ebcea10E"(%"std::ffi::os_str::OsString"* %_13)
          to label %bb11 unwind label %funclet_bb9

bb1:                                              ; preds = %bb6, %bb11
  ret void

bb9:                                              ; preds = %bb8, %funclet_bb9
  %5 = load %"std::ffi::os_str::OsString"*, %"std::ffi::os_str::OsString"** %_9, align 8
  %_12 = icmp eq %"std::ffi::os_str::OsString"* %5, %_10
  br i1 %_12, label %bb9_cleanup_trampoline_bb2, label %bb8

funclet_bb9:                                      ; preds = %bb10
  %cleanuppad = cleanuppad within none []
  br label %bb9

bb8:                                              ; preds = %bb9
  %_11 = load %"std::ffi::os_str::OsString"*, %"std::ffi::os_str::OsString"** %_9, align 8
  %6 = load %"std::ffi::os_str::OsString"*, %"std::ffi::os_str::OsString"** %_9, align 8
  %7 = getelementptr inbounds %"std::ffi::os_str::OsString", %"std::ffi::os_str::OsString"* %6, i64 1
  store %"std::ffi::os_str::OsString"* %7, %"std::ffi::os_str::OsString"** %_9, align 8
; call core::ptr::drop_in_place<std::ffi::os_str::OsString>
  call void @"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hf6c23f463ebcea10E"(%"std::ffi::os_str::OsString"* %_11) #13 [ "funclet"(token %cleanuppad) ]
  br label %bb9

bb2:                                              ; preds = %funclet_bb2
  cleanupret from %cleanuppad1 unwind to caller

funclet_bb2:                                      ; preds = %bb4_cleanup_trampoline_bb2, %bb9_cleanup_trampoline_bb2
  %cleanuppad1 = cleanuppad within none []
  br label %bb2

bb9_cleanup_trampoline_bb2:                       ; preds = %bb9
  cleanupret from %cleanuppad unwind label %funclet_bb2

bb6:                                              ; preds = %bb5, %bb7
  %8 = load i64, i64* %_4, align 8
  %_8 = icmp eq i64 %8, %_1.1
  br i1 %_8, label %bb1, label %bb5

bb5:                                              ; preds = %bb6
  %9 = load i64, i64* %_4, align 8
  %_7 = getelementptr inbounds [0 x %"std::ffi::os_str::OsString"], [0 x %"std::ffi::os_str::OsString"]* %_1.0, i64 0, i64 %9
  %10 = load i64, i64* %_4, align 8
  %11 = add i64 %10, 1
  store i64 %11, i64* %_4, align 8
; invoke core::ptr::drop_in_place<std::ffi::os_str::OsString>
  invoke void @"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hf6c23f463ebcea10E"(%"std::ffi::os_str::OsString"* %_7)
          to label %bb6 unwind label %funclet_bb4

bb4:                                              ; preds = %bb3, %funclet_bb4
  %12 = load i64, i64* %_4, align 8
  %_6 = icmp eq i64 %12, %_1.1
  br i1 %_6, label %bb4_cleanup_trampoline_bb2, label %bb3

funclet_bb4:                                      ; preds = %bb5
  %cleanuppad2 = cleanuppad within none []
  br label %bb4

bb3:                                              ; preds = %bb4
  %13 = load i64, i64* %_4, align 8
  %_5 = getelementptr inbounds [0 x %"std::ffi::os_str::OsString"], [0 x %"std::ffi::os_str::OsString"]* %_1.0, i64 0, i64 %13
  %14 = load i64, i64* %_4, align 8
  %15 = add i64 %14, 1
  store i64 %15, i64* %_4, align 8
; call core::ptr::drop_in_place<std::ffi::os_str::OsString>
  call void @"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hf6c23f463ebcea10E"(%"std::ffi::os_str::OsString"* %_5) #13 [ "funclet"(token %cleanuppad2) ]
  br label %bb4

bb4_cleanup_trampoline_bb2:                       ; preds = %bb4
  cleanupret from %cleanuppad2 unwind label %funclet_bb2
}

; core::ptr::drop_in_place<&core::option::Option<usize>>
; Function Attrs: inlinehint uwtable
define internal void @"_ZN4core3ptr58drop_in_place$LT$$RF$core..option..Option$LT$usize$GT$$GT$17h19ecb1043aee27f1E"({ i64, i64 }** %_1) unnamed_addr #0 {
start:
  ret void
}

; core::ptr::unique::Unique<T>::new_unchecked
; Function Attrs: inlinehint uwtable
define internal i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17ha4b253678320e5adE"(i8* %ptr) unnamed_addr #0 {
start:
  %0 = alloca i8*, align 8
  store i8* %ptr, i8** %0, align 8
  %1 = load i8*, i8** %0, align 8, !nonnull !3, !noundef !3
  ret i8* %1
}

; core::ptr::unique::Unique<T>::new_unchecked
; Function Attrs: inlinehint uwtable
define internal i64* @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17ha674211a1d897b29E"(%"std::ffi::os_str::OsString"* %ptr) unnamed_addr #0 {
start:
  %0 = alloca i64*, align 8
  %1 = bitcast i64** %0 to %"std::ffi::os_str::OsString"**
  store %"std::ffi::os_str::OsString"* %ptr, %"std::ffi::os_str::OsString"** %1, align 8
  %2 = load i64*, i64** %0, align 8, !nonnull !3, !noundef !3
  ret i64* %2
}

; core::ptr::unique::Unique<T>::cast
; Function Attrs: inlinehint uwtable
define internal i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17h56e10057c8f7d5c7E"(i64* %self) unnamed_addr #0 {
start:
; call core::ptr::unique::Unique<T>::as_ptr
  %_3 = call %"std::ffi::os_str::OsString"* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h8402cff6598ae8feE"(i64* %self)
  br label %bb1

bb1:                                              ; preds = %start
  %_2 = bitcast %"std::ffi::os_str::OsString"* %_3 to i8*
; call core::ptr::unique::Unique<T>::new_unchecked
  %0 = call i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17ha4b253678320e5adE"(i8* %_2)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i8* %0
}

; core::ptr::unique::Unique<T>::cast
; Function Attrs: inlinehint uwtable
define internal i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17hec46cb201340efc6E"(i8* %self) unnamed_addr #0 {
start:
; call core::ptr::unique::Unique<T>::as_ptr
  %_3 = call i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h9206e7cf0f3ae9e0E"(i8* %self)
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::unique::Unique<T>::new_unchecked
  %0 = call i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17ha4b253678320e5adE"(i8* %_3)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i8* %0
}

; core::ptr::unique::Unique<T>::as_ptr
; Function Attrs: inlinehint uwtable
define internal %"std::ffi::os_str::OsString"* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h8402cff6598ae8feE"(i64* %self) unnamed_addr #0 {
start:
  %_2 = bitcast i64* %self to %"std::ffi::os_str::OsString"*
  ret %"std::ffi::os_str::OsString"* %_2
}

; core::ptr::unique::Unique<T>::as_ptr
; Function Attrs: inlinehint uwtable
define internal i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h9206e7cf0f3ae9e0E"(i8* %self) unnamed_addr #0 {
start:
  ret i8* %self
}

; core::ptr::drop_in_place<alloc::raw_vec::RawVec<std::ffi::os_str::OsString>>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr77drop_in_place$LT$alloc..raw_vec..RawVec$LT$std..ffi..os_str..OsString$GT$$GT$17h790dc7b886675322E"({ i64*, i64 }* %_1) unnamed_addr #1 {
start:
; call <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hae891832f2f092c4E"({ i64*, i64 }* align 8 %_1)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::mut_ptr::<impl *mut T>::guaranteed_eq
; Function Attrs: inlinehint uwtable
define internal zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$13guaranteed_eq17h316314d9caf0a0aeE"(i8* %self, i8* %other) unnamed_addr #0 {
start:
  %0 = alloca i8, align 1
  %1 = icmp eq i8* %self, %other
  %2 = zext i1 %1 to i8
  store i8 %2, i8* %0, align 1
  %3 = load i8, i8* %0, align 1, !range !5, !noundef !3
  %4 = trunc i8 %3 to i1
  br label %bb1

bb1:                                              ; preds = %start
  ret i1 %4
}

; core::ptr::mut_ptr::<impl *mut T>::is_null
; Function Attrs: inlinehint uwtable
define internal zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17hb7643854fcfece7fE"(i8* %self) unnamed_addr #0 {
start:
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::mut_ptr::<impl *mut T>::guaranteed_eq
  %0 = call zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$13guaranteed_eq17h316314d9caf0a0aeE"(i8* %self, i8* null)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i1 %0
}

; core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
; Function Attrs: inlinehint uwtable
define internal void @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h474366d8b931e683E"(i64** %_1) unnamed_addr #0 {
start:
  ret void
}

; core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr86drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$std..ffi..os_str..OsString$GT$$GT$17h5123e688c8a2a11cE"(%"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"* %_1) unnamed_addr #1 {
start:
; call <alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hb92d2045b8381d81E"(%"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"* align 8 %_1)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::metadata::from_raw_parts_mut
; Function Attrs: inlinehint uwtable
define internal { [0 x i8]*, i64 } @_ZN4core3ptr8metadata18from_raw_parts_mut17h54650e67c1e66d16E({}* %data_address, i64 %metadata) unnamed_addr #0 {
start:
  %_4 = alloca { i8*, i64 }, align 8
  %_3 = alloca %"core::ptr::metadata::PtrRepr<[u8]>", align 8
  %0 = bitcast { i8*, i64 }* %_4 to {}**
  store {}* %data_address, {}** %0, align 8
  %1 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_4, i32 0, i32 1
  store i64 %metadata, i64* %1, align 8
  %2 = bitcast %"core::ptr::metadata::PtrRepr<[u8]>"* %_3 to { i8*, i64 }*
  %3 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_4, i32 0, i32 0
  %4 = load i8*, i8** %3, align 8
  %5 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_4, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
  %7 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 0
  store i8* %4, i8** %7, align 8
  %8 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 1
  store i64 %6, i64* %8, align 8
  %9 = bitcast %"core::ptr::metadata::PtrRepr<[u8]>"* %_3 to { [0 x i8]*, i64 }*
  %10 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %9, i32 0, i32 0
  %11 = load [0 x i8]*, [0 x i8]** %10, align 8
  %12 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %9, i32 0, i32 1
  %13 = load i64, i64* %12, align 8
  %14 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %11, 0
  %15 = insertvalue { [0 x i8]*, i64 } %14, i64 %13, 1
  ret { [0 x i8]*, i64 } %15
}

; core::ptr::metadata::from_raw_parts_mut
; Function Attrs: inlinehint uwtable
define internal { [0 x %"std::ffi::os_str::OsString"]*, i64 } @_ZN4core3ptr8metadata18from_raw_parts_mut17h89113da6e0a491d0E({}* %data_address, i64 %metadata) unnamed_addr #0 {
start:
  %_4 = alloca { i8*, i64 }, align 8
  %_3 = alloca %"core::ptr::metadata::PtrRepr<[std::ffi::os_str::OsString]>", align 8
  %0 = bitcast { i8*, i64 }* %_4 to {}**
  store {}* %data_address, {}** %0, align 8
  %1 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_4, i32 0, i32 1
  store i64 %metadata, i64* %1, align 8
  %2 = bitcast %"core::ptr::metadata::PtrRepr<[std::ffi::os_str::OsString]>"* %_3 to { i8*, i64 }*
  %3 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_4, i32 0, i32 0
  %4 = load i8*, i8** %3, align 8
  %5 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_4, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
  %7 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 0
  store i8* %4, i8** %7, align 8
  %8 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 1
  store i64 %6, i64* %8, align 8
  %9 = bitcast %"core::ptr::metadata::PtrRepr<[std::ffi::os_str::OsString]>"* %_3 to { [0 x %"std::ffi::os_str::OsString"]*, i64 }*
  %10 = getelementptr inbounds { [0 x %"std::ffi::os_str::OsString"]*, i64 }, { [0 x %"std::ffi::os_str::OsString"]*, i64 }* %9, i32 0, i32 0
  %11 = load [0 x %"std::ffi::os_str::OsString"]*, [0 x %"std::ffi::os_str::OsString"]** %10, align 8
  %12 = getelementptr inbounds { [0 x %"std::ffi::os_str::OsString"]*, i64 }, { [0 x %"std::ffi::os_str::OsString"]*, i64 }* %9, i32 0, i32 1
  %13 = load i64, i64* %12, align 8
  %14 = insertvalue { [0 x %"std::ffi::os_str::OsString"]*, i64 } undef, [0 x %"std::ffi::os_str::OsString"]* %11, 0
  %15 = insertvalue { [0 x %"std::ffi::os_str::OsString"]*, i64 } %14, i64 %13, 1
  ret { [0 x %"std::ffi::os_str::OsString"]*, i64 } %15
}

; core::ptr::non_null::NonNull<T>::new_unchecked
; Function Attrs: inlinehint uwtable
define internal i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17heb024f450d5f02e1E"(i8* %ptr) unnamed_addr #0 {
start:
  %0 = alloca i8*, align 8
  store i8* %ptr, i8** %0, align 8
  %1 = load i8*, i8** %0, align 8, !nonnull !3, !noundef !3
  ret i8* %1
}

; core::ptr::non_null::NonNull<T>::as_ptr
; Function Attrs: inlinehint uwtable
define internal i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h5e59b14b9cab6074E"(i8* %self) unnamed_addr #0 {
start:
  ret i8* %self
}

; core::ptr::non_null::NonNull<T>::as_ptr
; Function Attrs: inlinehint uwtable
define internal %"std::ffi::os_str::OsString"* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h7ef62d35e080de5eE"(i64* %self) unnamed_addr #0 {
start:
  %_2 = bitcast i64* %self to %"std::ffi::os_str::OsString"*
  ret %"std::ffi::os_str::OsString"* %_2
}

; core::ptr::const_ptr::<impl *const T>::offset_from
; Function Attrs: inlinehint uwtable
define internal i64 @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$11offset_from17h8b15a4e317183e25E"(%"std::ffi::os_str::OsString"* %self, %"std::ffi::os_str::OsString"* %origin) unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  %_5 = alloca i8, align 1
  br label %bb1

bb1:                                              ; preds = %start
  %_6 = icmp ult i64 0, 24
  br i1 %_6, label %bb3, label %bb2

bb2:                                              ; preds = %bb1
  store i8 0, i8* %_5, align 1
  br label %bb4

bb3:                                              ; preds = %bb1
  %_8 = icmp ule i64 24, 9223372036854775807
  %1 = zext i1 %_8 to i8
  store i8 %1, i8* %_5, align 1
  br label %bb4

bb4:                                              ; preds = %bb2, %bb3
  %2 = load i8, i8* %_5, align 1, !range !5, !noundef !3
  %3 = trunc i8 %2 to i1
  %_4 = xor i1 %3, true
  br i1 %_4, label %bb5, label %bb6

bb6:                                              ; preds = %bb4
  %4 = ptrtoint %"std::ffi::os_str::OsString"* %self to i64
  %5 = ptrtoint %"std::ffi::os_str::OsString"* %origin to i64
  %6 = sub i64 %4, %5
  %7 = sdiv exact i64 %6, 24
  store i64 %7, i64* %0, align 8
  %8 = load i64, i64* %0, align 8
  br label %bb7

bb5:                                              ; preds = %bb4
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h529b7d74f0f4c3e5E([0 x i8]* align 1 bitcast (<{ [73 x i8] }>* @alloc16 to [0 x i8]*), i64 73, %"core::panic::location::Location"* align 8 bitcast (<{ i8*, [16 x i8] }>* @alloc18 to %"core::panic::location::Location"*)) #14
  unreachable

bb7:                                              ; preds = %bb6
  ret i64 %8
}

; core::hint::unreachable_unchecked
; Function Attrs: inlinehint noreturn uwtable
define internal void @_ZN4core4hint21unreachable_unchecked17hb433023e7dc94f82E() unnamed_addr #3 {
start:
  unreachable
}

; core::hint::black_box
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core4hint9black_box17he4759ebeb9a4deceE() unnamed_addr #0 {
start:
  call void asm sideeffect "", "r,~{memory}"({}* undef), !srcloc !7
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::iter::traits::exact_size::ExactSizeIterator::len
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core4iter6traits10exact_size17ExactSizeIterator3len17ha1a88d1d8ab39c74E(%"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"* align 8 %self) unnamed_addr #0 {
start:
  %_22 = alloca %"core::option::Option<core::fmt::Arguments>", align 8
  %kind = alloca i8, align 1
  %_8 = alloca { i64, i64 }, align 8
  %_5 = alloca { i64*, i64* }, align 8
  %_3 = alloca { i64, { i64, i64 } }, align 8
  %upper = alloca { i64, i64 }, align 8
; call <alloc::vec::into_iter::IntoIter<T,A> as core::iter::traits::iterator::Iterator>::size_hint
  call void @"_ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9size_hint17h8c861d489f5ed3e2E"({ i64, { i64, i64 } }* sret({ i64, { i64, i64 } }) %_3, %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"* align 8 %self)
  br label %bb1

bb1:                                              ; preds = %start
  %0 = bitcast { i64, { i64, i64 } }* %_3 to i64*
  %lower = load i64, i64* %0, align 8
  %1 = getelementptr inbounds { i64, { i64, i64 } }, { i64, { i64, i64 } }* %_3, i32 0, i32 1
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %1, i32 0, i32 0
  %3 = load i64, i64* %2, align 8, !range !2, !noundef !3
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %1, i32 0, i32 1
  %5 = load i64, i64* %4, align 8
  %6 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %upper, i32 0, i32 0
  store i64 %3, i64* %6, align 8
  %7 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %upper, i32 0, i32 1
  store i64 %5, i64* %7, align 8
  %8 = bitcast { i64, i64 }* %_8 to %"core::option::Option<usize>::Some"*
  %9 = getelementptr inbounds %"core::option::Option<usize>::Some", %"core::option::Option<usize>::Some"* %8, i32 0, i32 1
  store i64 %lower, i64* %9, align 8
  %10 = bitcast { i64, i64 }* %_8 to i64*
  store i64 1, i64* %10, align 8
  %11 = bitcast { i64*, i64* }* %_5 to { i64, i64 }**
  store { i64, i64 }* %upper, { i64, i64 }** %11, align 8
  %12 = getelementptr inbounds { i64*, i64* }, { i64*, i64* }* %_5, i32 0, i32 1
  %13 = bitcast i64** %12 to { i64, i64 }**
  store { i64, i64 }* %_8, { i64, i64 }** %13, align 8
  %14 = bitcast { i64*, i64* }* %_5 to { i64, i64 }**
  %left_val = load { i64, i64 }*, { i64, i64 }** %14, align 8, !nonnull !3, !align !4, !noundef !3
  %15 = getelementptr inbounds { i64*, i64* }, { i64*, i64* }* %_5, i32 0, i32 1
  %16 = bitcast i64** %15 to { i64, i64 }**
  %right_val = load { i64, i64 }*, { i64, i64 }** %16, align 8, !nonnull !3, !align !4, !noundef !3
; call <core::option::Option<T> as core::cmp::PartialEq>::eq
  %_13 = call zeroext i1 @"_ZN70_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..cmp..PartialEq$GT$2eq17h31979d036de26d1aE"({ i64, i64 }* align 8 %left_val, { i64, i64 }* align 8 %right_val)
  br label %bb2

bb2:                                              ; preds = %bb1
  %_12 = xor i1 %_13, true
  br i1 %_12, label %bb3, label %bb4

bb4:                                              ; preds = %bb2
  ret i64 %lower

bb3:                                              ; preds = %bb2
  store i8 0, i8* %kind, align 1
  %17 = bitcast %"core::option::Option<core::fmt::Arguments>"* %_22 to {}**
  store {}* null, {}** %17, align 8
; call core::panicking::assert_failed
  call void @_ZN4core9panicking13assert_failed17hc31224a192f3a61dE(i8 0, { i64, i64 }* align 8 %left_val, { i64, i64 }* align 8 %right_val, %"core::option::Option<core::fmt::Arguments>"* %_22, %"core::panic::location::Location"* align 8 bitcast (<{ i8*, [16 x i8] }>* @alloc20 to %"core::panic::location::Location"*)) #14
  unreachable
}

; core::alloc::layout::Layout::from_size_align_unchecked
; Function Attrs: inlinehint uwtable
define internal { i64, i64 } @_ZN4core5alloc6layout6Layout25from_size_align_unchecked17hb8d1ebdf7c1ec12cE(i64 %size, i64 %align) unnamed_addr #0 {
start:
  %0 = alloca { i64, i64 }, align 8
; call core::num::nonzero::NonZeroUsize::new_unchecked
  %_4 = call i64 @_ZN4core3num7nonzero12NonZeroUsize13new_unchecked17hf0411f9840ebd1edE(i64 %align), !range !6
  br label %bb1

bb1:                                              ; preds = %start
  %1 = bitcast { i64, i64 }* %0 to i64*
  store i64 %size, i64* %1, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  store i64 %_4, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 0
  %4 = load i64, i64* %3, align 8
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  %6 = load i64, i64* %5, align 8, !range !6, !noundef !3
  %7 = insertvalue { i64, i64 } undef, i64 %4, 0
  %8 = insertvalue { i64, i64 } %7, i64 %6, 1
  ret { i64, i64 } %8
}

; core::alloc::layout::Layout::size
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core5alloc6layout6Layout4size17h557370479cae743eE({ i64, i64 }* align 8 %self) unnamed_addr #0 {
start:
  %0 = bitcast { i64, i64 }* %self to i64*
  %1 = load i64, i64* %0, align 8
  ret i64 %1
}

; core::alloc::layout::Layout::align
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core5alloc6layout6Layout5align17h4ca7a91dafbd7cc1E({ i64, i64 }* align 8 %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  %_2 = load i64, i64* %0, align 8, !range !6, !noundef !3
; call core::num::nonzero::NonZeroUsize::get
  %1 = call i64 @_ZN4core3num7nonzero12NonZeroUsize3get17h9afe58b26c3b3ebfE(i64 %_2)
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %1
}

; core::alloc::layout::Layout::array
; Function Attrs: inlinehint uwtable
define internal { i64, i64 } @_ZN4core5alloc6layout6Layout5array17h4b565579a9208ff5E(i64 %n) unnamed_addr #0 {
start:
  %_3 = alloca { i64, i64 }, align 8
  %0 = alloca { i64, i64 }, align 8
  br label %bb1

bb1:                                              ; preds = %start
; call core::num::<impl usize>::checked_mul
  %1 = call { i64, i64 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_mul17hd8a29a9a7afade75E"(i64 1, i64 %n)
  %_5.0 = extractvalue { i64, i64 } %1, 0
  %_5.1 = extractvalue { i64, i64 } %1, 1
  br label %bb2

bb2:                                              ; preds = %bb1
; call core::option::Option<T>::ok_or
  %2 = call { i64, i64 } @"_ZN4core6option15Option$LT$T$GT$5ok_or17h8f248f15f7aa6417E"(i64 %_5.0, i64 %_5.1)
  %_4.0 = extractvalue { i64, i64 } %2, 0
  %_4.1 = extractvalue { i64, i64 } %2, 1
  br label %bb3

bb3:                                              ; preds = %bb2
; call <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
  %3 = call { i64, i64 } @"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h31957b1b4ed229ceE"(i64 %_4.0, i64 %_4.1)
  store { i64, i64 } %3, { i64, i64 }* %_3, align 8
  br label %bb4

bb4:                                              ; preds = %bb3
  %4 = bitcast { i64, i64 }* %_3 to i64*
  %_9 = load i64, i64* %4, align 8, !range !2, !noundef !3
  switch i64 %_9, label %bb6 [
    i64 0, label %bb5
    i64 1, label %bb7
  ]

bb6:                                              ; preds = %bb4
  unreachable

bb5:                                              ; preds = %bb4
  %5 = bitcast { i64, i64 }* %_3 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Continue"*
  %6 = getelementptr inbounds %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Continue", %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Continue"* %5, i32 0, i32 1
  %val = load i64, i64* %6, align 8
  br label %bb9

bb7:                                              ; preds = %bb4
; call <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
  %7 = call { i64, i64 } @"_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17hc7c150c0a4246803E"(%"core::panic::location::Location"* align 8 bitcast (<{ i8*, [16 x i8] }>* @alloc22 to %"core::panic::location::Location"*))
  store { i64, i64 } %7, { i64, i64 }* %0, align 8
  br label %bb8

bb8:                                              ; preds = %bb7
  br label %bb11

bb11:                                             ; preds = %bb10, %bb8
  %8 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 0
  %9 = load i64, i64* %8, align 8
  %10 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  %11 = load i64, i64* %10, align 8
  %12 = insertvalue { i64, i64 } undef, i64 %9, 0
  %13 = insertvalue { i64, i64 } %12, i64 %11, 1
  ret { i64, i64 } %13

bb9:                                              ; preds = %bb5
; call core::alloc::layout::Layout::from_size_align_unchecked
  %14 = call { i64, i64 } @_ZN4core5alloc6layout6Layout25from_size_align_unchecked17hb8d1ebdf7c1ec12cE(i64 %val, i64 1)
  %_13.0 = extractvalue { i64, i64 } %14, 0
  %_13.1 = extractvalue { i64, i64 } %14, 1
  br label %bb10

bb10:                                             ; preds = %bb9
  %15 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 0
  store i64 %_13.0, i64* %15, align 8
  %16 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  store i64 %_13.1, i64* %16, align 8
  br label %bb11
}

; core::alloc::layout::Layout::array
; Function Attrs: inlinehint uwtable
define internal { i64, i64 } @_ZN4core5alloc6layout6Layout5array17h60d3921e481ba807E(i64 %n) unnamed_addr #0 {
start:
  %_3 = alloca { i64, i64 }, align 8
  %0 = alloca { i64, i64 }, align 8
  br label %bb1

bb1:                                              ; preds = %start
; call core::num::<impl usize>::checked_mul
  %1 = call { i64, i64 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_mul17hd8a29a9a7afade75E"(i64 24, i64 %n)
  %_5.0 = extractvalue { i64, i64 } %1, 0
  %_5.1 = extractvalue { i64, i64 } %1, 1
  br label %bb2

bb2:                                              ; preds = %bb1
; call core::option::Option<T>::ok_or
  %2 = call { i64, i64 } @"_ZN4core6option15Option$LT$T$GT$5ok_or17h8f248f15f7aa6417E"(i64 %_5.0, i64 %_5.1)
  %_4.0 = extractvalue { i64, i64 } %2, 0
  %_4.1 = extractvalue { i64, i64 } %2, 1
  br label %bb3

bb3:                                              ; preds = %bb2
; call <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
  %3 = call { i64, i64 } @"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h31957b1b4ed229ceE"(i64 %_4.0, i64 %_4.1)
  store { i64, i64 } %3, { i64, i64 }* %_3, align 8
  br label %bb4

bb4:                                              ; preds = %bb3
  %4 = bitcast { i64, i64 }* %_3 to i64*
  %_9 = load i64, i64* %4, align 8, !range !2, !noundef !3
  switch i64 %_9, label %bb6 [
    i64 0, label %bb5
    i64 1, label %bb7
  ]

bb6:                                              ; preds = %bb4
  unreachable

bb5:                                              ; preds = %bb4
  %5 = bitcast { i64, i64 }* %_3 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Continue"*
  %6 = getelementptr inbounds %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Continue", %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Continue"* %5, i32 0, i32 1
  %val = load i64, i64* %6, align 8
  br label %bb9

bb7:                                              ; preds = %bb4
; call <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
  %7 = call { i64, i64 } @"_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17hc7c150c0a4246803E"(%"core::panic::location::Location"* align 8 bitcast (<{ i8*, [16 x i8] }>* @alloc22 to %"core::panic::location::Location"*))
  store { i64, i64 } %7, { i64, i64 }* %0, align 8
  br label %bb8

bb8:                                              ; preds = %bb7
  br label %bb11

bb11:                                             ; preds = %bb10, %bb8
  %8 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 0
  %9 = load i64, i64* %8, align 8
  %10 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  %11 = load i64, i64* %10, align 8
  %12 = insertvalue { i64, i64 } undef, i64 %9, 0
  %13 = insertvalue { i64, i64 } %12, i64 %11, 1
  ret { i64, i64 } %13

bb9:                                              ; preds = %bb5
; call core::alloc::layout::Layout::from_size_align_unchecked
  %14 = call { i64, i64 } @_ZN4core5alloc6layout6Layout25from_size_align_unchecked17hb8d1ebdf7c1ec12cE(i64 %val, i64 8)
  %_13.0 = extractvalue { i64, i64 } %14, 0
  %_13.1 = extractvalue { i64, i64 } %14, 1
  br label %bb10

bb10:                                             ; preds = %bb9
  %15 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 0
  store i64 %_13.0, i64* %15, align 8
  %16 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  store i64 %_13.1, i64* %16, align 8
  br label %bb11
}

; core::option::Option<T>::ok_or
; Function Attrs: inlinehint uwtable
define internal { i64, i64 } @"_ZN4core6option15Option$LT$T$GT$5ok_or17h8f248f15f7aa6417E"(i64 %0, i64 %1) unnamed_addr #0 {
start:
  %_7 = alloca i8, align 1
  %2 = alloca { i64, i64 }, align 8
  %self = alloca { i64, i64 }, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 0
  store i64 %0, i64* %3, align 8
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  store i64 %1, i64* %4, align 8
  store i8 0, i8* %_7, align 1
  store i8 1, i8* %_7, align 1
  %5 = bitcast { i64, i64 }* %self to i64*
  %_3 = load i64, i64* %5, align 8, !range !2, !noundef !3
  switch i64 %_3, label %bb2 [
    i64 0, label %bb1
    i64 1, label %bb3
  ]

bb2:                                              ; preds = %start
  unreachable

bb1:                                              ; preds = %start
  store i8 0, i8* %_7, align 1
  %6 = bitcast { i64, i64 }* %2 to %"core::result::Result<usize, core::alloc::layout::LayoutError>::Err"*
  %7 = getelementptr inbounds %"core::result::Result<usize, core::alloc::layout::LayoutError>::Err", %"core::result::Result<usize, core::alloc::layout::LayoutError>::Err"* %6, i32 0, i32 1
  %8 = bitcast { i64, i64 }* %2 to i64*
  store i64 1, i64* %8, align 8
  br label %bb6

bb3:                                              ; preds = %start
  %9 = bitcast { i64, i64 }* %self to %"core::option::Option<usize>::Some"*
  %10 = getelementptr inbounds %"core::option::Option<usize>::Some", %"core::option::Option<usize>::Some"* %9, i32 0, i32 1
  %v = load i64, i64* %10, align 8
  %11 = bitcast { i64, i64 }* %2 to %"core::result::Result<usize, core::alloc::layout::LayoutError>::Ok"*
  %12 = getelementptr inbounds %"core::result::Result<usize, core::alloc::layout::LayoutError>::Ok", %"core::result::Result<usize, core::alloc::layout::LayoutError>::Ok"* %11, i32 0, i32 1
  store i64 %v, i64* %12, align 8
  %13 = bitcast { i64, i64 }* %2 to i64*
  store i64 0, i64* %13, align 8
  br label %bb6

bb6:                                              ; preds = %bb1, %bb3
  %14 = load i8, i8* %_7, align 1, !range !5, !noundef !3
  %15 = trunc i8 %14 to i1
  br i1 %15, label %bb5, label %bb4

bb4:                                              ; preds = %bb5, %bb6
  %16 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %2, i32 0, i32 0
  %17 = load i64, i64* %16, align 8, !range !2, !noundef !3
  %18 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %2, i32 0, i32 1
  %19 = load i64, i64* %18, align 8
  %20 = insertvalue { i64, i64 } undef, i64 %17, 0
  %21 = insertvalue { i64, i64 } %20, i64 %19, 1
  ret { i64, i64 } %21

bb5:                                              ; preds = %bb6
  br label %bb4
}

; core::result::Result<T,E>::unwrap_unchecked
; Function Attrs: inlinehint uwtable
define internal { i64, i64 } @"_ZN4core6result19Result$LT$T$C$E$GT$16unwrap_unchecked17h7c70a8d02defbab7E"(i64 %0, i64 %1, %"core::panic::location::Location"* align 8 %2) unnamed_addr #0 personality i32 (...)* @__CxxFrameHandler3 {
start:
  %_4 = alloca i8, align 1
  %self = alloca { i64, i64 }, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 0
  store i64 %0, i64* %3, align 8
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  store i64 %1, i64* %4, align 8
  store i8 0, i8* %_4, align 1
  store i8 1, i8* %_4, align 1
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
  %7 = icmp eq i64 %6, 0
  %_2 = select i1 %7, i64 1, i64 0
  switch i64 %_2, label %bb2 [
    i64 0, label %bb3
    i64 1, label %bb1
  ]

bb2:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  store i8 0, i8* %_4, align 1
  %8 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 0
  %t.0 = load i64, i64* %8, align 8
  %9 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  %t.1 = load i64, i64* %9, align 8, !range !6, !noundef !3
  %10 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  %11 = load i64, i64* %10, align 8
  %12 = icmp eq i64 %11, 0
  %_5 = select i1 %12, i64 1, i64 0
  %13 = icmp eq i64 %_5, 0
  br i1 %13, label %bb5, label %bb6

bb1:                                              ; preds = %start
; invoke core::hint::unreachable_unchecked
  invoke void @_ZN4core4hint21unreachable_unchecked17hb433023e7dc94f82E() #14
          to label %unreachable unwind label %funclet_bb10

bb10:                                             ; preds = %funclet_bb10
  %14 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  %15 = load i64, i64* %14, align 8
  %16 = icmp eq i64 %15, 0
  %_6 = select i1 %16, i64 1, i64 0
  %17 = icmp eq i64 %_6, 0
  br i1 %17, label %bb7, label %bb9

funclet_bb10:                                     ; preds = %bb1
  %cleanuppad = cleanuppad within none []
  br label %bb10

unreachable:                                      ; preds = %bb1
  unreachable

bb7:                                              ; preds = %bb10
  %18 = load i8, i8* %_4, align 1, !range !5, !noundef !3
  %19 = trunc i8 %18 to i1
  br i1 %19, label %bb8, label %bb4

bb9:                                              ; preds = %bb10
  br label %bb4

bb4:                                              ; preds = %bb8, %bb7, %bb9
  cleanupret from %cleanuppad unwind to caller

bb8:                                              ; preds = %bb7
  br label %bb4

bb5:                                              ; preds = %bb6, %bb3
  %20 = insertvalue { i64, i64 } undef, i64 %t.0, 0
  %21 = insertvalue { i64, i64 } %20, i64 %t.1, 1
  ret { i64, i64 } %21

bb6:                                              ; preds = %bb3
  br label %bb5
}

; core::panicking::assert_failed
; Function Attrs: cold noreturn uwtable
define internal void @_ZN4core9panicking13assert_failed17hc31224a192f3a61dE(i8 %kind, { i64, i64 }* align 8 %0, { i64, i64 }* align 8 %1, %"core::option::Option<core::fmt::Arguments>"* %args, %"core::panic::location::Location"* align 8 %2) unnamed_addr #4 {
start:
  %_12 = alloca %"core::option::Option<core::fmt::Arguments>", align 8
  %right = alloca { i64, i64 }*, align 8
  %left = alloca { i64, i64 }*, align 8
  store { i64, i64 }* %0, { i64, i64 }** %left, align 8
  store { i64, i64 }* %1, { i64, i64 }** %right, align 8
  %_6.0 = bitcast { i64, i64 }** %left to {}*
  %_9.0 = bitcast { i64, i64 }** %right to {}*
  %3 = bitcast %"core::option::Option<core::fmt::Arguments>"* %_12 to i8*
  %4 = bitcast %"core::option::Option<core::fmt::Arguments>"* %args to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %3, i8* align 8 %4, i64 48, i1 false)
; call core::panicking::assert_failed_inner
  call void @_ZN4core9panicking19assert_failed_inner17hf17b0c5cd84309eaE(i8 %kind, {}* align 1 %_6.0, [3 x i64]* align 8 bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.1 to [3 x i64]*), {}* align 1 %_9.0, [3 x i64]* align 8 bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.1 to [3 x i64]*), %"core::option::Option<core::fmt::Arguments>"* %_12, %"core::panic::location::Location"* align 8 %2) #14
  unreachable
}

; <T as core::convert::From<T>>::from
; Function Attrs: uwtable
define internal void @"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17hbda7aff5bc07100dE"() unnamed_addr #1 {
start:
  ret void
}

; <T as core::convert::Into<U>>::into
; Function Attrs: uwtable
define internal i8* @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17hed7cd5e609af2246E"(i8* %self) unnamed_addr #1 {
start:
; call <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
  %0 = call i8* @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17h2eb096bfec32d0f7E"(i8* %self)
  br label %bb1

bb1:                                              ; preds = %start
  ret i8* %0
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hce51499ee67d1f1bE"() unnamed_addr #0 {
start:
; call <std::process::ExitCode as std::process::Termination>::report
  %0 = call i32 @"_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17ha457fe0fe5483d3fE"(i32 0)
  br label %bb1

bb1:                                              ; preds = %start
  ret i32 %0
}

; alloc::vec::Vec<T,A>::as_mut_ptr
; Function Attrs: inlinehint uwtable
define internal i8* @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$10as_mut_ptr17h55c4efb05310cafbE"(%"alloc::vec::Vec<u8>"* align 8 %self) unnamed_addr #0 {
start:
  %_2 = bitcast %"alloc::vec::Vec<u8>"* %self to { i8*, i64 }*
; call alloc::raw_vec::RawVec<T,A>::ptr
  %ptr = call i8* @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$3ptr17h3050778617ba4be4E"({ i8*, i64 }* align 8 %_2)
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::mut_ptr::<impl *mut T>::is_null
  %_5 = call zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17hb7643854fcfece7fE"(i8* %ptr)
  br label %bb2

bb2:                                              ; preds = %bb1
  %_4 = xor i1 %_5, true
  call void @llvm.assume(i1 %_4)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret i8* %ptr
}

; alloc::vec::into_iter::IntoIter<T,A>::as_raw_mut_slice
; Function Attrs: uwtable
define internal { [0 x %"std::ffi::os_str::OsString"]*, i64 } @"_ZN5alloc3vec9into_iter21IntoIter$LT$T$C$A$GT$16as_raw_mut_slice17h50051ddb97248dd5E"(%"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"* align 8 %0) unnamed_addr #1 {
start:
  %self = alloca %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"*, align 8
  store %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"* %0, %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"** %self, align 8
  %1 = load %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"*, %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"** %self, align 8, !nonnull !3, !align !4, !noundef !3
  %2 = getelementptr inbounds %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>", %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"* %1, i32 0, i32 4
  %_3 = load %"std::ffi::os_str::OsString"*, %"std::ffi::os_str::OsString"** %2, align 8
; call <&mut I as core::iter::traits::exact_size::ExactSizeIterator>::len
  %_4 = call i64 @"_ZN83_$LT$$RF$mut$u20$I$u20$as$u20$core..iter..traits..exact_size..ExactSizeIterator$GT$3len17hee7ee6a51bd4b4fdE"(%"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"** align 8 %self)
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::slice_from_raw_parts_mut
  %3 = call { [0 x %"std::ffi::os_str::OsString"]*, i64 } @_ZN4core3ptr24slice_from_raw_parts_mut17h8ebef298811a14e5E(%"std::ffi::os_str::OsString"* %_3, i64 %_4)
  %4 = extractvalue { [0 x %"std::ffi::os_str::OsString"]*, i64 } %3, 0
  %5 = extractvalue { [0 x %"std::ffi::os_str::OsString"]*, i64 } %3, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  %6 = insertvalue { [0 x %"std::ffi::os_str::OsString"]*, i64 } undef, [0 x %"std::ffi::os_str::OsString"]* %4, 0
  %7 = insertvalue { [0 x %"std::ffi::os_str::OsString"]*, i64 } %6, i64 %5, 1
  ret { [0 x %"std::ffi::os_str::OsString"]*, i64 } %7
}

; alloc::alloc::dealloc
; Function Attrs: inlinehint uwtable
define internal void @_ZN5alloc5alloc7dealloc17h074be10578023ebbE(i8* %ptr, i64 %0, i64 %1) unnamed_addr #0 {
start:
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call core::alloc::layout::Layout::size
  %_4 = call i64 @_ZN4core5alloc6layout6Layout4size17h557370479cae743eE({ i64, i64 }* align 8 %layout)
  br label %bb1

bb1:                                              ; preds = %start
; call core::alloc::layout::Layout::align
  %_6 = call i64 @_ZN4core5alloc6layout6Layout5align17h4ca7a91dafbd7cc1E({ i64, i64 }* align 8 %layout)
  br label %bb2

bb2:                                              ; preds = %bb1
  call void @__rust_dealloc(i8* %ptr, i64 %_4, i64 %_6) #15
  br label %bb3

bb3:                                              ; preds = %bb2
  ret void
}

; alloc::raw_vec::RawVec<T,A>::current_memory
; Function Attrs: uwtable
define internal void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h6c7df96ed1b5678cE"(%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* sret(%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>") %0, { i64*, i64 }* align 8 %self) unnamed_addr #1 {
start:
  %_9 = alloca { i8*, { i64, i64 } }, align 8
  %_2 = alloca i8, align 1
  br label %bb4

bb4:                                              ; preds = %start
  %1 = icmp eq i64 24, 0
  br i1 %1, label %bb1, label %bb2

bb1:                                              ; preds = %bb4
  store i8 1, i8* %_2, align 1
  br label %bb3

bb2:                                              ; preds = %bb4
  %2 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %self, i32 0, i32 1
  %_5 = load i64, i64* %2, align 8
  %_4 = icmp eq i64 %_5, 0
  %3 = zext i1 %_4 to i8
  store i8 %3, i8* %_2, align 1
  br label %bb3

bb3:                                              ; preds = %bb1, %bb2
  %4 = load i8, i8* %_2, align 1, !range !5, !noundef !3
  %5 = trunc i8 %4 to i1
  br i1 %5, label %bb5, label %bb6

bb6:                                              ; preds = %bb3
  %6 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %self, i32 0, i32 1
  %_8 = load i64, i64* %6, align 8
; call core::alloc::layout::Layout::array
  %7 = call { i64, i64 } @_ZN4core5alloc6layout6Layout5array17h60d3921e481ba807E(i64 %_8)
  %_7.0 = extractvalue { i64, i64 } %7, 0
  %_7.1 = extractvalue { i64, i64 } %7, 1
  br label %bb7

bb5:                                              ; preds = %bb3
  %8 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %0 to {}**
  store {}* null, {}** %8, align 8
  br label %bb11

bb11:                                             ; preds = %bb10, %bb5
  ret void

bb7:                                              ; preds = %bb6
; call core::result::Result<T,E>::unwrap_unchecked
  %9 = call { i64, i64 } @"_ZN4core6result19Result$LT$T$C$E$GT$16unwrap_unchecked17h7c70a8d02defbab7E"(i64 %_7.0, i64 %_7.1, %"core::panic::location::Location"* align 8 bitcast (<{ i8*, [16 x i8] }>* @alloc27 to %"core::panic::location::Location"*))
  %layout.0 = extractvalue { i64, i64 } %9, 0
  %layout.1 = extractvalue { i64, i64 } %9, 1
  br label %bb8

bb8:                                              ; preds = %bb7
  %10 = bitcast { i64*, i64 }* %self to i64**
  %_12 = load i64*, i64** %10, align 8, !nonnull !3, !noundef !3
; call core::ptr::unique::Unique<T>::cast
  %_11 = call i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17h56e10057c8f7d5c7E"(i64* %_12)
  br label %bb9

bb9:                                              ; preds = %bb8
; call <T as core::convert::Into<U>>::into
  %_10 = call i8* @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17hed7cd5e609af2246E"(i8* %_11)
  br label %bb10

bb10:                                             ; preds = %bb9
  %11 = bitcast { i8*, { i64, i64 } }* %_9 to i8**
  store i8* %_10, i8** %11, align 8
  %12 = getelementptr inbounds { i8*, { i64, i64 } }, { i8*, { i64, i64 } }* %_9, i32 0, i32 1
  %13 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %12, i32 0, i32 0
  store i64 %layout.0, i64* %13, align 8
  %14 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %12, i32 0, i32 1
  store i64 %layout.1, i64* %14, align 8
  %15 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %0 to %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"*
  %16 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"* %15 to { i8*, { i64, i64 } }*
  %17 = bitcast { i8*, { i64, i64 } }* %16 to i8*
  %18 = bitcast { i8*, { i64, i64 } }* %_9 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %17, i8* align 8 %18, i64 24, i1 false)
  br label %bb11
}

; alloc::raw_vec::RawVec<T,A>::current_memory
; Function Attrs: uwtable
define internal void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hf0df4d78afb7cf25E"(%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* sret(%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>") %0, { i8*, i64 }* align 8 %self) unnamed_addr #1 {
start:
  %_9 = alloca { i8*, { i64, i64 } }, align 8
  %_2 = alloca i8, align 1
  br label %bb4

bb4:                                              ; preds = %start
  %1 = icmp eq i64 1, 0
  br i1 %1, label %bb1, label %bb2

bb1:                                              ; preds = %bb4
  store i8 1, i8* %_2, align 1
  br label %bb3

bb2:                                              ; preds = %bb4
  %2 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %self, i32 0, i32 1
  %_5 = load i64, i64* %2, align 8
  %_4 = icmp eq i64 %_5, 0
  %3 = zext i1 %_4 to i8
  store i8 %3, i8* %_2, align 1
  br label %bb3

bb3:                                              ; preds = %bb1, %bb2
  %4 = load i8, i8* %_2, align 1, !range !5, !noundef !3
  %5 = trunc i8 %4 to i1
  br i1 %5, label %bb5, label %bb6

bb6:                                              ; preds = %bb3
  %6 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %self, i32 0, i32 1
  %_8 = load i64, i64* %6, align 8
; call core::alloc::layout::Layout::array
  %7 = call { i64, i64 } @_ZN4core5alloc6layout6Layout5array17h4b565579a9208ff5E(i64 %_8)
  %_7.0 = extractvalue { i64, i64 } %7, 0
  %_7.1 = extractvalue { i64, i64 } %7, 1
  br label %bb7

bb5:                                              ; preds = %bb3
  %8 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %0 to {}**
  store {}* null, {}** %8, align 8
  br label %bb11

bb11:                                             ; preds = %bb10, %bb5
  ret void

bb7:                                              ; preds = %bb6
; call core::result::Result<T,E>::unwrap_unchecked
  %9 = call { i64, i64 } @"_ZN4core6result19Result$LT$T$C$E$GT$16unwrap_unchecked17h7c70a8d02defbab7E"(i64 %_7.0, i64 %_7.1, %"core::panic::location::Location"* align 8 bitcast (<{ i8*, [16 x i8] }>* @alloc27 to %"core::panic::location::Location"*))
  %layout.0 = extractvalue { i64, i64 } %9, 0
  %layout.1 = extractvalue { i64, i64 } %9, 1
  br label %bb8

bb8:                                              ; preds = %bb7
  %10 = bitcast { i8*, i64 }* %self to i8**
  %_12 = load i8*, i8** %10, align 8, !nonnull !3, !noundef !3
; call core::ptr::unique::Unique<T>::cast
  %_11 = call i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17hec46cb201340efc6E"(i8* %_12)
  br label %bb9

bb9:                                              ; preds = %bb8
; call <T as core::convert::Into<U>>::into
  %_10 = call i8* @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17hed7cd5e609af2246E"(i8* %_11)
  br label %bb10

bb10:                                             ; preds = %bb9
  %11 = bitcast { i8*, { i64, i64 } }* %_9 to i8**
  store i8* %_10, i8** %11, align 8
  %12 = getelementptr inbounds { i8*, { i64, i64 } }, { i8*, { i64, i64 } }* %_9, i32 0, i32 1
  %13 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %12, i32 0, i32 0
  store i64 %layout.0, i64* %13, align 8
  %14 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %12, i32 0, i32 1
  store i64 %layout.1, i64* %14, align 8
  %15 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %0 to %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"*
  %16 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"* %15 to { i8*, { i64, i64 } }*
  %17 = bitcast { i8*, { i64, i64 } }* %16 to i8*
  %18 = bitcast { i8*, { i64, i64 } }* %_9 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %17, i8* align 8 %18, i64 24, i1 false)
  br label %bb11
}

; alloc::raw_vec::RawVec<T,A>::from_raw_parts_in
; Function Attrs: inlinehint uwtable
define internal { i64*, i64 } @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$17from_raw_parts_in17h548863d6577571bdE"(%"std::ffi::os_str::OsString"* %ptr, i64 %capacity) unnamed_addr #0 personality i32 (...)* @__CxxFrameHandler3 {
start:
  %0 = alloca { i64*, i64 }, align 8
; invoke core::ptr::unique::Unique<T>::new_unchecked
  %_4 = invoke i64* @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17ha674211a1d897b29E"(%"std::ffi::os_str::OsString"* %ptr)
          to label %bb1 unwind label %funclet_bb2

bb2:                                              ; preds = %funclet_bb2
  br label %bb3

funclet_bb2:                                      ; preds = %start
  %cleanuppad = cleanuppad within none []
  br label %bb2

bb1:                                              ; preds = %start
  %1 = bitcast { i64*, i64 }* %0 to i64**
  store i64* %_4, i64** %1, align 8
  %2 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %0, i32 0, i32 1
  store i64 %capacity, i64* %2, align 8
  %3 = bitcast { i64*, i64 }* %0 to %"alloc::alloc::Global"*
  %4 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %0, i32 0, i32 0
  %5 = load i64*, i64** %4, align 8, !nonnull !3, !noundef !3
  %6 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %0, i32 0, i32 1
  %7 = load i64, i64* %6, align 8
  %8 = insertvalue { i64*, i64 } undef, i64* %5, 0
  %9 = insertvalue { i64*, i64 } %8, i64 %7, 1
  ret { i64*, i64 } %9

bb3:                                              ; preds = %bb2
  cleanupret from %cleanuppad unwind to caller
}

; alloc::raw_vec::RawVec<T,A>::ptr
; Function Attrs: inlinehint uwtable
define internal i8* @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$3ptr17h3050778617ba4be4E"({ i8*, i64 }* align 8 %self) unnamed_addr #0 {
start:
  %0 = bitcast { i8*, i64 }* %self to i8**
  %_2 = load i8*, i8** %0, align 8, !nonnull !3, !noundef !3
; call core::ptr::unique::Unique<T>::as_ptr
  %1 = call i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h9206e7cf0f3ae9e0E"(i8* %_2)
  br label %bb1

bb1:                                              ; preds = %start
  ret i8* %1
}

; <alloc::alloc::Global as core::alloc::Allocator>::deallocate
; Function Attrs: inlinehint uwtable
define internal void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hb13e999b87819646E"(%"alloc::alloc::Global"* align 1 %self, i8* %ptr, i64 %0, i64 %1) unnamed_addr #0 {
start:
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call core::alloc::layout::Layout::size
  %_4 = call i64 @_ZN4core5alloc6layout6Layout4size17h557370479cae743eE({ i64, i64 }* align 8 %layout)
  br label %bb1

bb1:                                              ; preds = %start
  %4 = icmp eq i64 %_4, 0
  br i1 %4, label %bb5, label %bb2

bb5:                                              ; preds = %bb1
  br label %bb6

bb2:                                              ; preds = %bb1
; call core::ptr::non_null::NonNull<T>::as_ptr
  %_6 = call i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h5e59b14b9cab6074E"(i8* %ptr)
  br label %bb3

bb3:                                              ; preds = %bb2
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  %_8.0 = load i64, i64* %5, align 8
  %6 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  %_8.1 = load i64, i64* %6, align 8, !range !6, !noundef !3
; call alloc::alloc::dealloc
  call void @_ZN5alloc5alloc7dealloc17h074be10578023ebbE(i8* %_6, i64 %_8.0, i64 %_8.1)
  br label %bb4

bb4:                                              ; preds = %bb3
  br label %bb6

bb6:                                              ; preds = %bb5, %bb4
  ret void
}

; <core::option::Option<T> as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal zeroext i1 @"_ZN66_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h82beae0e53d79ef7E"({ i64, i64 }* align 8 %self, %"core::fmt::Formatter"* align 8 %f) unnamed_addr #1 {
start:
  %_20 = alloca i64*, align 8
  %_11 = alloca %"core::fmt::builders::DebugTuple", align 8
  %_3 = alloca i64*, align 8
  %0 = alloca i8, align 1
  %1 = bitcast i64** %_3 to { i64, i64 }**
  store { i64, i64 }* %self, { i64, i64 }** %1, align 8
  %2 = bitcast i64** %_3 to { i64, i64 }**
  %3 = load { i64, i64 }*, { i64, i64 }** %2, align 8, !nonnull !3, !align !4, !noundef !3
  %4 = bitcast { i64, i64 }* %3 to i64*
  %_5 = load i64, i64* %4, align 8, !range !2, !noundef !3
  switch i64 %_5, label %bb2 [
    i64 0, label %bb3
    i64 1, label %bb1
  ]

bb2:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
; call core::fmt::Formatter::write_str
  %5 = call zeroext i1 @_ZN4core3fmt9Formatter9write_str17h167c82fdf2572f1fE(%"core::fmt::Formatter"* align 8 %f, [0 x i8]* align 1 bitcast (<{ [4 x i8] }>* @alloc32 to [0 x i8]*), i64 4)
  %6 = zext i1 %5 to i8
  store i8 %6, i8* %0, align 1
  br label %bb4

bb1:                                              ; preds = %start
  %7 = bitcast i64** %_3 to { i64, i64 }**
  %8 = load { i64, i64 }*, { i64, i64 }** %7, align 8, !nonnull !3, !align !4, !noundef !3
  %9 = bitcast { i64, i64 }* %8 to %"core::option::Option<usize>::Some"*
  %__self_0 = getelementptr inbounds %"core::option::Option<usize>::Some", %"core::option::Option<usize>::Some"* %9, i32 0, i32 1
; call core::fmt::Formatter::debug_tuple
  call void @_ZN4core3fmt9Formatter11debug_tuple17h6577318650ca455bE(%"core::fmt::builders::DebugTuple"* sret(%"core::fmt::builders::DebugTuple") %_11, %"core::fmt::Formatter"* align 8 %f, [0 x i8]* align 1 bitcast (<{ [4 x i8] }>* @alloc28 to [0 x i8]*), i64 4)
  br label %bb5

bb5:                                              ; preds = %bb1
  store i64* %__self_0, i64** %_20, align 8
  %_17.0 = bitcast i64** %_20 to {}*
; call core::fmt::builders::DebugTuple::field
  %_15 = call align 8 %"core::fmt::builders::DebugTuple"* @_ZN4core3fmt8builders10DebugTuple5field17h206c25a3d0cf074eE(%"core::fmt::builders::DebugTuple"* align 8 %_11, {}* align 1 %_17.0, [3 x i64]* align 8 bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.2 to [3 x i64]*))
  br label %bb6

bb6:                                              ; preds = %bb5
; call core::fmt::builders::DebugTuple::finish
  %10 = call zeroext i1 @_ZN4core3fmt8builders10DebugTuple6finish17h739a8d7c87c0d37cE(%"core::fmt::builders::DebugTuple"* align 8 %_11)
  %11 = zext i1 %10 to i8
  store i8 %11, i8* %0, align 1
  br label %bb7

bb7:                                              ; preds = %bb6
  br label %bb8

bb8:                                              ; preds = %bb4, %bb7
  %12 = load i8, i8* %0, align 1, !range !5, !noundef !3
  %13 = trunc i8 %12 to i1
  ret i1 %13

bb4:                                              ; preds = %bb3
  br label %bb8
}

; <std::process::ExitCode as std::process::Termination>::report
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN68_$LT$std..process..ExitCode$u20$as$u20$std..process..Termination$GT$6report17ha457fe0fe5483d3fE"(i32 %self) unnamed_addr #0 {
start:
  ret i32 %self
}

; <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define internal void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h05ab1c2b38a1fc9dE"(%"alloc::vec::Vec<u8>"* align 8 %self) unnamed_addr #1 {
start:
; call alloc::vec::Vec<T,A>::as_mut_ptr
  %_3 = call i8* @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$10as_mut_ptr17h55c4efb05310cafbE"(%"alloc::vec::Vec<u8>"* align 8 %self)
  br label %bb1

bb1:                                              ; preds = %start
  %0 = getelementptr inbounds %"alloc::vec::Vec<u8>", %"alloc::vec::Vec<u8>"* %self, i32 0, i32 1
  %_5 = load i64, i64* %0, align 8
; call core::ptr::slice_from_raw_parts_mut
  %1 = call { [0 x i8]*, i64 } @_ZN4core3ptr24slice_from_raw_parts_mut17h589593876c82e883E(i8* %_3, i64 %_5)
  %_2.0 = extractvalue { [0 x i8]*, i64 } %1, 0
  %_2.1 = extractvalue { [0 x i8]*, i64 } %1, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  br label %bb3

bb3:                                              ; preds = %bb2
  ret void
}

; <core::option::Option<T> as core::cmp::PartialEq>::eq
; Function Attrs: inlinehint uwtable
define internal zeroext i1 @"_ZN70_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..cmp..PartialEq$GT$2eq17h31979d036de26d1aE"({ i64, i64 }* align 8 %self, { i64, i64 }* align 8 %other) unnamed_addr #0 {
start:
  %_13 = alloca { i64*, i64* }, align 8
  %0 = alloca i8, align 1
  %1 = bitcast { i64, i64 }* %self to i64*
  %__self_vi = load i64, i64* %1, align 8, !range !2, !noundef !3
  %2 = bitcast { i64, i64 }* %other to i64*
  %__arg_1_vi = load i64, i64* %2, align 8, !range !2, !noundef !3
  %_10 = icmp eq i64 %__self_vi, %__arg_1_vi
  br i1 %_10, label %bb1, label %bb6

bb6:                                              ; preds = %start
  store i8 0, i8* %0, align 1
  br label %bb7

bb1:                                              ; preds = %start
  %3 = bitcast { i64*, i64* }* %_13 to { i64, i64 }**
  store { i64, i64 }* %self, { i64, i64 }** %3, align 8
  %4 = getelementptr inbounds { i64*, i64* }, { i64*, i64* }* %_13, i32 0, i32 1
  %5 = bitcast i64** %4 to { i64, i64 }**
  store { i64, i64 }* %other, { i64, i64 }** %5, align 8
  %6 = bitcast { i64*, i64* }* %_13 to { i64, i64 }**
  %7 = load { i64, i64 }*, { i64, i64 }** %6, align 8, !nonnull !3, !align !4, !noundef !3
  %8 = bitcast { i64, i64 }* %7 to i64*
  %_16 = load i64, i64* %8, align 8, !range !2, !noundef !3
  %9 = getelementptr inbounds { i64*, i64* }, { i64*, i64* }* %_13, i32 0, i32 1
  %10 = bitcast i64** %9 to { i64, i64 }**
  %11 = load { i64, i64 }*, { i64, i64 }** %10, align 8, !nonnull !3, !align !4, !noundef !3
  %12 = bitcast { i64, i64 }* %11 to i64*
  %_21 = load i64, i64* %12, align 8, !range !2, !noundef !3
  %_22 = icmp ne i64 %_16, %_21
  br i1 %_22, label %bb2, label %bb8

bb8:                                              ; preds = %bb1
  %13 = icmp eq i64 %_16, 1
  br i1 %13, label %bb3, label %bb2

bb2:                                              ; preds = %bb8, %bb1
  store i8 1, i8* %0, align 1
  br label %bb5

bb3:                                              ; preds = %bb8
  %14 = bitcast { i64*, i64* }* %_13 to { i64, i64 }**
  %15 = load { i64, i64 }*, { i64, i64 }** %14, align 8, !nonnull !3, !align !4, !noundef !3
  %16 = bitcast { i64, i64 }* %15 to %"core::option::Option<usize>::Some"*
  %__self_0 = getelementptr inbounds %"core::option::Option<usize>::Some", %"core::option::Option<usize>::Some"* %16, i32 0, i32 1
  %17 = getelementptr inbounds { i64*, i64* }, { i64*, i64* }* %_13, i32 0, i32 1
  %18 = bitcast i64** %17 to { i64, i64 }**
  %19 = load { i64, i64 }*, { i64, i64 }** %18, align 8, !nonnull !3, !align !4, !noundef !3
  %20 = bitcast { i64, i64 }* %19 to %"core::option::Option<usize>::Some"*
  %__arg_1_0 = getelementptr inbounds %"core::option::Option<usize>::Some", %"core::option::Option<usize>::Some"* %20, i32 0, i32 1
; call core::cmp::impls::<impl core::cmp::PartialEq for usize>::eq
  %21 = call zeroext i1 @"_ZN4core3cmp5impls56_$LT$impl$u20$core..cmp..PartialEq$u20$for$u20$usize$GT$2eq17h4bbeac1807bdf9dfE"(i64* align 8 %__self_0, i64* align 8 %__arg_1_0)
  %22 = zext i1 %21 to i8
  store i8 %22, i8* %0, align 1
  br label %bb4

bb5:                                              ; preds = %bb4, %bb2
  br label %bb7

bb4:                                              ; preds = %bb3
  br label %bb5

bb7:                                              ; preds = %bb6, %bb5
  %23 = load i8, i8* %0, align 1, !range !5, !noundef !3
  %24 = trunc i8 %23 to i1
  ret i1 %24
}

; <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define internal void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8dfb863328ab7775E"({ i8*, i64 }* align 8 %self) unnamed_addr #1 {
start:
  %_2 = alloca %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>", align 8
; call alloc::raw_vec::RawVec<T,A>::current_memory
  call void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hf0df4d78afb7cf25E"(%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* sret(%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>") %_2, { i8*, i64 }* align 8 %self)
  br label %bb1

bb1:                                              ; preds = %start
  %0 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %_2 to {}**
  %1 = load {}*, {}** %0, align 8
  %2 = icmp eq {}* %1, null
  %_4 = select i1 %2, i64 0, i64 1
  %3 = icmp eq i64 %_4, 1
  br i1 %3, label %bb2, label %bb4

bb2:                                              ; preds = %bb1
  %4 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %_2 to %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"*
  %5 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"* %4 to { i8*, { i64, i64 } }*
  %6 = bitcast { i8*, { i64, i64 } }* %5 to i8**
  %ptr = load i8*, i8** %6, align 8, !nonnull !3, !noundef !3
  %7 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %_2 to %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"*
  %8 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"* %7 to { i8*, { i64, i64 } }*
  %9 = getelementptr inbounds { i8*, { i64, i64 } }, { i8*, { i64, i64 } }* %8, i32 0, i32 1
  %10 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %9, i32 0, i32 0
  %layout.0 = load i64, i64* %10, align 8
  %11 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %9, i32 0, i32 1
  %layout.1 = load i64, i64* %11, align 8, !range !6, !noundef !3
  %_7 = bitcast { i8*, i64 }* %self to %"alloc::alloc::Global"*
; call <alloc::alloc::Global as core::alloc::Allocator>::deallocate
  call void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hb13e999b87819646E"(%"alloc::alloc::Global"* align 1 %_7, i8* %ptr, i64 %layout.0, i64 %layout.1)
  br label %bb3

bb4:                                              ; preds = %bb3, %bb1
  ret void

bb3:                                              ; preds = %bb2
  br label %bb4
}

; <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define internal void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hae891832f2f092c4E"({ i64*, i64 }* align 8 %self) unnamed_addr #1 {
start:
  %_2 = alloca %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>", align 8
; call alloc::raw_vec::RawVec<T,A>::current_memory
  call void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h6c7df96ed1b5678cE"(%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* sret(%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>") %_2, { i64*, i64 }* align 8 %self)
  br label %bb1

bb1:                                              ; preds = %start
  %0 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %_2 to {}**
  %1 = load {}*, {}** %0, align 8
  %2 = icmp eq {}* %1, null
  %_4 = select i1 %2, i64 0, i64 1
  %3 = icmp eq i64 %_4, 1
  br i1 %3, label %bb2, label %bb4

bb2:                                              ; preds = %bb1
  %4 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %_2 to %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"*
  %5 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"* %4 to { i8*, { i64, i64 } }*
  %6 = bitcast { i8*, { i64, i64 } }* %5 to i8**
  %ptr = load i8*, i8** %6, align 8, !nonnull !3, !noundef !3
  %7 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>"* %_2 to %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"*
  %8 = bitcast %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>::Some"* %7 to { i8*, { i64, i64 } }*
  %9 = getelementptr inbounds { i8*, { i64, i64 } }, { i8*, { i64, i64 } }* %8, i32 0, i32 1
  %10 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %9, i32 0, i32 0
  %layout.0 = load i64, i64* %10, align 8
  %11 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %9, i32 0, i32 1
  %layout.1 = load i64, i64* %11, align 8, !range !6, !noundef !3
  %_7 = bitcast { i64*, i64 }* %self to %"alloc::alloc::Global"*
; call <alloc::alloc::Global as core::alloc::Allocator>::deallocate
  call void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hb13e999b87819646E"(%"alloc::alloc::Global"* align 1 %_7, i8* %ptr, i64 %layout.0, i64 %layout.1)
  br label %bb3

bb4:                                              ; preds = %bb3, %bb1
  ret void

bb3:                                              ; preds = %bb2
  br label %bb4
}

; <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
; Function Attrs: inlinehint uwtable
define internal { i64, i64 } @"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h31957b1b4ed229ceE"(i64 %0, i64 %1) unnamed_addr #0 {
start:
  %_6 = alloca %"core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>::Err", align 1
  %2 = alloca { i64, i64 }, align 8
  %self = alloca { i64, i64 }, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 0
  store i64 %0, i64* %3, align 8
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  store i64 %1, i64* %4, align 8
  %5 = bitcast { i64, i64 }* %self to i64*
  %_2 = load i64, i64* %5, align 8, !range !2, !noundef !3
  switch i64 %_2, label %bb2 [
    i64 0, label %bb3
    i64 1, label %bb1
  ]

bb2:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  %6 = bitcast { i64, i64 }* %self to %"core::result::Result<usize, core::alloc::layout::LayoutError>::Ok"*
  %7 = getelementptr inbounds %"core::result::Result<usize, core::alloc::layout::LayoutError>::Ok", %"core::result::Result<usize, core::alloc::layout::LayoutError>::Ok"* %6, i32 0, i32 1
  %v = load i64, i64* %7, align 8
  %8 = bitcast { i64, i64 }* %2 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Continue"*
  %9 = getelementptr inbounds %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Continue", %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Continue"* %8, i32 0, i32 1
  store i64 %v, i64* %9, align 8
  %10 = bitcast { i64, i64 }* %2 to i64*
  store i64 0, i64* %10, align 8
  br label %bb4

bb1:                                              ; preds = %start
  %11 = bitcast %"core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>::Err"* %_6 to %"core::alloc::layout::LayoutError"*
  %12 = bitcast { i64, i64 }* %2 to %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Break"*
  %13 = getelementptr inbounds %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Break", %"core::ops::control_flow::ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::layout::LayoutError>, usize>::Break"* %12, i32 0, i32 1
  %14 = bitcast { i64, i64 }* %2 to i64*
  store i64 1, i64* %14, align 8
  br label %bb4

bb4:                                              ; preds = %bb3, %bb1
  %15 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %2, i32 0, i32 0
  %16 = load i64, i64* %15, align 8, !range !2, !noundef !3
  %17 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %2, i32 0, i32 1
  %18 = load i64, i64* %17, align 8
  %19 = insertvalue { i64, i64 } undef, i64 %16, 0
  %20 = insertvalue { i64, i64 } %19, i64 %18, 1
  ret { i64, i64 } %20
}

; <&mut I as core::iter::traits::exact_size::ExactSizeIterator>::len
; Function Attrs: uwtable
define internal i64 @"_ZN83_$LT$$RF$mut$u20$I$u20$as$u20$core..iter..traits..exact_size..ExactSizeIterator$GT$3len17hee7ee6a51bd4b4fdE"(%"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"** align 8 %self) unnamed_addr #1 {
start:
  %_2 = load %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"*, %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"** %self, align 8, !nonnull !3, !align !4, !noundef !3
; call core::iter::traits::exact_size::ExactSizeIterator::len
  %0 = call i64 @_ZN4core4iter6traits10exact_size17ExactSizeIterator3len17ha1a88d1d8ab39c74E(%"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"* align 8 %_2)
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %0
}

; <alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define internal void @"_ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hb92d2045b8381d81E"(%"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"* align 8 %self) unnamed_addr #1 personality i32 (...)* @__CxxFrameHandler3 {
start:
  %guard = alloca i64*, align 8
  %0 = bitcast i64** %guard to %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"**
  store %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"* %self, %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"** %0, align 8
  %1 = bitcast i64** %guard to %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"**
  %_6 = load %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"*, %"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"** %1, align 8, !nonnull !3, !align !4, !noundef !3
; invoke alloc::vec::into_iter::IntoIter<T,A>::as_raw_mut_slice
  %2 = invoke { [0 x %"std::ffi::os_str::OsString"]*, i64 } @"_ZN5alloc3vec9into_iter21IntoIter$LT$T$C$A$GT$16as_raw_mut_slice17h50051ddb97248dd5E"(%"alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>"* align 8 %_6)
          to label %bb1 unwind label %funclet_bb4

bb4:                                              ; preds = %funclet_bb4
; call core::ptr::drop_in_place<<alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop::DropGuard<std::ffi::os_str::OsString,alloc::alloc::Global>>
  call void @"_ZN4core3ptr180drop_in_place$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$std..ffi..os_str..OsString$C$alloc..alloc..Global$GT$$GT$17hed087f7f6a8908f6E"(i64** %guard) #13 [ "funclet"(token %cleanuppad) ]
  br label %bb5

funclet_bb4:                                      ; preds = %bb1, %start
  %cleanuppad = cleanuppad within none []
  br label %bb4

bb1:                                              ; preds = %start
  %_5.0 = extractvalue { [0 x %"std::ffi::os_str::OsString"]*, i64 } %2, 0
  %_5.1 = extractvalue { [0 x %"std::ffi::os_str::OsString"]*, i64 } %2, 1
; invoke core::ptr::drop_in_place<[std::ffi::os_str::OsString]>
  invoke void @"_ZN4core3ptr57drop_in_place$LT$$u5b$std..ffi..os_str..OsString$u5d$$GT$17h1515d6f4ee4cd1d6E"([0 x %"std::ffi::os_str::OsString"]* %_5.0, i64 %_5.1)
          to label %bb2 unwind label %funclet_bb4

bb2:                                              ; preds = %bb1
; call core::ptr::drop_in_place<<alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop::DropGuard<std::ffi::os_str::OsString,alloc::alloc::Global>>
  call void @"_ZN4core3ptr180drop_in_place$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$std..ffi..os_str..OsString$C$alloc..alloc..Global$GT$$GT$17hed087f7f6a8908f6E"(i64** %guard)
  br label %bb3

bb5:                                              ; preds = %bb4
  cleanupret from %cleanuppad unwind to caller

bb3:                                              ; preds = %bb2
  ret void
}

; test::main
; Function Attrs: uwtable
define internal void @_ZN4test4main17h4334b6ba00ae6a3eE() unnamed_addr #1 personality i32 (...)* @__CxxFrameHandler3 {
start:
  %_5 = alloca %"std::env::Args", align 8
  %d = alloca i32, align 4
; call std::env::args
  call void @_ZN3std3env4args17ha755d079c1c702a0E(%"std::env::Args"* sret(%"std::env::Args") %_5)
  br label %bb1

bb1:                                              ; preds = %start
; invoke <std::env::Args as core::iter::traits::exact_size::ExactSizeIterator>::len
  %_3 = invoke i64 @"_ZN84_$LT$std..env..Args$u20$as$u20$core..iter..traits..exact_size..ExactSizeIterator$GT$3len17h0da2416253d8fe06E"(%"std::env::Args"* align 8 %_5)
          to label %bb2 unwind label %funclet_bb7

bb7:                                              ; preds = %funclet_bb7
; call core::ptr::drop_in_place<std::env::Args>
  call void @"_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17h8bb3a0fa3e904967E"(%"std::env::Args"* %_5) #13 [ "funclet"(token %cleanuppad) ]
  br label %bb8

funclet_bb7:                                      ; preds = %bb1
  %cleanuppad = cleanuppad within none []
  br label %bb7

bb2:                                              ; preds = %bb1
  %_2 = icmp eq i64 %_3, 2
; call core::ptr::drop_in_place<std::env::Args>
  call void @"_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17h8bb3a0fa3e904967E"(%"std::env::Args"* %_5)
  br label %bb3

bb8:                                              ; preds = %bb7
  cleanupret from %cleanuppad unwind to caller

bb3:                                              ; preds = %bb2
  br i1 %_2, label %bb4, label %bb5

bb5:                                              ; preds = %bb3
  store i32 6, i32* %d, align 4
  br label %bb6

bb4:                                              ; preds = %bb3
  store i32 5, i32* %d, align 4
  br label %bb6

bb6:                                              ; preds = %bb5, %bb4
  ret void
}

declare i32 @__CxxFrameHandler3(...) unnamed_addr #5

; std::rt::lang_start_internal
; Function Attrs: uwtable
declare i64 @_ZN3std2rt19lang_start_internal17hafda6cb4b2d28b93E({}* align 1, [3 x i64]* align 8, i64, i8**) unnamed_addr #1

; core::fmt::Formatter::debug_lower_hex
; Function Attrs: uwtable
declare zeroext i1 @_ZN4core3fmt9Formatter15debug_lower_hex17h705fc45d79f414f0E(%"core::fmt::Formatter"* align 8) unnamed_addr #1

; core::fmt::num::<impl core::fmt::LowerHex for usize>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN4core3fmt3num55_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$usize$GT$3fmt17h9eb247cc005c0d76E"(i64* align 8, %"core::fmt::Formatter"* align 8) unnamed_addr #1

; core::fmt::Formatter::debug_upper_hex
; Function Attrs: uwtable
declare zeroext i1 @_ZN4core3fmt9Formatter15debug_upper_hex17h8b753905378ff302E(%"core::fmt::Formatter"* align 8) unnamed_addr #1

; core::fmt::num::<impl core::fmt::UpperHex for usize>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN4core3fmt3num55_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$usize$GT$3fmt17h946be138df9b7392E"(i64* align 8, %"core::fmt::Formatter"* align 8) unnamed_addr #1

; core::fmt::num::imp::<impl core::fmt::Display for usize>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17hc8b6f8f88d6483acE"(i64* align 8, %"core::fmt::Formatter"* align 8) unnamed_addr #1

; Function Attrs: nofree nosync nounwind readnone willreturn
declare i1 @llvm.expect.i1(i1, i1) #6

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64) #7

; Function Attrs: argmemonly nofree nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #8

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17h529b7d74f0f4c3e5E([0 x i8]* align 1, i64, %"core::panic::location::Location"* align 8) unnamed_addr #9

; core::panicking::assert_failed_inner
; Function Attrs: noreturn uwtable
declare void @_ZN4core9panicking19assert_failed_inner17hf17b0c5cd84309eaE(i8, {}* align 1, [3 x i64]* align 8, {}* align 1, [3 x i64]* align 8, %"core::option::Option<core::fmt::Arguments>"*, %"core::panic::location::Location"* align 8) unnamed_addr #10

; Function Attrs: inaccessiblememonly nofree nosync nounwind willreturn
declare void @llvm.assume(i1 noundef) #11

; Function Attrs: nounwind uwtable
declare void @__rust_dealloc(i8*, i64, i64) unnamed_addr #12

; core::fmt::Formatter::debug_tuple
; Function Attrs: uwtable
declare void @_ZN4core3fmt9Formatter11debug_tuple17h6577318650ca455bE(%"core::fmt::builders::DebugTuple"* sret(%"core::fmt::builders::DebugTuple"), %"core::fmt::Formatter"* align 8, [0 x i8]* align 1, i64) unnamed_addr #1

; core::fmt::builders::DebugTuple::field
; Function Attrs: uwtable
declare align 8 %"core::fmt::builders::DebugTuple"* @_ZN4core3fmt8builders10DebugTuple5field17h206c25a3d0cf074eE(%"core::fmt::builders::DebugTuple"* align 8, {}* align 1, [3 x i64]* align 8) unnamed_addr #1

; core::fmt::builders::DebugTuple::finish
; Function Attrs: uwtable
declare zeroext i1 @_ZN4core3fmt8builders10DebugTuple6finish17h739a8d7c87c0d37cE(%"core::fmt::builders::DebugTuple"* align 8) unnamed_addr #1

; core::fmt::Formatter::write_str
; Function Attrs: uwtable
declare zeroext i1 @_ZN4core3fmt9Formatter9write_str17h167c82fdf2572f1fE(%"core::fmt::Formatter"* align 8, [0 x i8]* align 1, i64) unnamed_addr #1

; std::env::args
; Function Attrs: uwtable
declare void @_ZN3std3env4args17ha755d079c1c702a0E(%"std::env::Args"* sret(%"std::env::Args")) unnamed_addr #1

; <std::env::Args as core::iter::traits::exact_size::ExactSizeIterator>::len
; Function Attrs: uwtable
declare i64 @"_ZN84_$LT$std..env..Args$u20$as$u20$core..iter..traits..exact_size..ExactSizeIterator$GT$3len17h0da2416253d8fe06E"(%"std::env::Args"* align 8) unnamed_addr #1

define i32 @main(i32 %0, i8** %1) unnamed_addr #5 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start
  %3 = call i64 @_ZN3std2rt10lang_start17h8acd9567097365c6E(void ()* @_ZN4test4main17h4334b6ba00ae6a3eE, i64 %2, i8** %1)
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

attributes #0 = { inlinehint uwtable "target-cpu"="x86-64" }
attributes #1 = { uwtable "target-cpu"="x86-64" }
attributes #2 = { noinline uwtable "target-cpu"="x86-64" }
attributes #3 = { inlinehint noreturn uwtable "target-cpu"="x86-64" }
attributes #4 = { cold noreturn uwtable "target-cpu"="x86-64" }
attributes #5 = { "target-cpu"="x86-64" }
attributes #6 = { nofree nosync nounwind readnone willreturn }
attributes #7 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #8 = { argmemonly nofree nounwind willreturn }
attributes #9 = { cold noinline noreturn uwtable "target-cpu"="x86-64" }
attributes #10 = { noreturn uwtable "target-cpu"="x86-64" }
attributes #11 = { inaccessiblememonly nofree nosync nounwind willreturn }
attributes #12 = { nounwind uwtable "target-cpu"="x86-64" }
attributes #13 = { noinline }
attributes #14 = { noreturn }
attributes #15 = { nounwind }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{i64 0, i64 2}
!3 = !{}
!4 = !{i64 8}
!5 = !{i8 0, i8 2}
!6 = !{i64 1, i64 0}
!7 = !{i32 3185505}
