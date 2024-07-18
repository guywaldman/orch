// use std::collections::HashMap;

// use crate::{llm::TextCompletionLlm, prompt::Prompt, tool::Tool, utils::format_list};

// pub struct Agent {
//     pub tools: Vec<Tool>,
//     pub llm: Box<dyn TextCompletionLlm>,
// }

// pub struct AgentBuilder {
//     tools: Vec<Tool>,
//     llm: Option<Box<dyn TextCompletionLlm>>,
// }

// // TODO: Change into a result graph.
// #[derive(Debug)]
// pub struct RunResult {
//     pub output: String,
// }

// impl Agent {
//     pub fn new(llm: Box<dyn TextCompletionLlm>) -> Self {
//         Agent {
//             tools: Vec::new(),
//             llm,
//         }
//     }

//     // TODO: Change to return a `Result`
//     pub async fn run<'a>(self, task: &'a str) -> Option<RunResult> {
//         let mut messages = vec![self.prompt(task)];

//         for _ in 1..5 {
//             let result = self.llm.complete(&messages).await.unwrap();
//             let result = result.split('\n').last().unwrap().to_owned();
//             if !result.starts_with("Action: ") {
//                 return None;
//             }
//             let action_str = result.trim_start_matches("Action: ");

//             let result: serde_json::Value = serde_json::from_str(action_str).unwrap();
//             if result["tool_name"].is_string() {
//                 let tool_name = result["tool_name"].to_string();
//                 let tool_name = tool_name.replace('\"', "");
//                 let mut params = HashMap::new();
//                 for (key, value) in result["input"].as_object().unwrap() {
//                     params.insert(key.to_owned(), value.to_string());
//                 }
//                 let tool_to_run = self.tools.iter().find(|tool| tool.name == tool_name);
//                 if let Some(tool_to_run) = tool_to_run {
//                     let tool_result = tool_to_run.run(&params).await.unwrap();
//                     messages.push(format!("Observation: I ran tool '{}' and got the result: {}\n\nThought: ", tool_name, tool_result));
//                 } else {
//                     todo!("I don't know the tool '{}'.", tool_name);
//                 }
//             } else if result["message"].is_string() {
//                 let message = result["message"].to_string();
//                 let message = message.replace('\"', "");
//                 return Some(RunResult {
//                     output: message,
//                 });
//             }
//         }
//         None
//     }
// }

// pub struct AgentRun {}

// impl AgentBuilder {
//     pub fn new() -> Self {
//         Self {
//             tools: Vec::new(),
//             llm: None,
//         }
//     }

//     pub fn with_llm(mut self, llm: Box<dyn TextCompletionLlm>) -> Self {
//         self.llm = Some(llm);
//         self
//     }

//     pub fn with_tool(mut self, tool: Tool) -> Self {
//         self.tools.push(tool);
//         self
//     }

//     pub fn build(self) -> Agent {
//         Agent {
//             tools: self.tools,
//             llm: self.llm.unwrap(),
//         }
//     }
// }

// impl Default for AgentBuilder {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// const HELPFUL_AGENT_PREFACE: &str =
//     "You are an helpful agent, designed to answer the following questions as best you can.";

// const COT: &str = 
//     "You should follow the following format until completing the task you are given:

//     Thought: <thought>
//     Observation: <observation>
//     Action: <action>

//     Where <action> is either a tool or a message (a JSON object).

//     If you wish to run a tool, <action> should look like: { \"tool_name\": \"<tool_name>\", \"input\": \"<tool_input>\" }
//     Where <tool_name> is the name of the tool you wish to run, and <tool_input> is the JSON input you wish to give to the tool.
//     <tool_input> should be a JSON and start with a '{' and end with a '}'.
//     If you wish to return a final answer, <action> should look like: { \"message\": \"<message>\" }

//     Thought: <thought>
//     Observation: <observation>
//     Action: <action>

//     ...where <action> is either a tool or a message (a JSON object).

//     In other words, you should start by thinking about the task you are given, make an observation, then (if needed) running a tool, and then sending a message to the user.

//     Important notes:
//     - Stick to the above format only, and you should *always* return \"Action: <action>\" at the end, where <action> is either a tool or a message (a JSON object).
//     - You should not apply your own judgement, **always use a tool**. Do not make any observations that are not based on a tool.
//     - You should especially not return any facts, numbers or other information without applying the right tool.
//     - You should not use any tools that are not provided to you.
//     - You should not use any tools that are not relevant to the task you are given.
//     - When you think you have the answer, stop! Return it to the user (in a JSON!) and phrase it in a concise and informative manner in a JSON object with a 'message' field. Simply return a JSON with the 'message' field.
//     - You should never return a result that is not a JSON object!
//     - Stick only to the answer format above.

//     For example:
    
//     USER: What's 4+4?

//     ---
//     Thought: I need to calculate 4+4.
//     Observation: I should use the tool 'calculator'.
//     Action: { \"tool_name\": \"calculator\", \"input\": { \"query\": \"4+4\" } }
//     ---

//     ---
//     Thought: I ran the tool 'calculator' and got the result: 8.
//     Observation: I have the final answer, I can now return the answer to the user.
//     Action: { \"message\": \"The answer is 8.\" }
//     ---

//     In the above examples, the first JSON you return is the one that tells the user which tool to run.
//     The second JSON you return is the one that tells the user the result of the tool.

//     Remember, *ALWAYS* end with \"Action: <action>\" where <action> is either a tool or a message (a JSON object).
//     Make the JSON valid, and make sure it has the right fields.
//     The JSON should end with '{', end with '}', and have a comma between each field.
//     Each field should be in the format \"<field>\": \"<value>\", where <field> is the name of the field, and <value> is the value of the field.
//     Same JSON validity rules apply to <tool_input>.

//     This is the task/question:
// ";

// impl Prompt for Agent {
//     fn prompt(&self, input: &str) -> String {
//         let mut prompt = String::new();
//         let preface = HELPFUL_AGENT_PREFACE;
//         prompt.push_str(&format!("{}\n", preface));

//         if !self.tools.is_empty() {
//             prompt.push_str("You have access to the following tools:\n");
//             prompt.push_str(
//                 format_list(
//                     &self
//                         .tools
//                         .iter()
//                         .map(|tool| {
//                             tool.prompt()
//                                 .lines()
//                                 .map(|line| format!("  {}", line))
//                                 .collect::<Vec<String>>()
//                                 .join("\n")
//                         })
//                         .collect::<Vec<String>>(),
//                     2,
//                 )
//                 .as_str(),
//             );
//         }

//         prompt.push_str(COT);
//         prompt.push_str(format!("Task: {}\n", input).as_str());
//         prompt.push_str("\n\n");
//         prompt.push_str("Thought: ");
//         prompt
//     }
// }
