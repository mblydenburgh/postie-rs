use dioxus::prelude::*;

pub fn RequestBar() -> Element {
    rsx! {
        div {
            id: "request_bar",
            select {
                id: "method_select",
                name: "method",
                option {
                    value: "get",
                    "GET"
                },
                option {
                    value: "post",
                    "POST"
                }
            },
            input {
                id: "url_bar",
                name: "url",
                type: "text"
            }
        }
    }
}
