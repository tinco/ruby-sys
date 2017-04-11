extern crate ruby_sys;
use ruby_sys::{vm,rproc, value, fixnum, fiber};

#[test]
fn rb_creating_and_running_fibers_works() {
    unsafe { vm::ruby_init() };

    let nil = value::Value { value: value::RubySpecialConsts::Nil as usize };

    extern fn test_fn() -> value::Value { unsafe { fixnum::rb_int2inum(3) } }

    unsafe {
        let fiber = fiber::rb_fiber_new(test_fn, nil);
        let result = fiber::rb_fiber_resume(fiber, 0, vec![].as_ptr());
        assert!(!result.is_nil());
        let val_result = fixnum::rb_num2int(result);
        assert!(3 == val_result);
    }

    unsafe { vm::ruby_cleanup(0) };
}
