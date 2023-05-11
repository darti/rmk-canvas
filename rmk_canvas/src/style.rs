use dioxus_daisyui::prelude::*;
use lazy_static::lazy_static;

style! {
    none: Class::NONE.clone(),
    completed: class!("completed"),
    editing: class!("editing"),
}

lazy_static! {
    pub static ref STYLE: Style = Style::default();
}
