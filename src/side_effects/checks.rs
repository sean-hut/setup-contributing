use std::fs::remove_dir_all;
use std::path::Path;
use std::process::exit;

use clap::ArgMatches;

use crate::rules::{
    commit_preparation::any_preparation, committing::any_committing,
    contributing_prerequisites::any_prerequisites,
};

pub fn require_flag(arguments: &ArgMatches) {
    if !any_prerequisites(arguments) && !any_preparation(arguments) && !any_committing(arguments) {
        eprintln!("[Error] Use at least one include rule flag");
        exit(1);
    }
}

pub fn gpg_related_flag_checks(arguments: &ArgMatches) {
    let public_key: bool = arguments.occurrences_of("public-key") > 0;
    let sign_commit: bool = arguments.occurrences_of("sign-commit") > 0;
    let contributor_agreement: bool = arguments.occurrences_of("contributor-agreement") > 0;

    if sign_commit && !public_key {
        eprintln!(
            "[Error] Use of the --sign-commit flag requires \
             the use of the --public-key flag"
        );
        exit(1);
    }

    if contributor_agreement && !public_key {
        eprintln!(
            "[Error] Use of the --contributor-agreement flag requires \
             the use of the --public-key flag"
        );
        exit(1);
    }

    if public_key && !sign_commit && !contributor_agreement {
        eprintln!(
            "[Error] Use of the --public-key flag requires the use of \
             at least one of the --sign-commit flag or the --contributor-agreement flag"
        );
        exit(1);
    }
}

pub fn check_directory(arguments: &ArgMatches) {
    let contributing_directory: &str = "CONTRIBUTING/";

    let remove: bool = arguments.occurrences_of("remove") > 0;

    let contributing_exists: bool = Path::new(contributing_directory).exists();

    let verbose: bool = arguments.occurrences_of("verbose") > 0;

    if contributing_exists && !remove {
        eprintln!(
            "[Error] CONTRIBUTING directory already exists. \
             Use the --remove flag to remove the directory and it's contents and recreate it."
        );
        exit(1);
    }

    if !contributing_exists && remove && verbose {
        println!("[Info] No directory {} to remove", contributing_directory);
    }

    if contributing_exists && remove {
        match remove_dir_all(contributing_directory) {
            Ok(_) => {
                if verbose {
                    println!("[Info] Directory {} was removed", contributing_directory);
                }
            }
            Err(e) => {
                eprintln!(
                    "[Error] {} directory could not be removed. {}.",
                    contributing_directory, e
                );
                exit(1);
            }
        }
    }
}
