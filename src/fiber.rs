use types::{Value, Argc};

extern "C" {
    pub fn rb_fiber_new(func: extern fn() -> Value, binding: Value) -> Value;
    pub fn rb_fiber_resume(fiber: Value, argc: Argc, argv: *const Value) -> Value;
}
