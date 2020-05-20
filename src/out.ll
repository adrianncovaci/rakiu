; ModuleID = 'example_module'
source_filename = "example_module"

define i64 @main() {
entry:
  %b = alloca i64
  %a = alloca i64
  store i64 2, i64* %a
  %0 = call i64* (...) @printf(i64 3)
  store i64 2000, i64* %b
  br i1 false, label %entry1, label %entry2

entry1:                                           ; preds = %entry
  store i64 200, i64* %a
  br label %entry3

entry2:                                           ; preds = %entry
  store i64 100, i64* %b
  br label %entry3

entry3:                                           ; preds = %entry2, %entry1
  %iftmp = phi i64 [ 200, %entry1 ], [ 100, %entry2 ]
  %1 = call i64* (...) @printf(i64 %iftmp)
  ret i64 %iftmp
}

declare i64* @printf(...)
