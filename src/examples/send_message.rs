use nodejs_sys::{
    napi_callback_info, napi_env, napi_get_cb_info, napi_get_undefined, napi_get_value_string_utf8,
    napi_value,
};

// send_message(msg) => () --- //
pub unsafe extern "C" fn run(env: napi_env, info: napi_callback_info) -> napi_value {
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
