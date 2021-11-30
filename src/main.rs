#![warn(clippy::pedantic)]

mod exec;
mod history;
mod parser;

use crate::exec::exec;
use crate::history::{cmd_time, read_history, write_history};
use crate::parser::separate;
use std::{env, thread, time};
use std::ffi::OsString;
use std::io::{self, stdin, stdout, Write};
use std::process::exit;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    loop {
        let stdin: Option<String> = read();
        // new_read();
        if stdin.is_some() {
            let mut input = stdin.unwrap();
            let time = cmd_time();
            let (cmd, vec) = separate(&input);
            let ecode = match cmd {
                Some(x) => exec(x, vec),
                None => 0,
            };
            if ecode == 127 {
                eprintln!("{}: command not found...", cmd.unwrap());
            }
            if cmd.is_some() && cmd.unwrap().ne("history") {
                write_history(time, &input);
            }
        }
    }
}

fn read() -> Option<String> {
    let mut input: String = String::new();
    prompt();
    let mut stdin = stdin();
    let mut stdin = stdin.lock();
    input = stdin.read_line().unwrap().unwrap();
    return Some(input);
}

fn prompt() {
    let username = users::get_current_username().unwrap();
    let host = hostname::get().unwrap();
    let homedir = home::home_dir()
        .expect("No home dir set!")
        .display()
        .to_string();
    let cwd_buf = env::current_dir().unwrap();
    let cwd = cwd_buf.to_str().unwrap().replace(&homedir, "~");
    let stdout = stdout();
    let mut stdout = stdout.lock();
    print!(
        "{}@{} {} $ ",
        username.to_str().unwrap(),
        host.to_str().unwrap(),
        cwd
    );
    stdout.flush().unwrap();
}
