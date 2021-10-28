use std::fs::OpenOptions;
use std::time::SystemTime;
use std::io::prelude::*;

pub fn write_history(time: u64, command: &str) {
    let log = format!(": {}:{};{}", time, cmd_time() - time, command);
    let filepath = format!("{}/.shell_history", home::home_dir().expect("No home dir set!").display().to_string());
    let mut file = OpenOptions::new().create(true).write(true).append(true).open(filepath).unwrap();
    if let Err(_) = writeln!(file, "{}", log) {
        eprintln!("Couldn't write history to file");
    }
}

pub fn cmd_time() -> u64 {
    return SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("This happened before the Unix Epoch!").as_secs();
}