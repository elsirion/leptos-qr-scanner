use leptos::html::Input;
use leptos::logging::log;
use leptos::*;
use leptos_meta::*;
use leptos_qr_scanner::Scan;
use leptos_router::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <App /> }
    })
}

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light" />
        <Title text="Welcome to Leptos QRScanner CSR" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes>
                <Route path="/" view=Page />
            </Routes>
        </Router>
    }
}

#[component]
pub fn Page() -> impl IntoView {
    let (scan_signal, scan_set) = create_signal(false);
    let checkbox_ref = create_node_ref::<Input>();

    let (result_signal, set_result) = create_signal("".to_string());

    view! {
        <h1>QRScanner CSR</h1>
        <Scan
            active=scan_signal
            on_scan=move |a| {
                log!("scanned: {}", &a);
                set_result.set(a);
            }
            class=""
            video_class=""
        />
        <label>
            Scan
            <input
                type="checkbox"
                ref=checkbox_ref
                on:change=move |_e| {
                    let checked = checkbox_ref.get().expect("<input> to exist").checked();
                    scan_set.set(checked);
                }
            />
        </label>
        <p>Scan result: {result_signal}</p>
    }
}
