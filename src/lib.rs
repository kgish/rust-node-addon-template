// Raw bindings to the Node.js API
use nodejs_sys::{
    napi_async_work, napi_callback_info, napi_create_async_work, napi_create_double,
    napi_create_error, napi_create_function, napi_create_int64, napi_create_promise,
    napi_create_string_utf8, napi_deferred, napi_delete_async_work, napi_env, napi_get_cb_info,
    napi_get_undefined, napi_get_value_double, napi_get_value_int64, napi_get_value_string_utf8,
    napi_queue_async_work, napi_reject_deferred, napi_resolve_deferred, napi_set_named_property,
    napi_status, napi_value,
};

// FFI bindings

// std::ffi::CString - A type representing an owned, C-compatible, nul-terminated string with no
// nul bytes in the middle.
// This type serves the purpose of being able to safely generate a C-compatible string from a Rust
// byte slice or vector. An instance of this type is a static guarantee that the underlying bytes
// contain no interior 0 bytes ("nul characters") and that the final byte is 0 ("nul terminator").
// See: https://doc.rust-lang.org/std/ffi/struct.CString.html
use std::ffi::CString;

// std::ffi::c_void - Equivalent to C's void type when used as a pointer.
// In essence, *const c_void is equivalent to C's const void* and *mut c_void is equivalent to
// C's void*. That said, this is not the same as C's void return type, which is Rust's () type.
// See: https://doc.rust-lang.org/std/ffi/enum.c_void.html
use std::ffi::c_void;

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
//   napi_create_double: https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_create_double.html
//   napi_create_error: https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_create_error.html
//   napi_create_function: https://docs.rs/nodejs-sys/0.2.0/nodejs_sys/fn.napi_create_function.html
//   napi_create_int64: https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_create_int64.html
//   napi_create_string_utf8: https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_create_string_utf8.html
//   napi_get_cb_info: https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_get_cb_info.html
//   napi_get_undefined: https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_get_undefined.html
//   napi_get_value_double: https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_get_value_double.html
//   napi_get_value_int64: https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_get_value_int64.html
//   napi_get_value_string_utf8: https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_get_value_string_utf8.html
//   napi_set_named_property: https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_set_named_property.html
//   napi_status: https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/enum.napi_status.html

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

    napi_create_string_utf8(env, str_ptr(&val1), str_len(&val1), &mut result1);

    napi_set_named_property(env, exports, str_ptr(&key1), result1);

    // --- 2. Create a function: sayHello() => string --- //

    let str2 = CString::new("sayHello").expect("CString::new failed");
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

    // --- 3. Create a function passing numbers addNumbers() -> double --- //

    let str3 = CString::new("addNumbers").expect("CString::new failed");
    let mut result3: napi_value = std::mem::zeroed();

    napi_create_function(
        env,
        str_ptr(&str3),
        str_len(&str3),
        Some(add_numbers),
        std::ptr::null_mut(),
        &mut result3,
    );

    napi_set_named_property(env, exports, str_ptr(&str3), result3);

    // --- 4. Create a function passing string: sendMessage(message) -> () --- //

    let str4 = CString::new("sendMessage").expect("CString::new failed");
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

    // --- 5. Create an async function fibonacci(n) -> u64 --- //

    let str5 = CString::new("fibonacci").expect("CString::new failed");
    let mut result5: napi_value = std::mem::zeroed();

    napi_create_function(
        env,
        str_ptr(&str5),
        str_len(&str5),
        Some(fibonacci),
        std::ptr::null_mut(),
        &mut result5,
    );

    napi_set_named_property(env, exports, str_ptr(&str5), result5);

    exports
}

// --- Public --- //

pub unsafe extern "C" fn say_hello(env: napi_env, _info: napi_callback_info) -> napi_value {
    let mut result: napi_value = std::mem::zeroed();
    let str = CString::new("Hello from the kingdom of Rust!").expect("CString::new failed");

    napi_create_string_utf8(env, str_ptr(&str), str_len(&str), &mut result);

    println!("lib.rs: say_hello() => {:?} ({:?})", str, result);

    result
}

