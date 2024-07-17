pub fn format_list(list: &[String], padding: usize) -> String {
    let mut formatted_list = String::new();
    let padding = " ".repeat(padding);
    for (i, item) in list.iter().enumerate() {
			let lines = item.lines().skip(1).map(|line| format!("{}{}", padding, line)).collect::<Vec<String>>();
			let first_line = item.lines().next().unwrap();
			formatted_list.push_str(&format!("{}{}.{}\n", padding, i+1, first_line));
			for line in lines {
				formatted_list.push_str(&format!("{}{}\n", padding, line));
			}
    }
    formatted_list
}
