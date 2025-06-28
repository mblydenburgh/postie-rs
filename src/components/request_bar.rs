use dioxus::prelude::*;
use log::info;

use crate::api::load_active_tab;
use crate::models::method::HttpMethod;

#[component]
pub fn RequestBar() -> Element {
    // let mut active_tab = use_resource(load_active_tab);
    // let active_tab_signal = active_tab.suspend()?;
    let mut url_input_value = use_signal(|| String::new());
    let mut http_method = use_signal(|| HttpMethod::GET);
    // use_effect(move || {
    //     let tab = active_tab_signal.unwrap();
    //     info!("{:?}", &tab);
    //     url_input_value.set(active_tab_signal().unwrap().url);
    //     http_method.set(active_tab_signal().unwrap().method);
    // });
    use_future(move || async move {
        if let Ok(tab) = load_active_tab().await {
            info!("tab: {:?}", tab);
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
            p { "tab: {url_input_value}" }
        }
    }
}
