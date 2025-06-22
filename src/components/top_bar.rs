use dioxus::prelude::*;

pub fn TopBar() -> Element {
    rsx! {
        div {
            id: "top_bar",
            style {
                display: "flex"
            }
            button {
                id: "postie_button",
                "Postie"
            }
        }
    }
}
