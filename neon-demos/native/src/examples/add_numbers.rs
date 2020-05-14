use neon::prelude::*;

// --- add_numbers(x,y) => number --- //
pub fn run(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let x = cx.argument::<JsNumber>(0)?.value();
    let y = cx.argument::<JsNumber>(1)?.value();
    Ok(cx.number(x + y))
}
