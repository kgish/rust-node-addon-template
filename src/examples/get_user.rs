use nodejs_sys::{
    napi_callback_info, napi_create_object, napi_create_uint32, napi_env, napi_value,
};

// --- get_user() => user --- //
pub unsafe extern "C" fn run(env: napi_env, _info: napi_callback_info) -> napi_value {
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
