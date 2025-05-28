use std::process::Command;

fn main() {
    println!("cargo::rerun-if-changed=../database");

    Command::new("spacetime")
        .args(["generate", "-y"])
        .args(["--lang", "rust"])
        .args(["-p", "../database"])
        .args(["-o", "src/database/bindings"])
        .output()
        .ok();
}
