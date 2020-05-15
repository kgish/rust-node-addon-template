use nodejs_sys::{
    napi_callback_info, napi_create_object, napi_create_string_utf8, napi_create_uint32, napi_env,
    napi_set_named_property, napi_value,
};

use std::ffi::CString;

// --- get_user() => user --- //
pub unsafe extern "C" fn run(env: napi_env, _info: napi_callback_info) -> napi_value {
    let mut result: napi_value = std::mem::zeroed();

    let mut val_name: napi_value = std::mem::zeroed();
    let mut val_age: napi_value = std::mem::zeroed();
    let cstr_key_name = CString::new("name").expect("CString::new failed");
    let cstr_val_name = CString::new("kiffin").expect("CString::new failed");
    let cstr_key_age = CString::new("age").expect("CString::new failed");

    napi_create_string_utf8(
        env,
        cstr_val_name.as_ptr(),
        cstr_val_name.as_bytes().len(),
        &mut val_name,
    );

    napi_create_uint32(env, 36 as u32, &mut val_age);

    // Create object
    napi_create_object(env, &mut result);
    napi_set_named_property(env, result, cstr_key_name.as_ptr(), val_name);
    napi_set_named_property(env, result, cstr_key_age.as_ptr(), val_age);

    println!("lib.rs: get_user() => ({:?})", result);

    result
}
