#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

pub mod alignment;
pub mod execution;
pub mod lm;
mod net;
pub mod response;
