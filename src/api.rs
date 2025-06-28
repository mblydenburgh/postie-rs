use dioxus::prelude::*;

// #[cfg(feature = "server")]
// thread_local! {
//     pub static DB: rusqlite::Connection = {
//         let conn = rusqlite::Connection::open("postie.db").expect("Failed to open db");
// TODO: execute migrations

// conn
// };
// }

// TODO - fix lsp issues when using server macro
//#[server]
pub async fn get_tabs() -> Result<Vec<crate::models::tab::Tab>, ServerFnError> {
    let tabs: Vec<crate::models::tab::Tab> = vec![
        crate::models::tab::Tab::default(),
        crate::models::tab::Tab::default(),
    ];
    Ok(tabs)
}

//#[server]
pub async fn load_active_tab() -> Result<crate::models::tab::Tab, ServerFnError> {
    let tab = crate::models::tab::Tab::default();
    Ok(tab)
}

//#[server]
pub async fn remove_tab(id: &String) -> Result<Vec<crate::models::tab::Tab>, ServerFnError> {
    if let Ok(tabs) = get_tabs().await {
        Ok(tabs.into_iter().filter(|t| t.id == id.clone()).collect())
    } else {
        Err(ServerFnError::ServerError(String::from(
            "could not remove tab",
        )))
    }
}
