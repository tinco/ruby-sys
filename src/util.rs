use types::{Argc, c_char, Id, Value, c_void};

extern "C" {
    pub fn rb_const_get(klass: Value, id: Id) -> Value;
    pub fn rb_funcallv(receiver: Value, method: Id, argc: Argc, argv: *const Value) -> Value;
    pub fn rb_intern(name: *const c_char) -> Id;
}

// This function can be used to create a box around Rust closures so they can be used
// to pass into Ruby methods.
pub fn closure_box_ptr_create<F,R>(func: F) -> *mut c_void
    where F: FnOnce() -> R,
{
    let closure_box = Box::new(func) as Box<FnOnce() -> R>;
    Box::into_raw(Box::new(closure_box)) as *mut c_void
}

// This function is used to unwrap a closure_box returned by closure_box_ptr_create
// and execute its wrapped closure, returning a Value.
pub extern "C" fn rbsys_closure_box_ptr_value(box_ptr: *mut c_void) -> Value
{
    type FnBox = Box<FnMut() -> Value>;
    let mut closure_box: Box<FnBox> = unsafe { Box::from_raw(box_ptr as *mut FnBox) };
    closure_box()
}

// This function is used to unwrap a closure_box returned by closure_box_ptr_create
// and execute its wrapped closure, returning a *const c_void.
pub extern "C" fn rbsys_closure_box_ptr_void(box_ptr: *mut c_void) -> *const c_void {
    type FnBox = Box<FnMut() -> *const c_void>;
    let mut closure_box: Box<FnBox> = unsafe { Box::from_raw(box_ptr as *mut FnBox) };
    closure_box()
}
