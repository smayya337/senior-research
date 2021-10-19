use std::process::{Child, Command, ExitStatus};

pub fn exec(command: &str, args: Vec<&str>) -> bool {
    let mut child = Command::new(command)
        .args(args)
        .spawn();
    if child.is_err() {
        return false;
    }
    let ecode = child.expect("This should never happen.").wait()
        .expect("Failed to wait on child").success();
    ecode
}