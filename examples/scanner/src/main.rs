use leptos::*;
use leptos_qr_scanner::Scan;
use leptos::html::Input;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (scan_signal, scan_set) = create_signal(cx, true);
    let checkbox_ref = create_node_ref::<Input>(cx);

    let (result_signal, set_result) = create_signal(cx, "".to_string());

    view! { cx,
        <Scan
            active=scan_signal
            on_scan=move |a| {
                log!("scanned: {}", &a);
                set_result.set(a);
            }
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
    mount_to_body(move |cx| {
        view! { cx,
            <App />
        }
    });
}
