use std::process::Command;

fn main() {
    Command::new("npx")
        .args([
            "tailwindcss",
            "-i",
            "./app.css",
            "-o",
            "../target/style.css",
            "-m",
        ])
        .output()
        .unwrap();

    println!("cargo:rerun-if-changed=.");
}
