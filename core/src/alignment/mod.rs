//! A module containing all logic related to alignment.
//! Alignment, in this context, means "aligning" the model's output with the desired output.
//! This takes the form of a so-called `[AlignmentStrategy]`, which is a trait that defines how to align the model's output.
//!
//! This concept has similarities to to traditional "resilience" techniques and libraries, such as .NET's [Polly](https://github.com/App-vNext/Polly),
//! which I personally like a lot.

mod strategy;
mod strategy_builder;

pub use strategy::*;
pub use strategy_builder::*;
