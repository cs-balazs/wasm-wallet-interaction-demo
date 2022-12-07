use js_sys::{Function, JsString, Object, Promise};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::{console, window};

fn get_sign_request(msg: &str, address: String) -> js_sys::Object {
    let sign_request_param = js_sys::Object::new();

    js_sys::Reflect::set(&sign_request_param, &"address".into(), &address.into()).unwrap();
    js_sys::Reflect::set(&sign_request_param, &"data".into(), &msg.into()).unwrap();

    sign_request_param
}

#[wasm_bindgen]
pub async fn polkadot_sign(message: &str) {
    let window = window().expect("Failed to access window object");

    let injected_web3: JsValue = window
        .get("injectedWeb3")
        .expect("Failed to access window.injectedWeb3")
        .try_into()
        .expect("Failed to access window.injectedWeb3");

    let polkadot_js: JsValue = js_sys::Reflect::get(&injected_web3, &JsString::from("polkadot-js"))
        .expect("Failed to access window.injectedWeb3[\"polkadot-js\"]");

    console::log_2(&"Message to sign".into(), &message.into());
    console::log_1(&polkadot_js);

    let enable: Function = js_sys::Reflect::get(&polkadot_js, &JsString::from("enable"))
        .unwrap()
        .into();

    let lib: Object =
        wasm_bindgen_futures::JsFuture::from(Promise::from(enable.call0(&JsValue::NULL).unwrap()))
            .await
            .unwrap()
            .into();

    let addresses: js_sys::Array = wasm_bindgen_futures::JsFuture::from(Promise::from(
        js_sys::Function::from(
            js_sys::Reflect::get(
                &js_sys::Reflect::get(&lib, &JsString::from("accounts")).unwrap(),
                &JsString::from("get"),
            )
            .unwrap(),
        )
        .call0(&JsValue::NULL)
        .unwrap(),
    ))
    .await
    .unwrap()
    .into();

    console::log_1(&addresses);

    let name: String = js_sys::Reflect::get(&addresses.at(0), &"name".into())
        .unwrap()
        .as_string()
        .unwrap();
    let address: String = js_sys::Reflect::get(&addresses.at(0), &"address".into())
        .unwrap()
        .as_string()
        .unwrap();

    console::log_1(&format!("Hello {}! ({})", name, address).into());

    let signer = js_sys::Reflect::get(&lib, &"signer".into()).unwrap();
    let sign_raw: Function = js_sys::Reflect::get(&signer, &"signRaw".into())
        .unwrap()
        .into();

    let sign_payload = get_sign_request(message, address);

    console::log_1(&sign_payload);
    console::log_1(&sign_raw);

    let sign_promise: Promise = sign_raw
        .call1(&JsValue::NULL, &sign_payload)
        .unwrap()
        .into();

    let signature = wasm_bindgen_futures::JsFuture::from(sign_promise)
        .await
        .unwrap();

    console::log_1(&signature);
}
