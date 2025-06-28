use crate::api::*;
use dioxus::prelude::*;

#[component]
pub fn BodyView() -> Element {
    let mut body_input_value = use_signal(|| String::new());
    let mut body_type = use_signal(|| "application/json");
    let mut res_body_value = use_signal(|| String::new());
    use_future(move || async move {
        if let Ok(tab) = load_active_tab().await {
            body_input_value.set(tab.req_body);
            res_body_value.set(tab.res_body);
        }
    });
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
                    id: "req_body",
                    onchange: move |event| body_input_value.set(event.value()),
                    value: "{body_input_value}"
                }
            }
            br {  }
            div {
                id: "body",
                textarea {
                    id: "res_body",
                    value: "{res_body_value}"
                }
            }
        }
    }
}
