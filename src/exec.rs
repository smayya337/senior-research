use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command};
use home;

pub fn exec(command: &str, args: Vec<&str>) -> i32 {
    return match command {
        "cd" => {
            let dest = args.first();
            cd(dest)
        },
        _ => {
            let child = Command::new(command)
                .args(args)
                .spawn();
            let ecode = match child {
                Ok(x) => {
                    let mut y = x;
                    y.wait()
                        .expect("Failed to wait on child").code().expect("This should not happen.")
                },
                Err(_) => 127,
            };
            ecode
        },
    }
}

fn cd(dest: Option<&&str>) -> i32 {
    return match dest {
        Some(x) => {
            let homedir = home::home_dir().expect("No home dir set!").display().to_string();
            let mut canon = String::from(*x);
            if x.starts_with(&"~") {
                canon = canon.replace("~", &homedir);
            }
            else if x.contains(&"..") {
                let mut spl = x.split("..").peekable();
                let mut np = Vec::new();
                while let Some(y) = spl.next() {
                    let p = match spl.peek() {
                        Some(_) => dotdot(PathBuf::from(y)),
                        None => String::from(y),
                    };
                    np.push(p);
                }
                canon = np.join("/");
            }
            let new_path = Path::new(&canon);
            let chdir = env::set_current_dir(&new_path);
            if chdir.is_ok() {
                0
            }
            else {
                1
            }
        },
        None => cd(Some(&"~")),
    }
}

fn dotdot(mut buf: PathBuf) -> String {
    buf.pop();
    return buf.display().to_string();
}