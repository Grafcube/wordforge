use std::{fs, process::Command};

fn main() {
    Command::new("npm")
        .args(["run", "check"])
        .current_dir(fs::canonicalize("./ui").unwrap())
        .output()
        .unwrap();
    Command::new("npm")
        .args(["run", "build"])
        .current_dir(fs::canonicalize("./ui").unwrap())
        .output()
        .unwrap();

    println!("cargo:rerun-if-changed=ui");
}
