use dioxus::{
    html::geometry::{euclid::Vector2D, ElementPoint, ElementSpace, PixelsVector, WheelDelta},
    prelude::*,
};
use dioxus_daisyui::prelude::*;
use log::info;
use rmk_format::{CANVAS_HEIGHT, CANVAS_WIDTH};

#[derive(Props)]
pub struct ViewerProps<'a> {
    class: Class,
    body: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Viewer<'a>(cx: Scope<'a, ViewerProps<'a>>) -> Element {
    let dragging = use_state(cx, || false);
    let origin = use_state(cx, || ElementPoint::zero());
    let translation = use_state(cx, || Vector2D::<f64, ElementSpace>::zero());
    let mut scale = use_state(cx, || 1.0);

    let inherited_class = cx.props.class.clone();

    let doc_box = format!("0 0 {} {}", CANVAS_WIDTH, CANVAS_HEIGHT);

    let mousedown = move |event: Event<MouseData>| {
        dragging.set(true);
        origin.set(event.element_coordinates());
    };

    let mousemove = move |event: Event<MouseData>| {
        if *dragging.get() {
            translation.set(event.element_coordinates() - *origin.get() + translation.get());
            origin.set(event.element_coordinates());
        }
    };

    let mouseup = move |_event| {
        dragging.set(false);
    };

    let wheel = move |event: Event<WheelData>| match event.delta() {
        WheelDelta::Pixels(PixelsVector { y, .. }) => {
            scale -= y * 1e-3;
        }
        _ => {}
    };

    let reset = move |_| {
        translation.set(Vector2D::<f64, ElementSpace>::zero());
        scale.set(1.0);
    };

    cx.render(rsx! {
            svg {
                class: inherited_class + class!(bg_base_100),
                // view_box: "{doc_box}",
                onmousedown: mousedown,
                onmousemove: mousemove,
                onmouseup: mouseup,
                onwheel: wheel,


                g {
                    transform:"translate({translation.x} {translation.y}) scale({scale})",
                    class: class!(bg_base_200),
                    view {
                        view_box: "{doc_box}",
                    },

                    rect {
                        // class: class!(fill_base_100),
                        fill: "none",
                        stroke: "black",
                        x: 0,
                        y: 0,
                        width: "{CANVAS_WIDTH}",
                        height: "{CANVAS_HEIGHT}",
                    },

                    &cx.props.body

                }
            }

    div {
        class: class!( m_10),
        div {
            class: class!( card shadow_xl w_96 bg_base_100),
            div{
               class: class!(card_body),
               h2 {
                    class: class!(card_title),
                    "Pointer Data",
                }

                p {
                    class: class!(text_sm),
                    "Translation: ({translation.x}, {translation.y})"
                }

                 p {
                    class: class!(text_sm),
                    "Zoom: {scale}"
                }


                div {
                    class: class!(card_actions justify_end),
                    button {
                        class: class!(btn btn_primary),
                        onclick: reset,
                        "Reset"
                    }

                }
            }
        }


    }


    })
}
