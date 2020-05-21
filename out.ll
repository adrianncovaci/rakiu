; ModuleID = 'example_module'
source_filename = "example_module"

define i64 @main() {
entry:
  %a = alloca i64
  store i64 100, i64* %a
  %0 = call i64* (...) @printf(i64 986498496)
  %1 = call i64* (...) @printf(i64 986499495)
  %2 = call i64* (...) @printf(i64 0)
  %3 = call i64* (...) @printf(i64 986499495)
  %4 = call i64* (...) @printf(i64 986499595)
  %5 = call i64* (...) @printf(i64 984984)
  %6 = call i64* (...) @printf(i64 994983)
  %7 = call i64* (...) @printf(i64 0)
  %8 = call i64* (...) @printf(i64 994983)
  %9 = call i64* (...) @printf(i64 995083)
  ret i64 995083
}

declare i64* @printf(...)
