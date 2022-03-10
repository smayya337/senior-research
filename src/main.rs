#![warn(clippy::pedantic)]

mod exec;
mod history;
mod parser;

use std::cmp::min;
use crate::exec::exec;
use crate::history::{cmd_time, read_history, write_history};
use crate::parser::separate;
use std::env;
use std::io::Write;
use termion::input::{MouseTerminal, TermRead};

use std::io;
use std::process::exit;
use std::thread;
use std::time;

use termion;
use termion::raw::IntoRawMode;

fn main() {
    let mut history = read_history();
    let mut history_pos = history.len();
    let mut beginning_of_line = prompt().len() as u16;
    let mut horiz_pos = beginning_of_line + 1;
    // Set terminal to raw mode to allow reading stdin one key at a time
    let mut stdout = MouseTerminal::from(io::stdout()).into_raw_mode().unwrap();
    write!(stdout, "{}", prompt()).unwrap();
    stdout.lock().flush().unwrap();

    // Use asynchronous stdin
    let mut stdin = termion::async_stdin().keys();

    // Our string
    let mut input = String::new();
    let mut length_of_line = beginning_of_line as u16;

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
                    write!(
                        stdout,
                        "{}{}\r",
                        termion::clear::All,
                        termion::cursor::Goto(1, 1)
                    )
                    .unwrap();
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
                        history.push(input);
                    }
                    input = String::new();
                    beginning_of_line = prompt().len() as u16;
                    horiz_pos = beginning_of_line + 1;
                    length_of_line = beginning_of_line as u16;
                    write!(stdout, "{}", prompt()).unwrap();
                    stdout.activate_raw_mode().unwrap();
                }
                termion::event::Key::Backspace => {
                    if input.len() > 0 {
                        input.pop();
                        write!(
                            stdout,
                            "{}{}{}",
                            termion::cursor::Left(1),
                            " ",
                            termion::cursor::Left(1)
                        )
                        .unwrap();
                        length_of_line -= 1;
                        horiz_pos -= 1;
                    }
                }
                termion::event::Key::Left => {
                    if horiz_pos > beginning_of_line + 1 {
                        write!(stdout, "{}", termion::cursor::Left(1)).unwrap();
                        horiz_pos -= 1;
                    }
                }
                termion::event::Key::Right => {
                    if horiz_pos < length_of_line {
                        write!(stdout, "{}", termion::cursor::Right(1)).unwrap();
                        horiz_pos += 1;
                    }
                }
                termion::event::Key::Up => {
                    if history_pos > 0 {
                        history_pos -= 1;
                        write!(stdout, "{}", termion::cursor::Left(horiz_pos)); // TODO: it's not clearing everything - don't use the implementation in Down
                        for _i in [0..length_of_line] {
                            write!(stdout, " ");
                        }
                        write!(stdout, "{}", termion::cursor::Left(length_of_line));
                        write!(stdout, "{}", prompt()).unwrap();
                        input = history[history_pos].clone();
                        length_of_line = beginning_of_line + input.len() as u16;
                        horiz_pos = length_of_line;
                        write!(stdout, "{}", input).unwrap();
                    }
                }
                termion::event::Key::Down => {
                    if history_pos < history.len() {
                        history_pos += 1;
                        write!(stdout, "{}", termion::cursor::Left(horiz_pos - beginning_of_line + 1));
                        for _i in [0..length_of_line - beginning_of_line + 1] {
                            write!(stdout, " ");
                        }
                        if history_pos == history.len() {
                            input = String::new();
                        }
                        else {
                            input = history[history_pos].clone();
                        }
                        length_of_line = beginning_of_line + input.len() as u16;
                        let new_horiz = min(horiz_pos, length_of_line);
                        write!(stdout, "{}", termion::cursor::Left(horiz_pos - new_horiz)).unwrap();
                        horiz_pos = new_horiz;
                        write!(stdout, "{}", input).unwrap();
                    }
                }
                termion::event::Key::Char(x) => {
                    input.push(x);
                    write!(stdout, "{}", x).unwrap();
                    length_of_line += 1;
                    horiz_pos += 1;
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
