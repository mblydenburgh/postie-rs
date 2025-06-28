use super::header::RequestHeaders;
use super::method::HttpMethod;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Tab {
    pub id: String,
    pub method: HttpMethod,
    pub url: String,
    pub req_body: String,
    pub req_headers: RequestHeaders,
    pub res_status: Option<String>,
    pub res_body: String,
    pub res_headers: RequestHeaders,
}
impl Default for Tab {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            url: "https://httpbin.org/json".into(),
            req_body: "some body".into(),
            req_headers: RequestHeaders(vec![]),
            method: HttpMethod::POST,
            res_status: None,
            res_body: "some response".into(),
            res_headers: RequestHeaders(vec![]),
        }
    }
}