pub unsafe extern "C" fn add_numbers(env: napi_env, info: napi_callback_info) -> napi_value {
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

    napi_get_value_string_utf8(env, message[0], std::ptr::null_mut(), 0, &mut len);

    let size = len as usize;
    let mut vec: Vec<u8> = Vec::with_capacity(size + 1);
    let raw = vec.as_mut_ptr();
    std::mem::forget(vec);
    let mut capacity = 0;

    let _s = napi_get_value_string_utf8(env, message[0], raw as *mut i8, size + 1, &mut capacity);

    let s = String::from_raw_parts(raw, capacity as usize, size);

    println!("lib.rs: send_message({})", s);

    let mut und: napi_value = std::mem::zeroed();
    napi_get_undefined(env, &mut und);

    und
}

#[derive(Debug, Clone)]
struct Data {
    deferred: napi_deferred,
    work: napi_async_work,
    val: u64,
    result: Option<Result<u64, String>>,
}

pub unsafe extern "C" fn fibonacci(env: napi_env, info: napi_callback_info) -> napi_value {
    let mut buffer: Vec<napi_value> = Vec::with_capacity(1);
    let p = buffer.as_mut_ptr();
    let mut argc = 1 as usize;
    std::mem::forget(buffer);

    napi_get_cb_info(
        env,
        info,
        &mut argc,
        p,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );

    let mut start = 0;
    napi_get_value_int64(env, *p, &mut start);

    let mut promise: napi_value = std::mem::zeroed();
    let mut deferred: napi_deferred = std::mem::zeroed();
    let mut work_name: napi_value = std::mem::zeroed();
    let mut work: napi_async_work = std::mem::zeroed();

    let async_name = CString::new("async fibonacci").expect("Error creating string");
    napi_create_string_utf8(
        env,
        str_ptr(&async_name),
        str_len(&async_name),
        &mut work_name,
    );
    napi_create_promise(env, &mut deferred, &mut promise);

    let v = Data {
        deferred,
        work,
        val: start as u64,
        result: None,
    };

    let data = Box::new(v);
    let raw = Box::into_raw(data);

    napi_create_async_work(
        env,
        std::ptr::null_mut(),
        work_name,
        Some(perform),
        Some(complete),
        std::mem::transmute(raw),
        &mut work,
    );

    napi_queue_async_work(env, work);
    (*raw).work = work;

    println!("lib.rs: fibonacci({})", start);

    promise
}

pub unsafe extern "C" fn perform(_env: napi_env, data: *mut c_void) {
    let mut t: Box<Data> = Box::from_raw(std::mem::transmute(data));
    let mut last = 1;
    let mut second_last = 0;

    for _ in 2..t.val {
        let temp = last;
        last = last + second_last;
        second_last = temp;
    }

    t.result = Some(Ok(last));

    println!(
        "lib.rs: perform() val='{:?}' result='{:?}'",
        t.val, t.result
    );

    Box::into_raw(t);
}

pub unsafe extern "C" fn complete(env: napi_env, _status: napi_status, data: *mut c_void) {
    let t: Box<Data> = Box::from_raw(std::mem::transmute(data));
    let v = match t.result {
        Some(d) => match d {
            Ok(result) => {
                println!("lib.rs: complete() result='{:?}'", result);
                result
            },
            Err(_) => {
                let mut js_error: napi_value = std::mem::zeroed();
                napi_create_error(
                    env,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    &mut js_error,
                );
                napi_reject_deferred(env, t.deferred, js_error);
                napi_delete_async_work(env, t.work);
                return;
            }
        },
        None => {
            let mut js_error: napi_value = std::mem::zeroed();
            napi_create_error(
                env,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut js_error,
            );
            napi_reject_deferred(env, t.deferred, js_error);
            napi_delete_async_work(env, t.work);
            return;
        }
    };
    let mut obj: napi_value = std::mem::zeroed();
    napi_create_int64(env, v as i64, &mut obj);
    napi_resolve_deferred(env, t.deferred, obj);

    napi_delete_async_work(env, t.work);
}

// --- Private --- //

fn str_ptr(s: &CString) -> *const i8 {
    s.as_ptr()
}

fn str_len(s: &CString) -> usize {
    s.as_bytes().len()
}
