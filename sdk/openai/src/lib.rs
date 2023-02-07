pub mod client;
pub mod models;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
