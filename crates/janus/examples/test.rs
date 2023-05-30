use janus::*;

fn main() {
    let agent = AgentBuilder::new()
        .with_tool(Tool::new(
            "calculator",
            "Performs calculations",
            vec![
                ToolRunExample::new("1 + 1", "2"),
                ToolRunExample::new("2 + 2", "4"),
            ],
            ToolExecutor::Command("Perform the mathematical calculation".to_string()),
        ))
        .build();

    println!("{}", agent.prompt());
}
