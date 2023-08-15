// Extracted from https://github.com/sectore/fm-faucet-leptos/blob/main/src/component/scan.rs by  Jens K./sectore, licensed under MIT License

use std::sync::Arc;
use leptos::*;
use js_sys::Object;
use leptos_meta::{Script, Title, Stylesheet, provide_meta_context};
use leptos::html::Video;
use leptos::html::Input;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    type QrScanner;

    #[wasm_bindgen(constructor)]
    fn new(
        video_elem: &web_sys::HtmlVideoElement,
        callback: &js_sys::Function,
        options: &JsValue,
    ) -> QrScanner;

    #[wasm_bindgen(method, js_name = start)]
    fn start(this: &QrScanner);
    #[wasm_bindgen(method, js_name = stop)]
    fn stop(this: &QrScanner);
    #[wasm_bindgen(method, js_name = destroy)]
    fn destroy(this: &QrScanner);
}

#[wasm_bindgen]
pub fn process_js_value_with_cast(js_value: JsValue) -> Result<String, JsValue> {
    // Attempt to cast JsValue to an Object
    if let Ok(obj) = js_value.dyn_into::<Object>() {
        // Try to get the 'data' property from the object
        if let Ok(data) = js_sys::Reflect::get(&obj, &JsValue::from_str("data")) {
            // Convert the value to a string if possible
            if let Some(data_string) = data.as_string() {
                return Ok(data_string);
            }
        }
    }

    // Return an error if the extraction fails
    Err(JsValue::from_str("Failed to extract the data properly"))
}

#[component]
pub fn Scan<A, F>(cx: Scope, active: A, on_scan: F) -> impl IntoView
    where
        A: SignalGet<bool> + 'static,
        F: Fn(String) + 'static,
{
    provide_meta_context(cx);

    let video_ref = create_node_ref::<Video>(cx);
    let (error, set_error) = create_signal(cx, None);

    let o_scanner: StoredValue<Option<QrScanner>> = store_value(cx, None);

    let on_scan = Arc::new(on_scan);
    let scan = move || {
        if let Some(video) = video_ref.get() {
            let on_scan_inner = on_scan.clone();
            let callback = Closure::wrap(Box::new(move |result: JsValue| {
                match process_js_value_with_cast(result) {
                    Ok(data) => {
                        on_scan_inner(data);
                    }
                    Err(e) => {
                        let error_message = format!("Error: {:?}", e);
                        set_error.set(Some(error_message));
                    }
                };
            }) as Box<dyn Fn(JsValue)>);

            // Options of `QrScanner`
            // JS: {returnDetailedScanResult: true} - Enforce reporting detailed scan results
            // https://github.com/nimiq/qr-scanner/#2-create-a-qrscanner-instance
            let options = js_sys::Object::new();
            js_sys::Reflect::set(
                &options,
                &JsValue::from_str("returnDetailedScanResult"),
                &JsValue::from_bool(true),
            )
                .unwrap();

            let scanner = QrScanner::new(&video, callback.as_ref().unchecked_ref(), &options);
            scanner.start();
            callback.forget();

            o_scanner.set_value(Some(scanner));
        }
    };

    let cancel = move || {
        if let Some(scanner) = o_scanner.get_value() {
            scanner.stop();
            scanner.destroy();
            o_scanner.set_value(None);
        }
    };

    create_effect(cx, move |_| {
        if active.get() {
            scan();
        } else {
            cancel();
        }
    });

    view! { cx,
        <Script src="https://unpkg.com/qr-scanner@1.4.2/qr-scanner.legacy.min.js" />
        <div>
          <video _ref=video_ref></video>
          <Show
            when=move || error.get().is_some()
            fallback=|_| {
                view! { cx, "" }
            }
          >
            <p>
              {error.get()}
            </p>
          </Show>
        </div>
  }
}
