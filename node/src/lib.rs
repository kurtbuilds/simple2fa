use neon::prelude::*;
use simple2fa;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    let s = simple2fa::hello();
    Ok(cx.string(&s))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    Ok(())
}
