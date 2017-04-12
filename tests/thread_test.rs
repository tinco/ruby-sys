extern crate ruby_sys;
use ruby_sys::{vm,value, fixnum, thread, util};

#[test]
fn rb_creating_and_running_threads_works() {
    unsafe { vm::ruby_init() };

    fn test_fn() -> value::Value { unsafe { fixnum::rb_int2inum(3) } }

    let box_ptr = util::closure_box_ptr_create(test_fn);

    unsafe {
        thread::rb_thread_create(util::rbsys_closure_box_ptr_value, box_ptr);
    }

    unsafe { vm::ruby_cleanup(0) };
}
