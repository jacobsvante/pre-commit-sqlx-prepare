use std::{
    env::args,
    io::Write,
    process::Command,
};

fn main() {
    let filenames_were_passed = !args().skip(1).take(1).count() > 0;
    if filenames_were_passed {
        let output = Command::new("cargo")
            .args(["sqlx", "prepare", "--", "--lib"])
            .output()
            .expect("Failed to run command");
        std::io::stdout().write_all(&output.stdout).unwrap();
        std::io::stderr().write_all(&output.stdout).unwrap();
    } else {
        println!("No files passed in");
    }
}
