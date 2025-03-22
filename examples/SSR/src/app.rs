use crate::error_template::{AppError, ErrorTemplate};
use leptos::html::Input;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_qr_scanner::Scan;
use leptos_router::{components::*, path};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/qrscanner_ssr.css" />
        <Title text="Welcome to Leptos" />

        <Router>
            <main>
                <Routes fallback=|| {
                    let mut outside_errors = Errors::default();
                    outside_errors.insert_with_default_key(AppError::NotFound);
                    view! { <ErrorTemplate outside_errors /> }.into_view()
                }>
                    <Route path=path!("") view=Page />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Page() -> impl IntoView {
    let (scan_signal, scan_set) = signal(false);
    let checkbox_ref = NodeRef::<Input>::new();

    let (result_signal, set_result) = signal("".to_string());

    view! {
        <h1>QRScanner SSR</h1>
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
                node_ref=checkbox_ref
                on:change=move |_e| {
                    let checked = checkbox_ref.get().expect("<input> to exist").checked();
                    scan_set.set(checked);
                }
            />
        </label>
        <p>Scan result: {result_signal}</p>
    }
}
