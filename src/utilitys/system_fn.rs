use std::process::Command;
pub fn system(cmd: &str) {
    let _ = Command::new("sh").arg("-c").arg(cmd).status();
}
