pub fn separate(input: &str) -> Vec<&str> {
    let split = input.split_whitespace();
    let command_parts: Vec<&str> = Vec::new();
    for s in split {
        println!("{}", s);
    }
    command_parts
}