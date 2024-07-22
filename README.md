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
cago add orch_response
```

Alternatively, add `orch as a dependency to your `Cargo.toml` file:

```toml
[dependencies]
orch = "*" # Substitute with the latest version
orch_response = "*" # Substitute with the latest version
```

# Basic Usage

## Simple Text Generation

```rust no_run
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

```rust no_run
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

```rust no_run
use orch::execution::*;
use orch::lm::*;
use orch::response::*;

#[derive(Variants, serde::Deserialize)]
pub enum ResponseVariants {
    Answer(AnswerResponseVariant),
    Fail(FailResponseVariant),
}

#[derive(Variant, serde::Deserialize)]
#[variant(
    variant = "Answer",
    scenario = "You know the capital city of the country",
    description = "Capital city of the country"
)]
pub struct AnswerResponseVariant {
    #[schema(
        description = "Capital city of the received country",
        example = "London"
    )]
    pub capital: String,
}

#[derive(Variant, serde::Deserialize)]
#[variant(
    variant = "Fail",
    scenario = "You don't know the capital city of the country",
    description = "Reason why the capital city is not known"
)]
pub struct FailResponseVariant {
    #[schema(
        description = "Reason why the capital city is not known",
        example = "Country 'foobar' does not exist"
    )]
    pub reason: String,
}

#[tokio::main]
async fn main() {
    let lm = OllamaBuilder::new().try_build().unwrap();
    let executor = StructuredExecutorBuilder::new()
    .with_lm(&lm)
    .with_preamble("You are a geography expert who helps users understand the capital city of countries around the world.")
		.with_options(Box::new(variants!(ResponseVariants)))
    .try_build()
    .unwrap();
    let response = executor
        .execute("What is the capital of Fooland?")
        .await
        .expect("Execution failed");

    println!("Response:");
    match response.content {
        ResponseVariants::Answer(answer) => {
            println!("Capital city: {}", answer.capital);
        }
        ResponseVariants::Fail(fail) => {
            println!("Model failed to generate a response: {}", fail.reason);
        }
    }
}
```

## Embedding Generation

```rust no_run
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
