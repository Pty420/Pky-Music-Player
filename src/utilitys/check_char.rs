pub fn check_head_char(s: &str, c: char) -> bool {
    if s.chars().next().unwrap() == c {
        return true;
    }
    false
}
