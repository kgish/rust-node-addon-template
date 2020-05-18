use neon::prelude::*;

// --- add_numbers(x,y) => number --- //
pub fn add_numbers(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let x = cx.argument::<JsNumber>(0)?.value();
    let y = cx.argument::<JsNumber>(1)?.value();
    let result = x + y;
    println!("RUST: add_numbers({},{}) result='{}'", x, y, result);
    Ok(cx.number(result))
}
