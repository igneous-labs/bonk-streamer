pub mod config;
pub mod streamer;
pub mod utils;

mod accounts;
mod consts;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
