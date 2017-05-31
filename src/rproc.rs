use types::{Argc, Value, c_void};

extern "C" {
    pub fn rb_proc_call_with_block(rproc: Value,
                                   argc: Argc,
                                   argv: *const Value,
                                   pass_procval: Value)
                                   -> Value;

    pub fn rb_proc_new(function: extern "C" fn(arg: Value, box_ptr: *mut c_void, argc: Argc, argv: *const Value, blockarg: Value) -> Value,
                            data: *mut c_void) -> Value;
}
