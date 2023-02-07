use openai::Result;

#[tokio::main]
pub async fn main() -> Result<()> {
    let client = openai::client::OpenAIClient::new();
    let result = client.get_completions().await?;
    println!("Response text: {}", result);
    Ok(())
}
