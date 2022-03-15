use crate::parser::canonical_path;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::SystemTime;

pub fn write_history(time: u64, command: &str) {
    let log = format!(": {}:{};{}", time, cmd_time() - time, command);
    let filepath = canonical_path(&"~/.shell_history");
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(filepath)
        .unwrap();
    if writeln!(file, "{}", log).is_err() {
        eprintln!("Couldn't write history to file");
    }
}

pub fn cmd_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn read_history() -> Vec<String> {
    let filepath = canonical_path(&"~/.shell_history");
    let file = OpenOptions::new().read(true).open(filepath).unwrap();
    let lines = BufReader::new(file).lines();
    let mut shell_history: Vec<String> = Vec::new();
    for line in lines {
        let s = line.unwrap();
        let mut split = s.split(';');
        let _timestamp = split.next();
        let cmd = split.next().unwrap();
        shell_history.push(String::from(cmd));
    }
    shell_history
}

pub fn display_history() -> i32 {
    let shell_history = read_history();
    for (i, x) in shell_history.iter().enumerate() {
        let cmd = x.split(";").last().unwrap();
        println!("{} {}", i + 1, cmd);
    }
    0
}
