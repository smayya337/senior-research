mod exec;
mod parser;

use std::io::stdin;
use crate::parser::separate;
use crate::exec::exec;

fn main() {
    info();
    loop {
        let mut input: String = read();

        input.truncate(input.len() - 1);
        let (cmd, vec) = separate(&input);
        let success = match cmd {
            Some(x) => exec(x, vec),
            None => true,
        };
        if !success {
            eprintln!("{}: command not found...", cmd.expect("This should not happen."));
        }
    }
}

fn read() -> String {
    let mut input: String = String::new();
    stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");
    input
}

fn info() {
    let name = env!("CARGO_PKG_NAME");
    let desc = env!("CARGO_PKG_DESCRIPTION");
    let version = env!("CARGO_PKG_VERSION");
    let authors = str::replace(env!("CARGO_PKG_AUTHORS"), ":", ", ");
    let license = env!("CARGO_PKG_LICENSE");
    let repo = env!("CARGO_PKG_REPOSITORY");
    println!("{}", name);
    println!("{}", desc);
    println!("Version {}", version);
    println!("By {}", authors);
    println!("Available at {} under the {} license.", repo, license);
}
