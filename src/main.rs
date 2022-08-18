use std::{
    env,
    io::Write,
    process::{exit, Command},
};

fn main() {
    let mut command = Command::new("cargo");

    let mut args: Vec<String> = vec!["sqlx".into(), "prepare".into(), "--".into()];
    args.extend(env::args().skip(1));
    let mut command = command.args(args);

    // TODO: Any way to get around this? Can't see anything in pre-commit docs about
    //       changing directory while testing the hook.
    let testdir = env::var("PRE_COMMIT_SQLX_PREPARE_TESTDIR").unwrap_or_default();
    if !testdir.is_empty() {
        command = command.current_dir(testdir);
    }

    let output = command.output().expect("Failed to run command");

    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();
    if !output.status.success() {
        exit(output.status.code().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, process::Command};

    #[test]
    fn test_that_sqlx_data_is_created_as_expected() {
        let mut command = Command::new("pre-commit");
        fs::remove_file("todos/sqlx-data.json").unwrap_or_default();
        let command = command
            .args(["try-repo", "--all-files", "."])
            .env("DATABASE_URL", "sqlite:todos.db")
            .env("PRE_COMMIT_SQLX_PREPARE_TESTDIR", "todos");

        let output = command.output().expect("Failed to run command");

        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        eprintln!("{}", stdout);
        eprintln!("{}", stderr);
        assert!(stdout.contains("ensure that sqlx-data.json is up to date..."));
        assert!(stdout.contains("...Passed"));
        assert_eq!(stderr, "");
        assert_eq!(output.status.code(), Some(0));

        let sqlx_data = fs::read_to_string("todos/sqlx-data.json").unwrap();
        assert!(sqlx_data.contains(r#""db": "SQLite""#));
        assert!(sqlx_data.contains(r#""query": "SELECT id FROM todos""#));
    }
}
