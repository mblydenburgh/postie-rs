use dioxus::prelude::*;

use crate::api::get_tab;
use crate::models::method::HttpMethod;

#[component]
pub fn RequestBar() -> Element {
    let mut url_input_value = use_signal(|| String::new());
    let mut http_method = use_signal(|| HttpMethod::GET);
    use_future(move || async move {
        if let Ok(tab) = get_tab().await {
            url_input_value.set(tab.url);
            http_method.set(tab.method);
        }
    });
    rsx! {
        div {
            id: "request_bar",
            select {
                id: "method_select",
                name: "method",
                value: "{http_method}",
                option {
                    value: "GET",
                    "GET",
                },
                option {
                    value: "POST",
                    "POST"
                }
            },
            input {
                oninput: move |event| url_input_value.set(event.value().clone()),
                id: "url_bar",
                name: "url",
                type: "text",
                value: "{url_input_value}"
            }
        }
    }
}
