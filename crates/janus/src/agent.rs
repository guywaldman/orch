use openai::chat::{ChatCompletionMessageRole, ChatCompletionMessage};

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

    // TODO: Change to return a `Result`
    pub async fn run<'a>(self, task: &'a str) -> Option<RunResult> {
        let mut messages = vec![self.prompt(task)];

        for _ in 1..5 {
            let result = self.llm.complete(&messages).await;
            let result = result.split("\n\n").last().unwrap().to_owned();
            dbg!(&messages);
            dbg!(&result);
            let result: serde_json::Value = serde_json::from_str(&result).unwrap();
            if result["tool_name"].is_string() {
                let tool_name = result["tool_name"].to_string();
                let tool_name = tool_name.replace('\"', "");
                let input = result["input"].to_string();
                let tool_to_run = self.tools.iter().find(|tool| tool.name == tool_name);
                if let Some(tool_to_run) = tool_to_run {
                    let tool_result = tool_to_run.run(&input).unwrap();
                    messages.push(format!("Thought: I ran tool '{}' and got the result: {}\n\nThought: ", tool_name, tool_result));
                } else {
                    todo!("I don't know the tool '{}'.", tool_name);
                }
            } else if result["message"].is_string() {
                let message = result["message"].to_string();
                let message = message.replace('\"', "");
                return Some(RunResult {
                    output: message,
                });
            }
        }
        None
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

const COT: &str = 
    "You should follow the following format until completing the task you are given:

    Thought: <thought>
    Observation: <observation>

    <JSON object>

    Then, return a JSON object depending on what you wish to do.

    If you wish to run a tool, return the following JSON object:
    ```json
    {
        \"tool_name\": \"<tool name>\",
        \"input\": \"<tool input>\"
    }
    ```

    If you wish to send a message to the user (when you have completed the task), return the following JSON object:
    ```json
    {
        \"message\": \"<message>\"
    }
    ```

    In other words, you should start by thinking about the task you are given, make an observation, then (if needed) running a tool, and then sending a message to the user.
    If you need to run a tool, you should return before the 'Message' step.

    Important notes:
    - You should not apply your own judgement, **always use a tool**. Do not make any observations that are not based on a tool.
    - You should especially not return any facts, numbers or other information without applying the right tool.
    - You should not use any tools that are not provided to you.
    - You should not use any tools that are not relevant to the task you are given.
    - Stick only to the answer format above.

    For example:
    
    USER: What's 4+4?

    Thought: I need to calculate 4+4.
    Observation: I should use the tool 'calculator'.

    You should return:

    {
        \"tool_name\": \"calculator\",
        \"input\": \"4+4\"
    }

    Later, when you have the answer, you should return:

    {
        \"message\": \"8\"
    }

    This is the task/question:
";

impl Prompt for Agent {
    fn prompt(&self, input: &str) -> String {
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

        prompt.push_str(COT);
        prompt.push_str(format!("Task: {}\n", input).as_str());
        prompt.push_str("\n\n");
        prompt.push_str("Thought: ");
        prompt
    }
}
