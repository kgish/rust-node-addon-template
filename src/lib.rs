// Raw bindings to the Node.js API
use nodejs_sys::{
    napi_callback_info,
    napi_create_double,
    napi_create_function,
    napi_create_string_utf8,
    napi_env,
    napi_get_cb_info,
    napi_get_value_double,
    napi_set_named_property,
    napi_value,
};

// FFI bindings
use std::ffi::CString;

// #[no_mangle] : The Rust compiler mangles symbol names differently than native code linkers
// expect and therefore needs to be told NOT to mangle any functions exported to the outside world.
//
// extern "C" : By default, any function you write in Rust will use the Rust ABI. Instead, when
// building outwards facing FFI APIs we need to tell the compiler to use the system ABI.

// std::ffi: https://doc.rust-lang.org/std/ffi/index.html
// Types:
//   CString: https://doc.rust-lang.org/std/ffi/struct.CString.html

// nodejs_sys: https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/
// Types:
//   napi_env: https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/type.napi_env.html
//   napi_value: https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/type.napi_value.html
// Functions:
//   napi_create_string_utf8: https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_create_string_utf8.html
//   napi_set_named_property: https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_set_named_property.html
//   napi_create_function: https://docs.rs/nodejs-sys/0.2.0/nodejs_sys/fn.napi_create_function.html
//   napi_get_cb_info: https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_get_cb_info.html
//   napi_get_value_double: https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_get_value_double.html
//   napi_create_double: https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_create_double.html

#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(
    env: napi_env,
    exports: napi_value,
) -> nodejs_sys::napi_value {

    println!("lib.rs: napi_register_module_v1()");

    let key = CString::new("hello").expect("CString::new failed");
    let value = CString::new("world!").expect("CString::new failed");
    let mut local: napi_value = std::mem::zeroed();

    napi_create_string_utf8(
        env,
        value.as_ptr(),
        str_len(&value),
        &mut local,
    );

    napi_set_named_property(
        env,
        exports,
        key.as_ptr(),
        local,
    );

    let p = CString::new("say_hello").expect("CString::new failed");
    // let mut local: napi_value = std::mem::zeroed();

    napi_create_function(
        env,
        p.as_ptr(),
        str_len(&p),
        Some(say_hello),
        std::ptr::null_mut(),
        &mut local,
    );

    napi_set_named_property(env, exports, p.as_ptr(), local);

    let p = CString::new("add").expect("CString::new failed");
    // let mut local: napi_value = std::mem::zeroed();

    napi_create_function(
        env,
        p.as_ptr(),
        str_len(&p),
        Some(add),
        std::ptr::null_mut(),
        &mut local,
    );

    napi_set_named_property(env, exports, p.as_ptr(), local);

    exports
}

#[no_mangle]
pub unsafe extern "C" fn say_hello(env: napi_env, _info: napi_callback_info) -> napi_value {
    println!("lib.rs: say_hello()");

    let mut local: napi_value = std::mem::zeroed();
    let p = CString::new("Hello from rust!").expect("CString::new failed");

    napi_create_string_utf8(
        env,
        p.as_ptr(),
        str_len(&p),
        &mut local,
    );

    local
}

#[no_mangle]
pub unsafe extern "C" fn add(env: napi_env, info: napi_callback_info) -> napi_value {

    let mut buffer: [napi_value; 2] = std::mem::MaybeUninit::zeroed().assume_init();
    let mut argc = 2 as usize;
    let mut local: napi_value = std::mem::zeroed();

    napi_get_cb_info(
        env,
        info,
        &mut argc,
        buffer.as_mut_ptr(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );

    let mut x = 0 as f64;
    let mut y = 0 as f64;

    napi_get_value_double(env, buffer[0], &mut x);
    napi_get_value_double(env, buffer[1], &mut y);

    println!("lib.rs: add({},{})", x, y);

    napi_create_double(env, x + y, &mut local);

    local
}

fn str_len(s: &CString) -> usize {
    s.as_bytes().len()
}
