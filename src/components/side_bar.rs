use dioxus::prelude::*;

pub fn SideBar() -> Element {
    rsx! {
        div {
            id: "side_bar",
            style {
                display: "flex",
                flex_direction: "column"
            }
            button {
                id: "collections_button",
                class: "side_button",
                "collections"
            },
            button {
                id: "environments_button",
                class: "side_button",
                "environments"
            },
            button {
                id: "requests_button",
                "requests"
            }
        }
    }
}
