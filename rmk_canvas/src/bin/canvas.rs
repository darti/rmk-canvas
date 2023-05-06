use dioxus::prelude::*;
use kurbo::{Rect, Shape};

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let rect = Rect::from_origin_size((10.0, 5.0), (5.0, 5.0));

    cx.render(rsx!(svg {
        width: 200,
        height: 200,

        rect {
            width:"100%",
             height:"100%",
              fill:"red"
        }

        path {
            d:"{rect.to_path(0.1).to_svg()}",
            stroke: "blue"
        }


    }))
}
