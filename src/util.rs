use types::{Argc, c_char, Id, Value, c_void};

extern "C" {
    pub fn rb_const_get(klass: Value, id: Id) -> Value;
    pub fn rb_funcallv(receiver: Value, method: Id, argc: Argc, argv: *const Value) -> Value;
    pub fn rb_intern(name: *const c_char) -> Id;
}

pub fn closure_box_ptr_create<F>(func: F) -> *mut c_void
    where F: FnOnce() -> Value,
{
    let closure_box = Box::new(func) as Box<FnOnce() -> Value>;
    Box::into_raw(Box::new(closure_box)) as *mut c_void
}

pub extern "C" fn rbsys_closure_box_ptr_call(box_ptr: *mut c_void) -> Value
{
    type FnBox = Box<FnMut() -> Value>;
    let mut closure_box: Box<FnBox> = unsafe { Box::from_raw(box_ptr as *mut FnBox) };
    closure_box()
}
