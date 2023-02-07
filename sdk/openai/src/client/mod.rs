use super::Result;
#[cfg(feature="blocking")]
pub mod blocking;

pub struct OpenAIClient(reqwest::Client);

impl OpenAIClient {

    pub fn new() -> OpenAIClient {
        let client = reqwest::Client::builder()
            // .user_agent()
            .build()
            .expect("Error building HTTP client");
        OpenAIClient(client)
    }

    pub async fn get_completions(&self) -> Result<String> {
        let completion_text = self.0.get("https://www.rust-lang.org")
            .send()
            .await?
            .text()
            .await?;
        Ok(completion_text)
    }
}
