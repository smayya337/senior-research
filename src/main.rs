#![warn(clippy::pedantic)]

mod exec;
mod history;
mod parser;

use crate::exec::exec;
use crate::history::{cmd_time, write_history};
use crate::parser::separate;
use std::env;
use std::io::{stdin, stdout, Write};
use termion::input::TermRead;

use std::io;
use std::thread;
use std::time;

use termion;
use termion::cursor;
use termion::cursor::DetectCursorPos;
use termion::raw::IntoRawMode;

fn main() {
    loop {
        let stdin: Option<String> = new_read();
        if stdin.is_some() {
            let input = stdin.unwrap();
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

pub fn read() -> Option<String> {
    // let mut input: String = String::new();
    let prompt = prompt();
    print!("{}", prompt);
    let stdin = stdin();
    let mut stdin = stdin.lock();
    let input = stdin.read_line().unwrap().unwrap();
    return Some(input);
}

fn prompt() -> String {
    let username = users::get_current_username().unwrap();
    let host = hostname::get().unwrap();
    let homedir = home::home_dir()
        .expect("No home dir set!")
        .display()
        .to_string();
    let cwd_buf = env::current_dir().unwrap();
    let cwd = cwd_buf.to_str().unwrap().replace(&homedir, "~");
    format!(
        "{}@{} {} $ ",
        username.to_str().unwrap(),
        host.to_str().unwrap(),
        cwd
    )
}

fn new_read() -> Option<String> {
    // Set terminal to raw mode to allow reading stdin one key at a time
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    print!("{}", prompt());

    // Use asynchronous stdin
    let mut stdin = termion::async_stdin().keys();

    // Our string
    let mut cmd = String::new();

    loop {
        // Read input (if any)
        let input = stdin.next();

        // If a key was pressed
        if let Some(Ok(key)) = input {
            match key {
                // Exit if 'Ctrl+c' is pressed
                termion::event::Key::Ctrl('c') => break,
                // Quit everything if 'Ctrl+d' is pressed
                termion::event::Key::Ctrl('d') => return Some(String::from("exit")),
                // Clear screen if 'Ctrl-l' is pressed
                termion::event::Key::Ctrl('l') => {
                    write!(stdout, "{}", termion::clear::All);
                    stdout.lock().flush().unwrap();
                }
                // Return command if 'Enter' is pressed
                termion::event::Key::Char('\n') => {
                    write!(stdout, "\r\n");
                    stdout.suspend_raw_mode();
                    return Some(cmd);
                }
                termion::event::Key::Backspace => {
                    cmd.pop();
                    write!(stdout, "{}{}", " ", termion::cursor::Left(1));
                }
                termion::event::Key::Char(x) => {
                    cmd.push(x);
                    write!(stdout, "{}", x).unwrap();
                    stdout.lock().flush().unwrap();
                }
                _ => {}
            }
        }
        thread::sleep(time::Duration::from_millis(50));
    }
    None
}
