(module
  (type (;0;) (func (param i32) (result i32)))
  (func (;0;) (type 0) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      i32.const 1
      i32.or
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      return
    end
    local.get 0
    i32.const -1
    i32.add
    call 0
    local.get 0
    i32.const -2
    i32.add
    call 0
    i32.add)
  (table (;0;) 0 funcref)
  (memory (;0;) 1)
  (export "memory" (memory 0))
  (export "fib" (func 0)))
