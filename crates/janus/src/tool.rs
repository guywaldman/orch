use std::{fmt, fmt::Debug};

#[derive(Debug)]
pub struct ToolRunExample {
    input: String,
    output: String,
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

pub struct ToolExecutorFn(Box<dyn Fn(dyn fmt::Display) -> dyn fmt::Display>);

impl fmt::Debug for ToolExecutorFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ToolExecutorFn")
    }
}

pub struct Tool {
    pub name: String,
    pub description: String,
    pub examples: Vec<ToolRunExample>,
    pub executor: ToolExecutor,
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

pub enum ToolExecutor {
    Command(String),
    Function(ToolExecutorFn),
}

impl Tool {
    pub fn new(
        name: &str,
        description: &str,
        examples: Vec<ToolRunExample>,
        executor: ToolExecutor,
    ) -> Self {
        Tool {
            name: name.to_string(),
            description: description.to_string(),
            examples,
            executor,
        }
    }

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
}
