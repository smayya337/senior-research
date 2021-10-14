pub fn separate(input: &str) -> Vec<String> {
    let mut single_quote: bool = false;
    let mut double_quote: bool = false;
    let mut backslash: bool = false;
    let mut space: bool = false;
    let mut dollar: bool = false;
    let mut paren: bool = false;
    let mut backtick: bool = false;
    let mut newline: bool = false;
    let mut command_parts: Vec<String> = Vec::new();
    for c in input.chars() {
        println!("{}", c);
        match c {
            '\'' => println!("Single quote!"),
            '"' => println!("Double quote!"),
            '\\' => println!("Backslash!"),
            ' ' => println!("Space!"),
            '$' => println!("Dollar!"),
            '(' | ')' => println!("Parenthesis!"),
            '`' => println!("Backtick!"),
            '\n' => println!("New line!"),
            _ => println!("Nothing special!")
        }
    }
    command_parts
}