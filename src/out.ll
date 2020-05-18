; ModuleID = 'example_module'
source_filename = "example_module"

define i64 @main() {
entry:
  %a = alloca i64
  store i64 3, i64* %a
  ret i64 0
}
