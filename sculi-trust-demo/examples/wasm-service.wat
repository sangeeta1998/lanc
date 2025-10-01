(module
  (func $add (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.add)
  (export "add" (func $add))
  
  (func $trust_score (result f64)
    f64.const 0.85)
  (export "trust_score" (func $trust_score))
)
