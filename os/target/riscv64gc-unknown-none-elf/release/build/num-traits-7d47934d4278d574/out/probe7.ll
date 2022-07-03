; ModuleID = 'probe7.7fc417ed-cgu.0'
source_filename = "probe7.7fc417ed-cgu.0"
target datalayout = "e-m:e-p:64:64-i64:64-i128:128-n64-S128"
target triple = "riscv64"

%"core::panic::location::Location" = type { { [0 x i8]*, i64 }, i32, i32 }

@alloc1 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/1f7fb6413d6d6c0c929b223e478e44c3db991b03/library/core/src/num/mod.rs" }>, align 1
@alloc2 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [75 x i8] }>, <{ [75 x i8] }>* @alloc1, i32 0, i32 0, i32 0), [16 x i8] c"K\00\00\00\00\00\00\00J\03\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe7::probe
; Function Attrs: nounwind
define dso_local void @_ZN6probe75probe17h32a8b83e4c2a537dE() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false) #3
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h6373a349ddfc4fc2E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h42c2f306c9307685E([0 x i8]* align 1 bitcast ([25 x i8]* @str.0 to [0 x i8]*), i64 25, %"core::panic::location::Location"* align 8 bitcast (<{ i8*, [16 x i8] }>* @alloc2 to %"core::panic::location::Location"*)) #4
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h6373a349ddfc4fc2E.exit": ; preds = %start
  br label %bb1

bb1:                                              ; preds = %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h6373a349ddfc4fc2E.exit"
  ret void
}

; Function Attrs: nofree nosync nounwind readnone willreturn
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn nounwind
declare dso_local void @_ZN4core9panicking5panic17h42c2f306c9307685E([0 x i8]* align 1, i64, %"core::panic::location::Location"* align 8) unnamed_addr #2

attributes #0 = { nounwind "frame-pointer"="all" "target-cpu"="generic-rv64" "target-features"="+m,+a,+f,+d,+c" }
attributes #1 = { nofree nosync nounwind readnone willreturn }
attributes #2 = { cold noinline noreturn nounwind "frame-pointer"="all" "target-cpu"="generic-rv64" "target-features"="+m,+a,+f,+d,+c" }
attributes #3 = { nounwind }
attributes #4 = { noreturn nounwind }

!llvm.module.flags = !{!0}

!0 = !{i32 1, !"Code Model", i32 3}
