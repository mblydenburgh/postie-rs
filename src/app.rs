#![allow(non_snake_case)]

use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

use crate::components::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

static CSS: Asset = asset!("/assets/main.css");

pub fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: CSS }
        main {
            div {
                id: "app",
                TopBar{},
                div {
                    id: "main_view",
                        SideBar{},
                        ContentView{}
                }
            }
        }
    }
}
