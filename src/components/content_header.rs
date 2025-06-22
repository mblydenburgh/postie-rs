use crate::components::*;
use dioxus::prelude::*;

pub fn ContentHeader() -> Element {
    rsx! {
        div {
            id: "content_header",
            button {
               id: "request_button",
               "Request"
            },
            button {
                id: "auth_button",
                "Authorization"
            },
            button {
                id: "header_button",
                "Headers"
            }
        }
    }
}
