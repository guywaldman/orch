use openai::completions::*;

pub async fn complete(prompt: &str) -> String {
    let completion = Completion::builder("text-davinci-003")
        .prompt(&prompt.to_string())
        .max_tokens(1024)
        .create()
        .await
        .unwrap()
        .unwrap();

    let response = completion.choices.first().unwrap().text.to_string();
    response
}
