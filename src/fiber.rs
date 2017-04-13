use types::{Value, Argc, c_void};

extern "C" {
    pub fn rb_fiber_new(function: extern "C" fn(*mut c_void) -> Value,
                            data: *mut c_void) -> Value;
    pub fn rb_fiber_resume(fiber: Value, argc: Argc, argv: *const Value) -> Value;
}
