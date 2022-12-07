use js_sys::{Error, Function, JsString, Promise};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::{console, window};

fn get_sign_request(msg: &str, address: String) -> Result<js_sys::Object, Error> {
    let sign_request_param = js_sys::Object::new();
    let sign_params = js_sys::Array::new();
    js_sys::Reflect::set(
        &sign_request_param,
        &"method".into(),
        &"personal_sign".into(),
    )?;
    js_sys::Array::push(&sign_params, &msg.into());
    js_sys::Array::push(&sign_params, &address.clone().into());
    js_sys::Reflect::set(&sign_request_param, &"params".into(), &sign_params)?;
    Ok(sign_request_param)
}

#[wasm_bindgen]
pub async fn ethereum_sign(message: &str) -> Result<(), Error> {
    let window = window().expect("Failed to access window object");

    let ethereum = window
        .get("ethereum")
        .expect("Failed to access window.ethereum");

    console::log_1(&ethereum);

    let address: String = js_sys::Reflect::get(&ethereum, &JsString::from("selectedAddress"))?
        .as_string()
        .expect("Failed to cast window.ethereum.selectedAddress to String");

    console::log_1(&format!("Hello {}! Please sign this.", &address).into());

    let request: Function = js_sys::Reflect::get(&ethereum, &JsString::from("request"))?.into();

    let sign_request = get_sign_request(message, address)?;

    let res: Promise = request.call1(&JsValue::NULL, &sign_request)?.into();

    let signature: String = wasm_bindgen_futures::JsFuture::from(res)
        .await?
        .as_string()
        .expect("Failed to cast signature to String.");

    console::log_1(&format!("Signature: {}", &signature).into());

    Ok(())
}
