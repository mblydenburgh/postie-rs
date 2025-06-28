mod api;
mod app;
mod components;
mod models;

use app::App;
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}
