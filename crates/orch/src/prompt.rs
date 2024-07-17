pub trait Prompt {
    fn prompt(&self, input: &str) -> String;
}
