use std::path::PathBuf;

pub fn separate(input: &str) -> (Option<&str>, Vec<&str>) {
    let mut split = input.split_whitespace();
    let mut vec: Vec<&str> = Vec::new();
    let cmd = split.next();
    for s in split {
        vec.push(s);
    }
    (cmd, vec)
}

pub fn canonical_path(x: &&str) -> String {
    let homedir = home::home_dir().unwrap().display().to_string();
    let mut canon = String::from(*x);
    if x.starts_with(&"~") {
        canon = canon.replace("~", &homedir);
    } else if x.contains(&"..") {
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
    canon
}

fn dotdot(mut buf: PathBuf) -> String {
    buf.pop();
    return buf.display().to_string();
}
