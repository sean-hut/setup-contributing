use std::process::{exit, Command, Output};

enum User {
    Name,
    Email,
}

pub enum GitString {
    Name,
    Email,
    NameDirectory,
}

fn git_standard_out(variable: User) -> String {
    let config_variable = match variable {
        User::Name => "user.name",
        User::Email => "user.email",
    };

    let output: Output = match Command::new("git")
        .arg("config")
        .arg(config_variable)
        .output()
    {
        Ok(output) => output,
        Err(e) => {
            eprintln!(
                "[Error] Could not get Git's user.name configuration. \
                 Please make sure Git is installed and that user.name is configured. {}",
                e
            );
            exit(1);
        }
    };

    match String::from_utf8(output.stdout) {
        Ok(standard_out) => standard_out.trim_end().to_string(),
        Err(e) => {
            eprintln!(
                "[Error] Could not get Git's user.name configuration. \
                 Please make sure Git is installed and that user.name is configured. {}",
                e
            );
            exit(1);
        }
    }
}

pub fn git_string(git_string_type: GitString) -> String {
    match git_string_type {
        GitString::Name => git_standard_out(User::Name),
        GitString::Email => git_standard_out(User::Email),
        GitString::NameDirectory => git_standard_out(User::Name)
            .to_lowercase()
            .replace(" ", "-"),
    }
}
