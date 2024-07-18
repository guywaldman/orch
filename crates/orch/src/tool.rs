// use std::{collections::HashMap, fmt, fmt::Debug, future::Future, pin::Pin};

// pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

// use derive_builder::Builder;

// pub type ToolParams = HashMap<String, String>;

// #[derive(Debug, Clone)]
// pub struct ToolRunExample {
//     input: String,
//     output: String,
// }

// impl Into<ToolRunExample> for (String, String) {
//     fn into(self) -> ToolRunExample {
//         ToolRunExample {
//             input: self.0,
//             output: self.1,
//         }
//     }
// }

// impl ToolRunExample {
//     pub fn new(input: &str, output: &str) -> Self {
//         Self {
//             input: input.to_string(),
//             output: output.to_string(),
//         }
//     }
// }

// impl fmt::Display for ToolRunExample {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{} -> {}", self.input, self.output)
//     }
// }

// #[derive(Default, Builder)]
// #[builder(setter(into))]
// pub struct Tool {
//     pub name: String,
//     pub description: String,
//     pub examples: Vec<ToolRunExample>,
//     #[builder(default = "Vec::new()")]
//     pub parameter_names: Vec<String>,
//     #[builder(default = "Vec::new()")]
//     pub parameter_examples: Vec<Vec<String>>,
//     pub executor: Option<ToolExecutor>,
// }

// impl Debug for Tool {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "Tool {{ name: {}, description: {}, examples: {:?}, parameter_names: {:?}, parameter_examples: {:?} }}",
//             self.name,
//             self.description,
//             self.examples,
//             self.parameter_names,
//             self.parameter_examples
//         )
//     }
// }

// #[derive(Debug, Clone)]
// pub enum ToolExecutor {
//     Command(String),
//     Code(fn(ToolParams) -> BoxFuture<'static, Option<String>>),
// }

// impl Tool {
//     pub fn prompt(&self) -> String {
//         let mut prompt = String::new();
//         let title = format!("{}: {}", self.name, self.description);
//         prompt.push_str(&title);
//         prompt.push_str("\nParameters:\n");
//         for param_name in &self.parameter_names {
//             prompt.push_str(&format!(" - {}\n", param_name));
//         }
//         prompt.push_str("\nExample <tool_input>s:\n");
//         for param_examples in &self.parameter_examples {
//             let params: HashMap<String, String> = HashMap::from_iter(
//                 self.parameter_names
//                     .iter()
//                     .zip(param_examples.iter())
//                     .map(|(name, example)| (name.to_owned(), example.to_owned())),
//             );
//             prompt.push_str(&format!(" - {}\n", serde_json::to_string(&params).unwrap()));
//         }
//         prompt.push_str("\nExample Q&A:\n");

//         for ToolRunExample { input, output, .. } in &self.examples {
//             prompt.push_str(&format!(" - Input: {}, output: {}\n", input, output));
//         }
//         prompt
//     }

//     pub async fn run(&self, params: &ToolParams) -> Option<String> {
//         if let Some(executor) = &self.executor {
//             match executor {
//                 ToolExecutor::Command(_command) => {
//                     todo!("Implement running tools with commands.")
//                 }
//                 ToolExecutor::Code(function) => {
//                     // Remove quotes from input.
//                     let mut clean_params = HashMap::new();
//                     for (key, value) in params {
//                         let mut value = value.to_owned();
//                         if value.starts_with('"') && value.ends_with('"') {
//                             value = value[1..value.len() - 1].to_owned();
//                         }
//                         clean_params.insert(key.to_owned(), value);
//                     }

//                     (function)(clean_params).await
//                 }
//             }
//         } else {
//             todo!("Implement running tools without executors.");
//         }
//     }
// }
