use std::env;
use std::path::Path;
use std::process::{Command};

pub fn exec(command: &str, args: Vec<&str>) -> bool {
    return match command {
        "cd" => {
            let dest = args.first();
            cd(dest)
        },
        _ => {
            let child = Command::new(command)
                .args(args)
                .spawn();
            if child.is_err() {
                return false;
            }
            let ecode = child.expect("This should never happen.").wait()
                .expect("Failed to wait on child").success();
            ecode
        },
    }
}

fn cd(dest: Option<&&str>) -> bool {
    return match dest {
        Some(x) => {
            let mut new_path = Path::new(x);
            let dd = dotdot();
            if x.eq(&"~") {
                new_path = Path::new("/home/shreyas");
            }
            else if x.eq(&"..") {
                new_path = Path::new(&dd);
            }
            let chdir = env::set_current_dir(&new_path);
            chdir.is_ok()
        },
        None => cd(Some(&"~")),
    }
}

fn dotdot() -> String {
    let path = env::current_dir();
    let mut buf = path.expect("This should not happen.");
    buf.pop();
    return buf.display().to_string();
}