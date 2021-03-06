use nodejs_sys::{napi_callback_info, napi_create_string_utf8, napi_env, napi_value};
use std::ffi::CString;

// --- say_hello() => string --- //
pub unsafe extern "C" fn say_hello(env: napi_env, _info: napi_callback_info) -> napi_value {
    let mut result: napi_value = std::mem::zeroed();
    let s = CString::new("Hello from the mighty kingdom of Rust!").expect("CString::new failed");

    napi_create_string_utf8(env, s.as_ptr(), s.as_bytes().len(), &mut result);

    println!("RUST: say_hello() => {:?} ({:?})", s, result);

    result
}
