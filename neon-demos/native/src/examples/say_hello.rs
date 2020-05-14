use neon::prelude::*;

// --- say_hello() => string --- //
pub fn run(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("Hello from the mighty kingdom of Rust!"))
}
