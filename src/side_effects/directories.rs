use std::fs::create_dir_all;
use std::process::{exit, Command, Output};
use std::string::String;

use clap::ArgMatches;

pub fn create_contributing_directory_structure(arguments: &ArgMatches) {
    let verbose: bool = arguments.is_present("verbose");

    let public_key: bool = arguments.is_present("public-key");

    let output: Output = match Command::new("git").arg("config").arg("user.name").output() {
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

    let standard_out: String = match String::from_utf8(output.stdout) {
        Ok(standard_out) => standard_out,
        Err(e) => {
            eprintln!(
                "[Error] Could not get Git's user.name configuration. \
                 Please make sure Git is installed and that user.name is configured. {}",
                e
            );
            exit(1);
        }
    };

    let name_directory: String = standard_out
        .to_lowercase()
        .replace(" ", "-")
        .trim_end()
        .to_string();

    let directory_structure: String = if public_key {
        format!(
            "CONTRIBUTING/contributors/{}/public-key/current/",
            name_directory
        )
    } else {
        "CONTRIBUTING/contributors/".to_string()
    };

    match create_dir_all(directory_structure) {
        Ok(_) => {
            if verbose {
                println!("[Info] Created contributing directory structure")
            }
        }
        Err(e) => {
            eprintln!(
                "[Error] Could not create contributing directory structure. {}",
                e
            );
            exit(1);
        }
    }
}
