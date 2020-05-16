mod examples;

use nodejs_sys::{
    napi_callback_info, napi_create_function, napi_env, napi_set_named_property, napi_value,
};

use std::ffi::CString;

// Register module
// The N-API documentation recommends `NAPI_MODULE_INIT()` macro for module registration which
// compiles to the `napi_register_module_v1` function.
//
// -------------------------------------------------------------------------------
// addon_node.c
// #include <node_api.h>
// #include "addon.h"
//
// NAPI_MODULE_INIT() {
//   // This function body is expected to return a `napi_value`.
//   // The variables `napi_env env` and `napi_value exports` may be used within
//   // the body, as they are provided by the definition of `NAPI_MODULE_INIT()`.
//   return create_addon(env);
// }
// -------------------------------------------------------------------------------
//
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

type CallbackFn = unsafe extern "C" fn(napi_env, napi_callback_info) -> napi_value;

unsafe fn create_function(env: napi_env, exports: napi_value, name: &str, func: CallbackFn) {
    let cname = CString::new(name).expect("CString::new failed");
    let mut result: napi_value = std::mem::zeroed();

    println!("lib.rs: napi_create_function({})", name);

    // Create function
    // https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_create_function.html
    //
    // pub unsafe extern "C" fn napi_create_function(
    //     env: napi_env,
    //     utf8name: *const c_char,
    //     length: usize,
    //     cb: napi_callback,
    //     data: *mut c_void,
    //     result: *mut napi_value
    // ) -> napi_status
    //
    napi_create_function(
        env,
        cname.as_ptr(),
        cname.as_bytes().len(),
        Some(func),
        std::ptr::null_mut(),
        &mut result,
    );

    // Set named property
    // https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_set_named_property.html
    //
    // pub unsafe extern "C" fn napi_set_named_property(
    //     env: napi_env,
    //     object: napi_value,
    //     utf8name: *const c_char,
    //     value: napi_value
    // ) -> napi_status
    //
    napi_set_named_property(env, exports, cname.as_ptr(), result);
}
