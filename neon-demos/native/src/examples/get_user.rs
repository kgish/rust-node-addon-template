use neon::prelude::*;

// --- 4. get_user() => user --- //
pub fn run(mut cx: FunctionContext) -> JsResult<JsObject> {
    struct User { pub name: String, pub age: u8, }

    let user = User { name: "kiffin".to_string(), age: 36, };

    let object = JsObject::new(&mut cx);
    let name = cx.string(&user.name);
    let age = cx.number(user.age as f64);
    object.set(&mut cx, "name", name).unwrap();
    object.set(&mut cx, "age", age).unwrap();
    Ok(object)
}

