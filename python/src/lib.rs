use pyo3::prelude::*;
use simple2fa as core;
use pyo3::types::PyBytes;


// #[pyfunction]
// pub fn generate_2fa_secret(mut cx: FunctionContext) -> JsResult<JsString> {
//     let s = simple2fa::generate_2fa_secret();
//     Ok(cx.string(&s))
// }
//
//
// pub fn create_urlencoded_qrcode(mut cx: FunctionContext) -> JsResult<JsString> {
//     let service_name: String = cx.argument::<JsString>(0)?.value(&mut cx);
//     let user_name: String = cx.argument::<JsString>(1)?.value(&mut cx);
//     let secret: String = cx.argument::<JsString>(2)?.value(&mut cx);
//     let s = simple2fa::create_urlencoded_qrcode(&service_name, &user_name, &secret);
//     Ok(cx.string(&s))
// }
//
//
// pub fn create_png_qrcode(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
//     let service_name: String = cx.argument::<JsString>(0)?.value(&mut cx);
//     let user_name: String = cx.argument::<JsString>(1)?.value(&mut cx);
//     let secret: String = cx.argument::<JsString>(2)?.value(&mut cx);
//     let mut b = simple2fa::create_png_qrcode(&service_name, &user_name, &secret);
//     let js_buf = JsArrayBuffer::external(&mut cx, b);
//     Ok(js_buf)
// }
//
//
// pub fn check_2fa_code(mut cx: FunctionContext) -> JsResult<JsBoolean> {
//     let secret: String = cx.argument::<JsString>(0)?.value(&mut cx);
//     let code: String = cx.argument::<JsString>(1)?.value(&mut cx);
//     let result = simple2fa::check_2fa_code(&secret, &code);
//     Ok(cx.boolean(result))
// }

#[pyfunction]
fn add(a: u32, b: u32) -> PyResult<u32> {
    Ok(a + b)
}

#[pyfunction]
fn concat2(secret: &str, b: &str) -> PyResult<String> {
    let mut s = String::from(secret);
    s.push_str(b);
    Ok(s)
}

#[pyfunction]
fn concat3(secret: &str, b: &str) -> PyResult<String> {
    let mut s = String::from(secret);
    s.push_str(b);
    Ok(s)
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn generate_2fa_secret() -> PyResult<String> {
    Ok(core::generate_2fa_secret())
}


#[pyfunction]
pub fn create_urlencoded_qrcode(service_name: &str, user_name: &str, secret: &str) -> PyResult<String> {
    Ok(core::create_urlencoded_qrcode(service_name, user_name, secret))
}


#[pyfunction]
pub fn create_png_qrcode<'a>(py: Python<'a>, service_name: &str, user_name: &str, secret: &str) -> PyResult<&'a PyBytes> {
    let data = core::create_png_qrcode(service_name, user_name, secret);
    Ok(PyBytes::new(py, &data[..]))
}

// #[pyfunction]
// pub fn create_png_qrcode(service_name: &str, user_name: &str, secret: &str) -> PyResult<PyBytes> {
//     let bytes = core::create_png_qrcode(service_name, user_name, secret);
//     Ok(PyBytes::new(bytes))
// }


#[pyfunction]
pub fn check_2fa_code(secret: &str, code: &str) -> PyResult<bool> {
    Ok(core::check_2fa_code(secret, code))
}


#[pyfunction]
pub fn generate_2fa_code(secret: &str) -> PyResult<String> {
    Ok(core::generate_2fa_code(secret))
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn simple2fa(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_2fa_secret, m)?)?;
    m.add_function(wrap_pyfunction!(create_urlencoded_qrcode, m)?)?;
    m.add_function(wrap_pyfunction!(create_png_qrcode, m)?)?;
    m.add_function(wrap_pyfunction!(check_2fa_code, m)?)?;
    m.add_function(wrap_pyfunction!(generate_2fa_code, m)?)?;
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(concat2, m)?)?;
    m.add_function(wrap_pyfunction!(concat3, m)?)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
