use crate::components::*;
use dioxus::prelude::*;

pub fn ContentView() -> Element {
    rsx! {
        div {
            id: "content_view",
            ContentHeader{},
            RequestView{}
        }
    }
}
