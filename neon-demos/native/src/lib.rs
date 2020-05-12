use neon::prelude::*;
use neon::register_module;

// --- 1. say_hello() => string --- //
fn say_hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("Hello from the kingdom of Rust!"))
}

// --- 2. send_message(msg) => () --- //
fn send_message(mut cx: FunctionContext) -> JsResult<JsNull> {
    let s = cx.argument::<JsString>(0)?.value();
    println!("lib.rs: send_message({})", s);
    Ok(cx.null())
}

// --- 3. add_numbers(x,y) => number --- //
fn add_numbers(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let x = cx.argument::<JsNumber>(0)?.value();
    let y = cx.argument::<JsNumber>(1)?.value();
    Ok(cx.number(x + y))
}

// --- 4. get_user() => user --- //
fn get_user(mut cx: FunctionContext) -> JsResult<JsObject> {
    struct User { pub name: String, pub age: u8, }

    let user = User { name: "kiffin".to_string(), age: 36, };

    let object = JsObject::new(&mut cx);
    let name = cx.string(&user.name);
    let age = cx.number(user.age as f64);
    object.set(&mut cx, "name", name).unwrap();
    object.set(&mut cx, "age", age).unwrap();
    Ok(object)
}

// --- 5. fibonacci(n) => number --- //

// --- Register and export all functions --- //
register_module!(mut m, {
    m.export_function("getUser", get_user)?;
    m.export_function("sayHello", say_hello)?;
    m.export_function("addNumbers", add_numbers)?;
    m.export_function("sendMessage", send_message)?;
    Ok(())
});
