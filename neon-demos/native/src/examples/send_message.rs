use neon::prelude::*;

// send_message(msg) => () --- //
pub fn send_message(mut cx: FunctionContext) -> JsResult<JsNull> {
    let s = cx.argument::<JsString>(0)?.value();
    println!("RUST: send_message({})", s);
    Ok(cx.null())
}
