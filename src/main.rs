use std::process::{self, Stdio};
use temp_file;
fn main() {
    let temp = temp_file::empty();
    let _broot = process::Command::new("broot")
        .arg("--outcmd")
        .arg(temp.path())
        .stdout(Stdio::piped())
        .status();
    println!(
        "{}",
        std::fs::read_to_string(temp.path())
            .unwrap()
            .trim()
            .replace("\"", "")
    );
}
