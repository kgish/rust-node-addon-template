// Raw bindings to the Node.js API
use nodejs_sys::{
    napi_callback_info,
    napi_create_double,
    napi_create_function,
    napi_create_string_utf8,
    napi_env,
    napi_get_cb_info,
    napi_get_undefined,
    napi_get_value_double,
    napi_get_value_string_utf8,
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
//   napi_get_undefined: https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_get_undefined.html
//   napi_get_value_string_utf8: https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_get_value_string_utf8.html

// --- Register module --- //

#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(
    env: napi_env,
    exports: napi_value,
) -> nodejs_sys::napi_value {

    println!("lib.rs: napi_register_module_v1()");

    // --- 1. Create an object -> { key: value } --- //

    let key1 = CString::new("name").expect("CString::new failed");
    let val1 = CString::new("Kiffin Gish").expect("CString::new failed");
    let mut result1: napi_value = std::mem::zeroed();

    napi_create_string_utf8(
        env,
        str_ptr(&val1),
        str_len(&val1),
        &mut result1,
    );

    napi_set_named_property(
        env,
        exports,
        str_ptr(&key1),
        result1,
    );

    // --- 2. Create a function: say_hello() => string --- //

    let str2 = CString::new("say_hello").expect("CString::new failed");
    let mut result2: napi_value = std::mem::zeroed();

    napi_create_function(
        env,
        str_ptr(&str2),
        str_len(&str2),
        Some(say_hello),
        std::ptr::null_mut(),
        &mut result2,
    );

    napi_set_named_property(env, exports, str_ptr(&str2), result2);

    // --- 3. Create a function passing doubles: add_doubles() -> double --- //

    let str3 = CString::new("add_doubles").expect("CString::new failed");
    let mut result3: napi_value = std::mem::zeroed();

    napi_create_function(
        env,
        str_ptr(&str3),
        str_len(&str3),
        Some(add_doubles),
        std::ptr::null_mut(),
        &mut result3,
    );

    napi_set_named_property(env, exports, str_ptr(&str3), result3);

    // --- 4. Create a function passing string: send_message(message) -> () --- //

    let str4 = CString::new("send_message").expect("CString::new failed");
    let mut result4: napi_value = std::mem::zeroed();

    napi_create_function(
        env,
        str_ptr(&str4),
        str_len(&str4),
        Some(send_message),
        std::ptr::null_mut(),
        &mut result4,
    );

    napi_set_named_property(env, exports, str_ptr(&str4), result4);

    exports
}

// --- Public --- //

#[no_mangle]
pub unsafe extern "C" fn say_hello(env: napi_env, _info: napi_callback_info) -> napi_value {

    let mut result: napi_value = std::mem::zeroed();
    let str = CString::new("Hello from the kingdom of Rust!").expect("CString::new failed");

    napi_create_string_utf8(
        env,
        str_ptr(&str),
        str_len(&str),
        &mut result,
    );

    println!("lib.rs: say_hello() => {:?} ({:?})", str, result);

    result
}

#[no_mangle]
pub unsafe extern "C" fn add_doubles(env: napi_env, info: napi_callback_info) -> napi_value {
    // Extract the initialized data -- this is only allowed *after* properly initializing `buffer`
    let mut buffer: [napi_value; 2] = std::mem::MaybeUninit::zeroed().assume_init();

    let mut argc = 2 as usize;
    let mut result: napi_value = std::mem::zeroed();

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

    let value = x + y;

    println!("lib.rs: add_doubles({},{}) => {:?}", x, y, value);

    napi_create_double(env, value, &mut result);

    result
}

pub unsafe extern "C" fn send_message(env: napi_env, info: napi_callback_info) -> napi_value {
    // Extract the initialized data -- this is only allowed *after* properly initializing `buffer`
    let mut message: [napi_value; 1] = std::mem::MaybeUninit::zeroed().assume_init();
    let mut argc = 1 as usize;

    napi_get_cb_info(
        env,
        info,
        &mut argc,
        message.as_mut_ptr(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );

    let mut len = 0;

    napi_get_value_string_utf8(
        env,
        message[0],
        std::ptr::null_mut(),
        0,
        &mut len
    );

    let size = len as usize;
    let mut vec: Vec<u8> = Vec::with_capacity(size + 1);
    let raw = vec.as_mut_ptr();
    std::mem::forget(vec);
    let mut capacity = 0;

    let _s = napi_get_value_string_utf8(
        env,
        message[0],
        raw as *mut i8,
        size + 1,
        &mut capacity
    );

    let s = String::from_raw_parts(raw, capacity as usize, size);

    println!("lib.rs: send_message({})", s);

    // Function
    let mut und: napi_value = std::mem::zeroed();
    napi_get_undefined(env, &mut und);

    und
}

// --- Private --- //

fn str_ptr(s: &CString) -> *const i8 {
    s.as_ptr()
}

fn str_len(s: &CString) -> usize {
    s.as_bytes().len()
}
