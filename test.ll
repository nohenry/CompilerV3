; ModuleID = 'test/test.dsl'
source_filename = "test/test.dsl"

define void @main() {
  br label %1

1:                                                ; preds = %0
  %2 = alloca i8, align 1
  br label %3

3:                                                ; preds = %1
  %4 = call i8 @value1(i8 0, i1 true)
  br label %8
}

define i8 @value1(i8 %0, i1 %1) {
  %3 = alloca i8, align 1
  %4 = alloca i1, align 1
  store i8 %0, i8* %3, align 1
  store i1 %1, i1* %4, align 1
  %5 = alloca i8, align 1
  %6 = load i8, i8* %3, align 1
  store i8 %6, i8* %5, align 1
  %7 = load i8, i8* %5, align 1
  ret i8 %7

8:                                                ; preds = %3
  store i8 %4, i8* %2, align 1
  ret void
}
