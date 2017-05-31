extern crate ruby_sys;
use ruby_sys::{vm,rproc, value, fixnum, util, types};
use ruby_sys::types::{Argc, c_char, Id, Value, c_void};


#[test]
fn rb_proc_new_works_with_closure() {
    unsafe { vm::ruby_init() };

    let nil = value::Value { value: value::RubySpecialConsts::Nil as usize };

    let run_test = || {
        unsafe {
            let val = fixnum::rb_int2inum(3);
            let test_fn = |arg: Value, argc: Argc, argv: *const Value, blockarg: Value| { unsafe { fixnum::rb_int2inum(3) } };
            let test_fn_box = util::block_box_ptr_create(test_fn);
            let rproc = rproc::rb_proc_new(util::rbsys_block_box_ptr_value, test_fn_box);
            let result = rproc::rb_proc_call_with_block(rproc, 1, vec![val].as_ptr(), nil);
            assert!(!result.is_nil());
            let val_result = fixnum::rb_num2int(result);
            assert!(3 == val_result);
        }
    };

    unsafe {
        let run_test_box = util::closure_box_ptr_create(run_test);
        let mut state = 0;
        vm::rb_protect(util::rbsys_closure_box_ptr_void as types::CallbackPtr, run_test_box, &mut state as *mut types::c_int);
        vm::ruby_cleanup(state)
    };
}
