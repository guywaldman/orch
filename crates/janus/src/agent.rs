use crate::{prompt::Prompt, tool::Tool, utils::format_list};

pub struct Agent {
    pub tools: Vec<Tool>,
    pub llm: Box<LlmFn>,
}

pub type LlmFn = dyn Fn(String) -> String;

pub struct AgentBuilder {
    tools: Vec<Tool>,
    llm: Option<Box<LlmFn>>,
}

impl Agent {
    pub fn new(llm: Box<LlmFn>) -> Self {
        Agent {
            tools: Vec::new(),
            llm: llm,
        }
    }

    pub fn start(self) -> ActiveAgent {
        ActiveAgent {
            agent: self,
            history: Vec::new(),
        }
    }
}

pub struct ActiveAgent {
    agent: Agent,
    history: Vec<String>,
}

enum AgentRunMessage {
    Thought(String),
    Observation(String),
    ToolRun(String),
    Message(String),
}

pub struct AgentRun {}

impl ActiveAgent {
    pub fn new(agent: Agent) -> Self {
        ActiveAgent {
            agent,
            history: Vec::new(),
        }
    }

    pub fn prompt(&self) -> String {
        self.agent.prompt()
    }

    pub async fn run(&mut self, input: &str) -> AgentRun {
        todo!();
    }
}

impl AgentBuilder {
    pub fn new() -> Self {
        AgentBuilder {
            tools: Vec::new(),
            llm: None,
        }
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
        prompt
    }
}
