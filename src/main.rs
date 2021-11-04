mod exec;
mod parser;
mod history;

use std::io::{self, stdin, Write};
use crate::parser::separate;
use crate::exec::exec;
use crate::history::{cmd_time, read_history, write_history};
use users;
use hostname;
use std::env;

fn main() {
    loop {
        let shell_history = read_history();
        let mut input: String = read();

        input.truncate(input.len() - 1);
        let time = cmd_time();
        let (cmd, vec) = separate(&input);
        let ecode = match cmd {
            Some(x) => exec(x, vec),
            None => 0,
        };
        if ecode == 127 {
            eprintln!("{}: command not found...", cmd.expect("This should not happen."));
        }
        write_history(time, &input);
    }
}

fn read() -> String {
    let mut input: String = String::new();
    prompt();
    stdin().read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn prompt() {
    let username = users::get_current_username().expect("ERROR");
    let host = hostname::get().expect("ERROR");
    let cwd_buf = env::current_dir().expect("ERROR");
    let cwd = cwd_buf.to_str().expect("ERROR");
    print!("{}@{} {} $ ", username.to_str().expect("ERROR"), host.to_str().expect("ERROR"), cwd);
    io::stdout().flush().unwrap();
}
