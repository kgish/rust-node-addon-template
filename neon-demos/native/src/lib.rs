use neon::prelude::*;
use neon::register_module;

fn say_hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("Hello from the kingdom of Rust!"))
}

fn add_numbers(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let x = cx.argument::<JsNumber>(0)?.value();
    let y = cx.argument::<JsNumber>(1)?.value();
    Ok(cx.number(x + y))
}

fn send_message(mut cx: FunctionContext) -> JsResult<JsNull> {
    let s = cx.argument::<JsString>(0)?.value();
    println!("lib.rs: send_message({})", s);
    Ok(cx.null())
}

register_module!(mut m, {
    m.export_function("sayHello", say_hello)?;
    m.export_function("addNumbers", add_numbers)?;
    m.export_function("sendMessage", send_message)?;
    Ok(())
});
