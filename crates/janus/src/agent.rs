use crate::{llm::TextCompletionLlm, prompt::Prompt, tool::Tool, utils::format_list};

pub struct Agent {
    pub tools: Vec<Tool>,
    pub llm: Box<dyn TextCompletionLlm>,
}

pub struct AgentBuilder {
    tools: Vec<Tool>,
    llm: Option<Box<dyn TextCompletionLlm>>,
}

// TODO: Change into a result graph.
#[derive(Debug)]
pub struct RunResult {
    pub output: String,
}

impl Agent {
    pub fn new(llm: Box<dyn TextCompletionLlm>) -> Self {
        Agent {
            tools: Vec::new(),
            llm: llm,
        }
    }

    pub async fn run<'a>(self, task: &'a str) -> RunResult {
        let result = self.llm.complete(task).await;
        RunResult { output: result }
    }
}

enum AgentRunMessage {
    Thought(String),
    Observation(String),
    ToolRun(String),
    Message(String),
}

pub struct AgentRun {}

impl AgentBuilder {
    pub fn new() -> Self {
        Self {
            tools: Vec::new(),
            llm: None,
        }
    }

    pub fn with_llm(mut self, llm: Box<dyn TextCompletionLlm>) -> Self {
        self.llm = Some(llm);
        self
    }

    pub fn with_tool(mut self, tool: Tool) -> Self {
        self.tools.push(tool);
        self
    }

    pub fn build(self) -> Agent {
        Agent {
            tools: self.tools,
            llm: self.llm.unwrap(),
        }
    }
}

const HELPFUL_AGENT_PREFACE: &str =
    "You are an helpful agent, designed to answer the following questions as best you can.";

impl Prompt for Agent {
    fn prompt(&self) -> String {
        let mut prompt = String::new();
        let preface = HELPFUL_AGENT_PREFACE;
        prompt.push_str(&format!("{}\n", preface));

        if !self.tools.is_empty() {
            prompt.push_str("You have access to the following tools:\n");
            prompt.push_str(
                &format_list(
                    &self
                        .tools
                        .iter()
                        .map(|tool| {
                            tool.prompt()
                                .lines()
                                .map(|line| format!("  {}", line))
                                .collect::<Vec<String>>()
                                .join("\n")
                        })
                        .collect::<Vec<String>>(),
                    2,
                )
                .as_str(),
            );
        }

        prompt
    }
}
