mod examples;

use nodejs_sys::{
    napi_callback_info, napi_create_function, napi_env, napi_set_named_property, napi_value,
};
use std::ffi::CString;

type CallbackFn = unsafe extern "C" fn(napi_env, napi_callback_info) -> napi_value;

#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(
    env: napi_env,
    exports: napi_value,
) -> nodejs_sys::napi_value {
    println!("lib.rs: napi_register_module_v1()");

    create_function(env, exports, "sayHello", examples::say_hello::run);
    create_function(env, exports, "sendMessage", examples::send_message::run);
    create_function(env, exports, "addNumbers", examples::add_numbers::run);
    create_function(env, exports, "getUser", examples::get_user::run);
    create_function(env, exports, "fibonacci", examples::fibonacci_async::run);

    exports
}

unsafe fn create_function(env: napi_env, exports: napi_value, name: &str, func: CallbackFn) {
    let cname = CString::new(name).expect("CString::new failed");
    let mut result: napi_value = std::mem::zeroed();

    println!("lib.rs: napi_create_function({})", name);
    napi_create_function(
        env,
        cname.as_ptr(),
        cname.as_bytes().len(),
        Some(func),
        std::ptr::null_mut(),
        &mut result,
    );

    napi_set_named_property(env, exports, cname.as_ptr(), result);
}
