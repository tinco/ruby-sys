use types::{Value, Argc, c_void};

extern "C" {
    pub fn rb_fiber_new(function: extern "C" fn(arg: Value, box_ptr: *mut c_void, argc: Argc, argv: *const Value, blockarg: Value) -> Value,
                            data: *mut c_void) -> Value;
    pub fn rb_fiber_resume(fiber: Value, argc: Argc, argv: *const Value) -> Value;
    pub fn rb_fiber_yield(argc: Argc, argv: *const Value) -> Value;
}
