pub fn separate(input: &str) -> (Option<&str>, Vec<&str>) {
    let mut split = input.split_whitespace();
    let mut vec: Vec<&str> = Vec::new();
    let cmd = split.next();
    for s in split {
        vec.push(s);
    }
    (cmd, vec)
}