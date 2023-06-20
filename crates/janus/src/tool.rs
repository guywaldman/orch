use std::{fmt, fmt::Debug};

use derive_builder::Builder;

#[derive(Debug, Clone)]
pub struct ToolRunExample {
    input: String,
    output: String,
}

impl Into<ToolRunExample> for (String, String) {
    fn into(self) -> ToolRunExample {
        ToolRunExample {
            input: self.0,
            output: self.1,
        }
    }
}

impl ToolRunExample {
    pub fn new(input: &str, output: &str) -> Self {
        ToolRunExample {
            input: input.to_string(),
            output: output.to_string(),
        }
    }
}

impl fmt::Display for ToolRunExample {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.input, self.output)
    }
}

#[derive(Default, Builder)]
#[builder(setter(into))]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub examples: Vec<ToolRunExample>,
    pub executor: Option<ToolExecutor>,
}

impl Debug for Tool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Tool {{ name: {}, description: {}, examples: {:?} }}",
            self.name, self.description, self.examples
        )
    }
}

#[derive(Debug, Clone)]
pub enum ToolExecutor {
    Command(String),
    Function(fn(&str) -> Option<String>),
}

impl Tool {
    pub fn prompt(&self) -> String {
        let mut prompt = String::new();
        let title = format!("{}: {}", self.name, self.description);
        prompt.push_str(&title);
        prompt.push_str("\nFor example:\n");
        for ToolRunExample { input, output, .. } in &self.examples {
            prompt.push_str(&format!(" - Input: {}, output: {}\n", input, output));
        }
        prompt
    }

    pub fn run(&self, input: &str) -> Option<String> {
        if let Some(executor) = &self.executor {
            match executor {
                ToolExecutor::Command(command) => {
                    todo!("Implement running tools with commands.")
                }
                ToolExecutor::Function(function) => {
                    let output = (function)(input);
                    output
                }
            }
        } else {
            todo!("Implement running tools without executors.");
        }
    }
}
