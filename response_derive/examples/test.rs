// use orch_response::ExecutorResponseOptions;

use orch_response_derive::{options, OrchResponseOptions};

#[derive(OrchResponseOptions)]
pub enum CapitalCityExecutorResponseOptions {
    #[response(scenario = "You know the capital city of the country", description = "Capital city of the country")]
    #[schema(field = "capital", description = "Capital city of the received country", example = "London")]
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

fn main() {
    let _options = options!(CapitalCityExecutorResponseOptions);
}
