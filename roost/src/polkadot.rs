use js_sys::{Error, Function, JsString, Object, Promise};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::{console, window};

fn get_sign_request(msg: &str, address: String) -> Result<js_sys::Object, Error> {
    let sign_request_param = js_sys::Object::new();

    js_sys::Reflect::set(&sign_request_param, &"address".into(), &address.into())?;
    js_sys::Reflect::set(&sign_request_param, &"data".into(), &msg.into())?;

    Ok(sign_request_param)
}

#[wasm_bindgen]
pub async fn polkadot_sign(message: &str) -> Result<(), Error> {
    console::log_2(&"Message to sign:".into(), &message.into());

    let window = window().expect("Failed to access window object");

    let injected_web3 = window
        .get("injectedWeb3")
        .expect("Failed to access window.injectedWeb3");

    let polkadot_js: JsValue =
        js_sys::Reflect::get(&injected_web3, &JsString::from("polkadot-js"))?;

    let enable: Function = js_sys::Reflect::get(&polkadot_js, &JsString::from("enable"))?.into();

    let lib: Object =
        wasm_bindgen_futures::JsFuture::from(Promise::from(enable.call0(&JsValue::NULL)?))
            .await?
            .into();

    let addresses: js_sys::Array = wasm_bindgen_futures::JsFuture::from(Promise::from(
        js_sys::Function::from(js_sys::Reflect::get(
            &js_sys::Reflect::get(&lib, &JsString::from("accounts"))?,
            &JsString::from("get"),
        )?)
        .call0(&JsValue::NULL)?,
    ))
    .await?
    .into();

    let name: String = js_sys::Reflect::get(&addresses.at(0), &"name".into())?
        .as_string()
        .expect("Failed to cast addresses[0] to String");
    let address: String = js_sys::Reflect::get(&addresses.at(0), &"address".into())?
        .as_string()
        .expect("Failed to cast addresses[0] to String");

    console::log_1(&format!("Hello {}! ({})", name, address).into());

    let signer = js_sys::Reflect::get(&lib, &"signer".into())?;
    let sign_raw: Function = js_sys::Reflect::get(&signer, &"signRaw".into())?.into();

    let sign_payload = get_sign_request(message, address)?;

    let sign_promise: Promise = sign_raw.call1(&JsValue::NULL, &sign_payload)?.into();

    let signature = wasm_bindgen_futures::JsFuture::from(sign_promise).await?;

    console::log_1(&signature);

    Ok(())
}
