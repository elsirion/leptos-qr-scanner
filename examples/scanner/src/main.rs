use leptos::html::Input;
use leptos::logging::log;
use leptos::*;
use leptos_qr_scanner::Scan;

#[component]
pub fn App() -> impl IntoView {
    let (scan_signal, scan_set) = create_signal(true);
    let checkbox_ref = create_node_ref::<Input>();

    let (result_signal, set_result) = create_signal("".to_string());

    view! {
          <Scan
              active=scan_signal
              on_scan=move |a| {
                  log!("scanned: {}", &a);
                  set_result.set(a);
              }
              class=""
              video_class=""
          />
          "Scan "
          <input
              type="checkbox"
              ref=checkbox_ref
              checked
              on:change=move |_e| {
                  let checked = checkbox_ref.get().expect("<input> to exist").checked();
                  scan_set.set(checked);
              }
          />
          <p>"Scan result: " {result_signal} </p>
    }
}

pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(move || {
        view! {
            <App />
        }
    });
}
