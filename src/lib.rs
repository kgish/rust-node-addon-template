use nodejs_sys::{
    napi_async_work, napi_callback_info, napi_create_async_work, napi_create_double,
    napi_create_error, napi_create_function, napi_create_int64, napi_create_object,
    napi_create_promise, napi_create_string_utf8, napi_create_uint32, napi_deferred,
    napi_delete_async_work, napi_env, napi_get_cb_info, napi_get_undefined, napi_get_value_double,
    napi_get_value_int64, napi_get_value_string_utf8, napi_queue_async_work, napi_reject_deferred,
    napi_resolve_deferred, napi_set_named_property, napi_status, napi_value,
};

// FFI bindings
use std::ffi::c_void;
use std::ffi::CString;

// --- Register module --- //

#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(
    env: napi_env,
    exports: napi_value,
) -> nodejs_sys::napi_value {
    println!("lib.rs: napi_register_module_v1()");

    create_function(env, exports, "sayHello", say_hello);
    create_function(env, exports, "sendMessage", send_message);
    create_function(env, exports, "addNumbers", add_numbers);
    create_function(env, exports, "getUser", get_user);
    create_function(env, exports, "fibonacci", fibonacci);

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

// TODO
pub unsafe extern "C" fn get_user(env: napi_env, _info: napi_callback_info) -> napi_value {
    let mut result: napi_value = std::mem::zeroed();
    // let mut key_name: napi_value = std::mem::zeroed();
    // let mut val_name: napi_value = std::mem::zeroed();
    // let mut key_age: napi_value = std::mem::zeroed();
    let mut val_age: napi_value = std::mem::zeroed();
    // let cstr_key_name = CString::new("name").expect("CString::new failed");
    // let cstr_val_name = CString::new("Kiffin").expect("CString::new failed");
    // let cstr_key_age = CString::new("age").expect("CString::new failed");

    napi_create_object(env, &mut result);

    // napi_create_string_utf8(env, str_ptr(&cstr_key_name), str_len(&cstr_key_name), &mut key_name);
    // napi_create_string_utf8(env, str_ptr(&cstr_val_name), str_len(&cstr_val_name), &mut val_name);
    // napi_create_string_utf8(env, str_ptr(&cstr_key_age), str_len(&cstr_key_age), &mut key_age);
    //
    napi_create_uint32(env, 36 as u32, &mut val_age);
    //
    // (*result).set(key_name, val_name);
    // (*result).set(key_age, val_age);

    println!("lib.rs: get_user() => ({:?})", result);

    result
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

    let n = fib(t.val);

    t.result = Some(Ok(n));

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
            }
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

    // TODO:
    // Note that for Node v12.16.3 Number.MAX_SAFE_INTEGER 9007199254740991. Here we are
    // coercing a Rust u64 value to an i64 which could result in overflow. Currently there
    // is no nodejs-sys implementation for napi_bigint_uint64.
    // See: https://nodejs.org/api/n-api.html#n_api_napi_create_bigint_uint64
    napi_create_int64(env, v as i64, &mut obj);

    napi_resolve_deferred(env, t.deferred, obj);

    napi_delete_async_work(env, t.work);
}

// --- Private --- //

type CallbackFn = unsafe extern "C" fn(napi_env, napi_callback_info) -> napi_value;

unsafe fn create_function(env: napi_env, exports: napi_value, name: &str, func: CallbackFn) {
    let cname = CString::new(name).expect("CString::new failed");
    let mut result: napi_value = std::mem::zeroed();

    napi_create_function(
        env,
        cname.as_ptr(),
        cname.as_bytes().len(),
        Some(func),
        std::ptr::null_mut(),
        &mut result,
    );

    napi_set_named_property(env, exports, str_ptr(&cname), result);
}

fn str_ptr(s: &CString) -> *const i8 {
    s.as_ptr()
}

fn str_len(s: &CString) -> usize {
    s.as_bytes().len()
}

fn fib(n: u64) -> u64 {
    let mut result = 1;
    let mut previous = 0;

    for _ in 2..n {
        let temp = result;
        result = result + previous;
        previous = temp;
    }

    println!("lib.rs: fib({:?}) result='{:?}'", n, result);
    result
}

// fn fib(n: i64) -> i64 {
//   return match n {
//     1 | 2 => 1,
//     _ => fib(n - 1) + fib(n - 2)
//   }
// }
