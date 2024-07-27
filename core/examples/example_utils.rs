use orch::lm::{
    AnthropicBuilder, LanguageModel, LanguageModelBuilder, LanguageModelProvider, OllamaBuilder,
    OpenAiBuilder,
};

pub fn get_lm() -> (Box<dyn LanguageModel>, LanguageModelProvider) {
    let args = std::env::args().collect::<Vec<_>>();
    let provider_name = args.get(1).unwrap_or_else(|| {
        eprintln!("ERROR: Please provide a provider name");
        std::process::exit(1);
    });
    let provider = LanguageModelProvider::try_from(provider_name.as_str())
        .expect("Invalid provider name. Supported values: 'ollama', 'openai', 'anthropic'");

    let open_ai_api_key = {
        if provider == LanguageModelProvider::OpenAi {
            std::env::var("OPENAI_API_KEY")
                .unwrap_or_else(|_| panic!("OPENAI_API_KEY environment variable not set"))
        } else {
            String::new()
        }
    };
    let anthropic_api_key = {
        if provider == LanguageModelProvider::Anthropic {
            std::env::var("ANTHROPIC_API_KEY")
                .unwrap_or_else(|_| panic!("ANTHROPIC_API_KEY environment variable not set"))
        } else {
            String::new()
        }
    };
    let lm: Box<dyn LanguageModel> = match provider {
        LanguageModelProvider::Ollama => Box::new(OllamaBuilder::new().try_build().unwrap()),
        LanguageModelProvider::OpenAi => Box::new(
            OpenAiBuilder::new()
                .with_api_key(open_ai_api_key)
                .try_build()
                .unwrap(),
        ),
        LanguageModelProvider::Anthropic => Box::new(
            AnthropicBuilder::new()
                .with_api_key(anthropic_api_key)
                .try_build()
                .unwrap(),
        ),
    };

    (lm, provider)
}

#[allow(dead_code)]
fn main() {}
