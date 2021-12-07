use crate::history::display_history;
use crate::parser::canonical_path;
use crate::separate;
use is_executable::IsExecutable;
use std::env;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{exit, Command};

pub fn exec(command: &str, args: Vec<&str>) -> i32 {
    let canon = canonical_path(&command);
    let path = Path::new(&canon);
    if path.is_file() {
        if path.is_executable() {
            return read_from_script(command, args);
        } else {
            println!("{} is not executable", command);
            return 126;
        }
    }
    return match command {
        "cd" => {
            let dest = args.first();
            cd(dest)
        }
        "info" => info(),
        "urmom" => urmom(),
        "exit" => exit(0),
        "history" => display_history(),
        _ => {
            let child = Command::new(command).args(args).spawn();
            match child {
                Ok(x) => {
                    let mut y = x;
                    y.wait().unwrap().code().unwrap()
                }
                Err(_) => 127,
            }
        }
    };
}

fn cd(dest: Option<&&str>) -> i32 {
    return match dest {
        Some(x) => {
            let canon = canonical_path(x);
            let new_path = Path::new(&canon);
            let chdir = env::set_current_dir(&new_path);
            if chdir.is_ok() {
                0
            } else {
                1
            }
        }
        None => cd(Some(&"~")),
    };
}

fn info() -> i32 {
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
    0
}

fn urmom() -> i32 {
    for _i in 0..1000 {
        println!("UR MOM");
    }
    0
}

fn read_from_script(script: &str, args: Vec<&str>) -> i32 {
    let filepath = canonical_path(&script);
    let file = OpenOptions::new().read(true).open(filepath).unwrap();
    let lines = BufReader::new(file).lines();
    let mut ecodes: Vec<i32> = Vec::new();
    for line in lines {
        let s = line.unwrap();
        let (cmd, arguments) = separate(&s);
        let ec = exec(cmd.unwrap(), arguments);
        ecodes.push(ec);
    }
    if ecodes.len() == 0 {
        0
    } else {
        ecodes.sort();
        *ecodes.last().unwrap()
    }
}
