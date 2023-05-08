use dioxus::prelude::*;
use kurbo::BezPath;
use log::info;
use rmk_format::format::{Layer, Notebook, Point, Stroke};

#[derive(PartialEq, Props)]
pub struct NotebookProps<'a> {
    notebook: &'a Notebook,
}

#[allow(non_snake_case)]
pub fn Notebook<'a>(cx: Scope<'a, NotebookProps<'a>>) -> Element {
    cx.render(rsx! {
            g {
                class: "notebook",

                cx.props.notebook.layers.iter().map(|layer| {
                    rsx! {
                        Layer {
                            layer: layer,
                        }
                    }

                })
            }
    })
}

#[derive(PartialEq, Props)]
pub struct LayerProps<'a> {
    // num: usize,
    layer: &'a Layer,
}

#[allow(non_snake_case)]
pub fn Layer<'a>(cx: Scope<'a, LayerProps<'a>>) -> Element {
    cx.render(rsx! {
        g {
            class: "layer",

            cx.props.layer.strokes.iter().filter(|s| s.points.len() > 0).map(|stroke| {
                rsx! {
                    Stroke {
                        stroke: stroke,
                    }
                }
            })
        }
    })
}

#[derive(PartialEq, Props)]
pub struct StrokeProps<'a> {
    stroke: &'a Stroke,
}

#[allow(non_snake_case)]
pub fn Stroke<'a>(cx: Scope<'a, StrokeProps<'a>>) -> Element {
    let mut pts = cx.props.stroke.points.iter();

    if let Some(origin) = pts.next() {
        let mut path = BezPath::default();

        path.move_to(origin.coords());

        for pt in pts {
            path.line_to(pt.coords());
        }

        cx.render(rsx! {
            path {
                stroke: "black",
                fill: "none",
                d: "{path.to_svg()}",
            }
        })
    } else {
        None
    }
}
