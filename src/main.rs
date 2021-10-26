mod exec;
mod parser;

use std::io::{self, stdin, Write};
use crate::parser::separate;
use crate::exec::exec;
use users;
use hostname;
use std::env;

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
    prompt();
    stdin().read_line(&mut input)
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

fn prompt() {
    let username = users::get_current_username().expect("ERROR");
    let host = hostname::get().expect("ERROR");
    let cwd_buf = env::current_dir().expect("ERROR");
    let cwd = cwd_buf.to_str().expect("ERROR");
    print!("{}@{} {} $ ", username.to_str().expect("ERROR"), host.to_str().expect("ERROR"), cwd);
    io::stdout().flush().unwrap();
}
