#[cfg(feature="blocking")]
pub struct OpenAIClient(reqwest::blocking::Client);
