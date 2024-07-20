//! A module containing all logic related to LLM execution.
//! An [`Executor`] is the terminology for a component which executes an LLM,
//! and aligns it appropriately (e.g., error correction).
//!
//! It is not to be confused with an [`Orchestrator`] which manages the execution of an LLM
//! or multiple LLMs towards a task.

mod builder;
mod executor;
mod response;

pub use builder::*;
pub use executor::*;
pub use response::*;
