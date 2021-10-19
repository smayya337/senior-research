use std::process::{Command};

pub fn exec(command: &str, args: Vec<&str>) -> bool {
    let child = Command::new(command)
        .args(args)
        .spawn();
    if child.is_err() {
        return false;
    }
    let ecode = child.expect("This should never happen.").wait()
        .expect("Failed to wait on child").success();
    ecode
}