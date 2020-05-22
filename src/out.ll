; ModuleID = 'example_module'
source_filename = "example_module"

define i64 @main() {
entry:
  %a = alloca i64
  store i64 10, i64* %a
  store i64 2, i64* %a
  %0 = call i64* (...) @printf(i64 5)
  %1 = call i64* (...) @printf(i64 4)
  ret i64 4
}

declare i64* @printf(...)
