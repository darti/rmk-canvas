use std::collections::HashMap;

use dioxus::{
    html::geometry::{
        euclid::{Point2D, Vector2D},
        ClientSpace, ElementPoint, ElementSpace,
    },
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
    let delta = use_state(cx, || Vector2D::<f64, ElementSpace>::zero());
    let pointers = use_ref(cx, || HashMap::<i32, PointerData>::new());

    let inherited_class = cx.props.class.clone();

    let doc_box = format!("0 0 {} {}", CANVAS_WIDTH, CANVAS_HEIGHT);

    let mousedown = move |event: Event<MouseData>| {
        dragging.set(true);
        origin.set(event.element_coordinates());
        // origin.set(event.client_coordinates());
    };
    let mousemove = move |event: Event<MouseData>| {
        if *dragging.get() {
            delta.set(event.element_coordinates() - *origin.get() + delta.get());
            origin.set(event.element_coordinates());

            info!("delta: {:?}", delta);
        }
    };

    let mouseup = move |_event| {
        dragging.set(false);
    };

    let pointerdown = move |event: Event<PointerData>| {
        pointers
            .write()
            .insert(event.pointer_id, (*event.data).clone());
    };
    let pointermove = move |event: Event<PointerData>| {
        pointers
            .write()
            .insert(event.pointer_id, (*event.data).clone());
    };
    let pointerup = move |event: Event<PointerData>| {
        pointers.write().remove(&event.pointer_id);
    };

    cx.render(rsx! {
            svg {
                class: inherited_class + class!(bg_base_100),
                // view_box: "{doc_box}",
                onmousedown: mousedown,
                onmousemove: mousemove,
                onmouseup: mouseup,

                onpointerdown: pointerdown,
                onpointermove: pointermove,
                onpointerup: pointerup,

                g {
                    transform:"translate({delta.x} {delta.y})",
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
                    pointers.read().values().map(|p| {
                            rsx! {
                                pre {
                                    format!("{:#?}", p)
                                }
                            }
                    })
                }

                div {
                    class: class!(card_actions justify_end),
                    button {
                        class: class!(btn btn_primary),
                        "Reset"
                    }

                }
                }
            }


    }


    })
}
