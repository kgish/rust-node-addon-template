use nodejs_sys::{napi_create_string_utf8, napi_env, napi_set_named_property, napi_value};
use std::ffi::CString;

// #[no_mangle] : The Rust compiler mangles symbol names differently than native code linkers
// expect and therefore needs to be told NOT to mangle any functions exported to the outside world.
// extern "C" : By default, any function you write in Rust will use the Rust ABI. Instead, when
// building outwards facing FFI APIs we need to tell the compiler to use the system ABI.
#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(
    env: napi_env,
    exports: napi_value,
) -> nodejs_sys::napi_value {
    let key = CString::new("hello").expect("CString::new failed");
    let mut local: napi_value = std::mem::zeroed();
    let value = CString::new("world!").expect("CString::new failed");
    napi_create_string_utf8(env, value.as_ptr(), 6, &mut local);
    napi_set_named_property(env, exports, key.as_ptr(), local);
    exports
}
