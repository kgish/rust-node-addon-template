use neon::prelude::*;
use neon::register_module;

fn say_hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("Hello from the kingdom of Rust!"))
}

register_module!(mut m, { m.export_function("say_hello", say_hello) });
