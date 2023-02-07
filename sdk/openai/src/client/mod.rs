use super::Result;

#[cfg(feature="blocking")]
pub mod blocking;

pub async fn get_completions() -> Result<()> {
    Ok(())
}
