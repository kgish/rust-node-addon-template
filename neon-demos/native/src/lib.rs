use neon::prelude::*;
use neon::register_module;

use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::mem::replace;

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

struct FibonacciTask {
    argument: usize,
}

fn compute(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        // This is a low cost way of swapping f0 with f1 and f1 with f2.
        f0 = replace(&mut f1, f2);
    }
    f0
}

impl Task for FibonacciTask {
    type Output = BigUint;
    type Error = ();
    type JsEvent = JsString;

    fn perform(&self) -> Result<BigUint, ()> {
        Ok(compute(self.argument))
    }

    fn complete(self, mut cx: TaskContext, result: Result<BigUint, ()>) -> JsResult<JsString> {
        Ok(cx.string(result.unwrap().to_str_radix(10)))
    }
}

fn fibonacci_async(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let n = cx.argument::<JsNumber>(0)?.value() as usize;
    let cb = cx.argument::<JsFunction>(1)?;

    let task = FibonacciTask { argument: n };
    task.schedule(cb);

    Ok(cx.undefined())
}

// --- Register and export all functions --- //
register_module!(mut m, {
    m.export_function("sayHello", say_hello)?;
    m.export_function("addNumbers", add_numbers)?;
    m.export_function("sendMessage", send_message)?;
    m.export_function("getUser", get_user)?;
    m.export_function("fibonacci", fibonacci_async)?;
    Ok(())
});
