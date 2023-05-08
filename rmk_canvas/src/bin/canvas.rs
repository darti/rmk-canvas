use dioxus::prelude::*;
use rmk_canvas::components::Notebook;
use rmk_format::{notebook, CANVAS_HEIGHT, CANVAS_WIDTH};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let notebook = use_future(cx, (), |_| async move {
        reqwest::get("http://localhost:8080/e3c22b43-bd2c-42d8-8b45-d9bdf829b500.rm")
            .await
            .unwrap()
            .bytes()
            .await
            .map(|bs| notebook(&bs))
    });

    cx.render(match notebook.value() {
        Some(Ok(Ok(nb))) => rsx!(svg {
            width: "{CANVAS_WIDTH}",
            height: "{CANVAS_HEIGHT}",

            rect {
                width:"100%",
                height:"100%",
                fill:"none",
                stroke:"grey"
            }

            Notebook {
                notebook: nb,
            }
        }),
        Some(Ok(Err(_))) => rsx! { "Error" },
        Some(Err(_)) => rsx! { "Error" },
        None => rsx! { "Loading..." },
    })
}
