#![warn(clippy::pedantic)]

mod exec;
mod history;
mod parser;

use crate::exec::exec;
use crate::history::{cmd_time, write_history};
use crate::parser::separate;
use std::env;
use std::io::Write;
use termion::input::TermRead;

use std::io;
use std::process::exit;
use std::thread;
use std::time;

use termion;
use termion::raw::IntoRawMode;

fn main() {
    // Set terminal to raw mode to allow reading stdin one key at a time
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", prompt()).unwrap();
    stdout.lock().flush().unwrap();

    // Use asynchronous stdin
    let mut stdin = termion::async_stdin().keys();

    // Our string
    let mut input = String::new();

    loop {
        // Read input (if any)
        let next = stdin.next();

        // If a key was pressed
        if let Some(Ok(key)) = next {
            match key {
                // Exit if 'Ctrl+c' is pressed
                termion::event::Key::Ctrl('c') => {
                    input = String::new();
                    write!(stdout, "\r\n").unwrap();
                    write!(stdout, "{}", prompt()).unwrap();
                }
                // Quit everything if 'Ctrl+d' is pressed
                termion::event::Key::Ctrl('d') => {
                    exit(0);
                }
                // Clear screen if 'Ctrl-l' is pressed
                termion::event::Key::Ctrl('l') => {
                    input = String::new();
                    write!(stdout, "{}{}\r", termion::clear::All, termion::cursor::Goto(1,1)).unwrap();
                    write!(stdout, "{}", prompt()).unwrap();
                }
                // Return command if 'Enter' is pressed
                termion::event::Key::Char('\n') => {
                    write!(stdout, "\r\n").unwrap();
                    stdout.suspend_raw_mode().unwrap();
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
                    input = String::new();
                    write!(stdout, "{}", prompt()).unwrap();
                    stdout.activate_raw_mode().unwrap();
                }
                termion::event::Key::Backspace => {
                    input.pop();
                    write!(stdout, "{}{}", " ", termion::cursor::Left(1)).unwrap();
                }
                termion::event::Key::Char(x) => {
                    input.push(x);
                    write!(stdout, "{}", x).unwrap();
                }
                _ => {}
            }
        }
        stdout.lock().flush().unwrap();
        thread::sleep(time::Duration::from_millis(50));
    }
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