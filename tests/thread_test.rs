extern crate ruby_sys;
use ruby_sys::{vm,value, fixnum, thread, rproc, util};
use std::mem;

#[test]
fn rb_creating_and_running_threads_works() {
    unsafe { vm::ruby_init() };

    let nil = value::Value { value: value::RubySpecialConsts::Nil as usize };

    fn test_fn() -> value::Value { unsafe { fixnum::rb_int2inum(3) } }

    let box_ptr = util::closure_box_ptr_create(test_fn);

    unsafe {
        let fiber = thread::rb_thread_create(util::rbsys_closure_box_ptr_call, box_ptr);
        //let result = fiber::rb_fiber_resume(fiber, 0, vec![].as_ptr());
        //assert!(!result.is_nil());
        //let val_result = fixnum::rb_num2int(result);
        //assert!(3 == val_result);
    }

    unsafe { vm::ruby_cleanup(0) };
}
