use neon::prelude::*;
use simple2fa;


pub fn generate_2fa_secret(mut cx: FunctionContext) -> JsResult<JsString> {
    let s = simple2fa::generate_2fa_secret();
    Ok(cx.string(&s))
}


pub fn create_urlencoded_qrcode(mut cx: FunctionContext) -> JsResult<JsString> {
    let service_name: String = cx.argument::<JsString>(0)?.value(&mut cx);
    let user_name: String = cx.argument::<JsString>(1)?.value(&mut cx);
    let secret: String = cx.argument::<JsString>(2)?.value(&mut cx);
    let s = simple2fa::create_urlencoded_qrcode(&service_name, &user_name, &secret);
    Ok(cx.string(&s))
}


pub fn create_png_qrcode(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
    let service_name: String = cx.argument::<JsString>(0)?.value(&mut cx);
    let user_name: String = cx.argument::<JsString>(1)?.value(&mut cx);
    let secret: String = cx.argument::<JsString>(2)?.value(&mut cx);
    let mut b = simple2fa::create_png_qrcode(&service_name, &user_name, &secret);
    let js_buf = JsArrayBuffer::external(&mut cx, b);
    Ok(js_buf)
}


pub fn check_2fa_code(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let secret: String = cx.argument::<JsString>(0)?.value(&mut cx);
    let code: String = cx.argument::<JsString>(1)?.value(&mut cx);
    let result = simple2fa::check_2fa_code(&secret, &code);
    Ok(cx.boolean(result))
}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("generate_2fa_secret", generate_2fa_secret)?;
    cx.export_function("create_urlencoded_qrcode", create_urlencoded_qrcode)?;
    cx.export_function("create_png_qrcode", create_png_qrcode)?;
    cx.export_function("check_2fa_code", check_2fa_code)?;
    Ok(())
}
