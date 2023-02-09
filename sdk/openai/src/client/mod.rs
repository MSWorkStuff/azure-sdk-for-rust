use std::sync::Arc;

use azure_core::{HttpClient, Request, Url, Method};
use super::Result;
#[cfg(feature="blocking")]
pub mod blocking;

pub struct OpenAIClient(Arc<dyn HttpClient>);

impl OpenAIClient {

    pub fn new() -> OpenAIClient {
        let client = azure_core::new_http_client();
        OpenAIClient(client)
    }

    pub async fn authenticate(&self) {}

    pub async fn get_completions(&self) -> Result<String> {
        let request = Request::new(Url::parse("https://www.rust-lang.org")?, Method::Get);

        let completion_text = self.0.execute_request(&request)
            .await?
            .into_body()
            .collect()
            .await?;
        let completion_text = String::from_utf8(completion_text.into())?; // use serde::from_slice
        Ok(completion_text)
    }
}
