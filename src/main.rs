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
        let shell_history = read_history();
        let stdin: Option<String> = new_read();
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
            write_history(time, &input);
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

fn new_read() -> Option<String> {
    // TODO: stop this from eating up first character of each line
    let mut stdout = stdout();
    let mut stdin = termion::async_stdin().keys();
    let mut cmd = String::new();
    prompt();
    loop {
        let input = stdin.next();
        if let Some(Ok(key)) = input {
            match key {
                // TODO: make this not need enter to work
                termion::event::Key::Ctrl('h') => println!("BRO"),
                termion::event::Key::Ctrl('c') => {
                    // TODO: stop this from exiting the whole program
                    stdout.flush().unwrap();
                    return None;
                }
                termion::event::Key::Char('\n') => break,
                termion::event::Key::Char(x) => {
                    println!("Typed key {}", x);
                    cmd.push(x);
                stdout.lock().flush().unwrap();
                },
                _ => (),
            }
        }
        thread::sleep(time::Duration::from_millis(50));
    }
    return Some(cmd);
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
