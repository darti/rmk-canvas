use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

#[derive(Props)]
pub struct ViewerProps<'a> {
    body: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Viewer<'a>(cx: Scope<'a, ViewerProps<'a>>) -> Element {
    cx.render(rsx! {
            svg {
                class: class!(bg_base_100),
                &cx.props.body
            }
    })
}
