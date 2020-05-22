; ModuleID = 'example_module'
source_filename = "example_module"

define i64 @main() {
entry:
  %a = alloca i64
  store i64 10, i64* %a
  store i64 100, i64* %a
  ret i64 0
}

declare i64* @printf(...)
