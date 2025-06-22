use crate::components::*;
use dioxus::prelude::*;

pub fn RequestView() -> Element {
    rsx! {
        div {
            id: "request_view",
            RequestBar{},
            BodyView{}
        }
    }
}
