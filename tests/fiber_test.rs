extern crate ruby_sys;
use ruby_sys::{vm, value, fixnum, fiber, util, types};

static QNIL : value::Value = value::Value { value: value::RubySpecialConsts::Nil as usize };

#[test]
fn rb_creating_and_running_fibers_works() {
    unsafe { vm::ruby_init() };

    extern fn test_fn() -> value::Value { unsafe { fixnum::rb_int2inum(3) } }

    fn run_test() {
        unsafe {
            let fiber = fiber::rb_fiber_new(test_fn, QNIL);
            let result = fiber::rb_fiber_resume(fiber, 0, vec![].as_ptr());
            assert!(!result.is_nil());
            let val_result = fixnum::rb_num2int(result);
            assert!(3 == val_result);
        }
    }

    unsafe {
        let run_test_box = util::closure_box_ptr_create(run_test);
        let mut state = 0;
        vm::rb_protect(util::rbsys_closure_box_ptr_void as types::CallbackPtr, run_test_box, &mut state as *mut types::c_int);
        vm::ruby_cleanup(state)
    };
}
