use neon::prelude::*;
use neon::register_module;

struct User {
    pub name: String,
    pub age: u8,
}

fn convert_struct_to_js_object(mut cx: FunctionContext) -> JsResult<JsObject> {
    let user = User {
        name: "kiffin".to_string(),
        age: 29,
    };
    let object = JsObject::new(&mut cx);
    let name = cx.string(&user.name);
    let age = cx.number(user.age as f64);
    object.set(&mut cx, "name", name).unwrap();
    object.set(&mut cx, "age", age).unwrap();
    Ok(object)
}

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
    m.export_function("convertStructToJsObject", convert_struct_to_js_object)?;
    m.export_function("sayHello", say_hello)?;
    m.export_function("addNumbers", add_numbers)?;
    m.export_function("sendMessage", send_message)?;
    Ok(())
});
