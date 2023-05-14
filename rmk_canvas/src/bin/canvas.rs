use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;
use rmk_canvas::components::{Notebook, Viewer};
use rmk_format::notebook;

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    dioxus_web::launch(App);
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    let notebook = use_future(cx, (), |_| async move {
        reqwest::get("http://localhost:8080/e3c22b43-bd2c-42d8-8b45-d9bdf829b500.rm")
            .await
            .unwrap()
            .bytes()
            .await
            .map(|bs| notebook(&bs))
    });

    cx.render(match notebook.value() {
        Some(Ok(Ok(nb))) => rsx!(div {
            class: class!(bg_base_200 w_screen h_screen flex justify_center),

            Viewer {
                class: class!(grow m_10 drop_shadow_md),
                body: cx.render(rsx!{
                    Notebook {
                        notebook: &nb,
                    }
                })
            },
        }),
        Some(Ok(Err(_))) => rsx! { "Error" },
        Some(Err(_)) => rsx! { "Error" },
        None => rsx! { "Loading..." },
    })
}
