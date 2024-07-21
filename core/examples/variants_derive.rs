//! This example demonstrates how to use the `Variants` derive macro to generate a structured response from the LLM.
//!
//! Run like so: `cargo run --example variants_derive`

use orch::response::*;

#[derive(Variants, serde::Deserialize)]
pub enum ResponseOptions {
    Answer(AnswerResponseOption),
    Fail(FailResponseOption),
}

#[derive(Variant, serde::Deserialize)]
#[variant(
    variant = "Answer",
    scenario = "You know the capital city of the country",
    description = "Capital city of the country"
)]
pub struct AnswerResponseOption {
    #[schema(
        description = "Capital city of the received country",
        example = "London"
    )]
    pub capital: String,
    #[schema(
        description = "Country of the received capital city",
        example = "United Kingdom"
    )]
    pub country: String,
}

#[derive(Variant, serde::Deserialize)]
#[variant(
    variant = "Fail",
    scenario = "You don't know the capital city of the country",
    description = "Reason why the capital city is not known"
)]
pub struct FailResponseOption {
    #[schema(
        description = "Reason why the capital city is not known",
        example = "Country 'foobar' does not exist"
    )]
    pub reason: String,
}

fn main() {
    let response = r#"
        {
            "response_type": "Answer",
            "capital": "London",
            "country": "United Kingdom"
        }
    "#;
    let parsed_response = variants!(ResponseOptions).parse(response).unwrap();
    match parsed_response {
        ResponseOptions::Answer(answer_response) => {
            println!("{}", answer_response.capital);
        }
        ResponseOptions::Fail(fail_response) => {
            println!("{}", fail_response.reason);
        }
    }
}
