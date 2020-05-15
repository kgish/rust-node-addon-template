use nodejs_sys::{
    napi_async_work, napi_callback_info, napi_create_async_work, napi_create_error,
    napi_create_int64, napi_create_promise, napi_create_string_utf8, napi_deferred,
    napi_delete_async_work, napi_env, napi_get_cb_info, napi_get_value_int64,
    napi_queue_async_work, napi_reject_deferred, napi_resolve_deferred, napi_status, napi_value,
};
use std::ffi::c_void;
use std::ffi::CString;

// --- fibonacci(n) => number --- //
#[derive(Debug, Clone)]
struct Data {
    deferred: napi_deferred,
    work: napi_async_work,
    val: u64,
    result: Option<Result<u64, String>>,
}

pub unsafe extern "C" fn run(env: napi_env, info: napi_callback_info) -> napi_value {
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
        async_name.as_ptr(),
        async_name.as_bytes().len(),
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
