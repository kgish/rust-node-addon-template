use nodejs_sys::{
    napi_callback_info, napi_create_double, napi_env, napi_get_cb_info, napi_get_value_double,
    napi_value,
};

// --- add_numbers(x,y) => number --- //
pub unsafe extern "C" fn run(env: napi_env, info: napi_callback_info) -> napi_value {
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
