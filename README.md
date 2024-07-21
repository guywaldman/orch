# orch

![Crates.io Version](https://img.shields.io/crates/v/orch?link=https%3A%2F%2Fcrates.io%2Fcrates%2Forch)
![Crates.io Total Downloads](https://img.shields.io/crates/d/orch?link=https%3A%2F%2Fcrates.io%2Fcrates%2Forch)

`orch` is a library for building language model powered applications and agents for the Rust programming language.
It was primarily built for usage in [magic-cli](https://github.com/guywaldman/magic-cli), but can be used in other contexts as well.

> [!NOTE]
>
> If the project gains traction, this can be compiled as an addon to other languages such as Python or a standalone WebAssembly module.

# Installation

```shell
cargo add orch
```

Alternatively, add `orch as a dependency to your `Cargo.toml` file:

```toml
[dependencies]
orch = "0.0.4"
```

# Basic Usage

## Simple Text Generation

```rust
use orch::execution::*;
use orch::lm::*;

#[tokio::main]
async fn main() {
  let lm = OllamaBuilder::new().try_build().unwrap();
  let executor = TextExecutorBuilder::new().with_lm(&lm).try_build().unwrap();
  let response = executor.execute("What is 2+2?").await.expect("Execution failed");
  println!("{}", response.content);
}
```

## Streaming Text Generation

```rust
use orch::execution::*;
use orch::lm::*;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
  let lm = OllamaBuilder::new().try_build().unwrap();
  let executor = TextExecutorBuilder::new().with_lm(&lm).try_build().unwrap();
  let mut response = executor.execute_stream("What is 2+2?").await.expect("Execution failed");
  while let Some(chunk) = response.stream.next().await {
    match chunk {
      Ok(chunk) => print!("{chunk}"),
      Err(e) => {
        println!("Error: {e}");
        break;
      }
    }
  }
  println!();
}
```

## Structured Data Generation

```rust
use orch::execution::*;
use orch::lm::*;
use orch_response_derive::*;

#[derive(OrchResponseOptions)]
pub enum CapitalCityExecutorResponseOptions {
    #[response(
        scenario = "You know the capital city of the country",
        description = "Capital city of the country"
    )]
    #[schema(
        field = "capital",
        description = "Capital city of the received country",
        example = "London"
    )]
    Answer { capital: String },
    #[response(
        scenario = "You don't know the capital city of the country",
        description = "Reason why the capital city is not known"
    )]
    #[schema(
        field = "reason",
        description = "Reason why the capital city is not known",
        example = "Country 'foobar' does not exist"
    )]
    Fail { reason: String },
}

#[tokio::main]
async fn main() {
  let lm = OllamaBuilder::new().try_build().unwrap();
  let executor = StructuredExecutorBuilder::new()
    .with_lm(&lm)
    .with_preamble("You are a geography expert who helps users understand the capital city of countries around the world.")
    .with_options(&options!(CapitalCityExecutorResponseOptions))
    .try_build()
    .unwrap();
  let response = executor.execute("What is the capital of Fooland?").await.expect("Execution failed");

  println!("Response:");
  println!("{:?}", response.content);
}
```

## Embedding Generation

```rust
use orch::execution::*;
use orch::lm::*;

#[tokio::main]
async fn main() {
  let lm = OllamaBuilder::new().try_build().unwrap();
  let executor = TextExecutorBuilder::new()
    .with_lm(&lm)
    .try_build()
    .unwrap();
  let embedding = executor
    .generate_embedding("Phrase to generate an embedding for")
    .await
    .expect("Execution failed");

  println!("Embedding:");
  println!("{:?}", embedding);
}
```

## More Examples

See the [examples](https://github.com/guywaldman/orch/tree/main/core/examples) directory for usage examples.

## Roadmap

- [ ] Agents and tools
