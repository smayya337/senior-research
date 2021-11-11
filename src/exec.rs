use crate::parser::canonical_path;
use std::env;
use std::path::Path;
use std::process::Command;

pub fn exec(command: &str, args: Vec<&str>) -> i32 {
    return match command {
        "cd" => {
            let dest = args.first();
            cd(dest)
        }
        "info" => info(),
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
