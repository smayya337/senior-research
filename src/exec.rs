use std::env;
use std::path::Path;
use std::process::{Command};

pub fn exec(command: &str, args: Vec<&str>) -> bool {
    return match command {
        "cd" => {
            cd(args)
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

fn cd(args: Vec<&str>) -> bool {
    return match args.first() {
        Some(x) => {
            let new_path = Path::new(x);
            let chdir = env::set_current_dir(&new_path);
            assert!(env::set_current_dir(&new_path).is_ok());
            chdir.is_ok()
        },
        None => cd(Vec::from(["~"])),
    }
}