use neon::prelude::*;

#[derive(Debug)]
struct User { pub name: String, pub age: u8, }

// --- get_user() => user --- //
pub fn get_user(mut cx: FunctionContext) -> JsResult<JsObject> {

    let user = User { name: "kiffin".to_string(), age: 36, };

    println!("RUST: get_user() => '{:?}'", user);

    let object = JsObject::new(&mut cx);
    let name = cx.string(&user.name);
    let age = cx.number(user.age as f64);
    object.set(&mut cx, "name", name).unwrap();
    object.set(&mut cx, "age", age).unwrap();
    Ok(object)
}

