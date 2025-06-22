use dioxus::prelude::*;

pub fn BodyView() -> Element {
    rsx! {
        div {
            id: "body_view",
            style {
                border: "1px solid red"
            },
            div {
                id: "body_type",
                label {  "Body Type: " },
                select {
                    id: "body_select",
                    name: "body_type",
                    option {
                        value: "application/json",
                        "json"
                    },
                    option {
                        value: "form",
                        "form"
                    }
                }
            },
            div {
                id: "body",
                textarea {
                    id: "body_input"
                }
            }
        }
    }
}
