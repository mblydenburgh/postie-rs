use crate::api::*;
use dioxus::prelude::*;

#[component]
pub fn TopBar() -> Element {
    let mut tabs = use_resource(get_tabs);
    let tabs_signal = tabs.suspend()?;

    let header = rsx! {
        div {
            id: "top_bar",
            style {
                display: "flex"
            }
            button {
                id: "new_button",
                "+"
            }
        }
    };

    let tab_list = tabs_signal()
        .unwrap()
        .iter()
        .map(|tab| {
            let tab_id = tab.id.clone();
            rsx! {
                div {
                    id: format!("tab_container_{}", tab_id),
                    style { display: "flex" },
                    button {
                        id: format!("close_{}", tab_id),
                        onclick: move |_| {
                            let id = tab_id.clone();
                            async move {
                            remove_tab(&id).await.unwrap();
                            tabs.restart();
                            }
                        },
                        "x"
                    },
                    button {
                        id: format!("tab_{}", tab.id),
                        "{tab.url}"
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    rsx! {
        {header}
        {tab_list.into_iter()}
    }
}
