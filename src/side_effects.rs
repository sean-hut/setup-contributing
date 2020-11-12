mod contributing_markdown;
mod directories;
mod rules;

use std::fs::remove_dir_all;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::exit;

use clap::ArgMatches;

use crate::contributing_markdown::{
    preamble::append_preamble, preparation::append_prepartion, prerequisites::append_prerequisites,
};
use crate::directories::contributing::create_contributing_directory;
use crate::rules::rule::Rule;

pub fn setup_contributing(arguments: ArgMatches) {
    check_directory(&arguments);

    create_contributing_directory(&arguments);

    append_preamble();

    append_prerequisites(&arguments);

    append_prepartion(&arguments);
}

fn check_directory(arguments: &ArgMatches) {
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
        println!("No directory {} to remove", contributing_directory);
    }

    if contributing_exists && remove {
        match remove_dir_all(contributing_directory) {
            Ok(_) => {
                if verbose {
                    println!("Directory {} was removed", contributing_directory);
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

fn open(file_path: &str) -> File {
    match OpenOptions::new().append(true).create(true).open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("[Error] Can not open {}: {}", file_path, e);
            exit(1);
        }
    }
}

fn append(file: &mut File, text: &str) {
    match write!(file, "{}", text) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("[Error] Can not write to file");
            exit(1);
        }
    }
}

fn append_rule(rule: Rule) {
    let contributing: &str = "CONTRIBUTING/CONTRIBUTING.md";

    let mut file: File = open(contributing);

    if rule.flag {
        append(&mut file, rule.rule);
    }
}

fn append_link(arguments: &ArgMatches, rule: Rule) {
    let verbose: bool = arguments.occurrences_of("verbose") > 0;

    let contributing: &str = "CONTRIBUTING/CONTRIBUTING.md";

    let mut file: File = open(contributing);

    if rule.flag {
        append(&mut file, rule.link);

        if verbose {
            println!("{}", rule.verbose);
        }
    }
}
