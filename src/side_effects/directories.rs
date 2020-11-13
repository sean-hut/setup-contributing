use std::fs::create_dir_all;
use std::process::exit;
use std::string::String;

use clap::ArgMatches;

use crate::side_effects::git::{git_string, GitString};

pub fn create_contributing_directory_structure(arguments: &ArgMatches) {
    match create_dir_all(directory_structure(&arguments)) {
        Ok(_) => {
            if arguments.is_present("verbose") {
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

pub fn directory_structure(arguments: &ArgMatches) -> String {
    if arguments.is_present("public-key") {
        format!(
            "CONTRIBUTING/contributors/{}/public-key/current/",
            git_string(GitString::NameDirectory)
        )
    } else {
        "CONTRIBUTING/contributors/".to_string()
    }
}
