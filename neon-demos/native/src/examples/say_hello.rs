use neon::prelude::*;

// --- say_hello() => string --- //
pub fn say_hello(mut cx: FunctionContext) -> JsResult<JsString> {
    println!("RUST: say_hello()");
    Ok(cx.string("Hello from the mighty kingdom of Rust!"))
}
