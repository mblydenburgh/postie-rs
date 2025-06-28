use crate::models::tab::Tab;
use dioxus::prelude::*;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("postie.db").expect("Failed to open db");
        // TODO: execute migrations

        conn
    };
}

#[server]
pub async fn get_url() -> Result<String, ServerFnError> {
    Ok("https://httpbin.org/json".to_string())
}

#[server]
pub async fn get_tab() -> Result<Tab, ServerFnError> {
    Ok(Tab::default())
}
