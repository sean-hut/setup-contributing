mod directories;
mod rules;

use std::fs::remove_dir_all;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::exit;

use clap::ArgMatches;

use crate::directories::contributing::create_contributing_directory;
use crate::rules::{
    commit_preparation::{any_preparation, preparation, PREPARATION_HEADING},
    committing::{any_committing, committing, COMMITTING_HEADING},
    contributing_prerequisites::{any_prerequisites, prerequisites, PREREQUISITE_HEADING},
    rule::Rule,
};

const CONTRIBUTING: &str = "CONTRIBUTING/CONTRIBUTING.md";

pub fn setup_contributing(arguments: ArgMatches) {
    require_flag(&arguments);

    check_directory(&arguments);

    create_contributing_directory(&arguments);

    append_preamble();

    append_section(
        &arguments,
        any_prerequisites(&arguments),
        prerequisites(&arguments),
        PREREQUISITE_HEADING,
    );

    append_section(
        &arguments,
        any_preparation(&arguments),
        preparation(&arguments),
        PREPARATION_HEADING,
    );

    append_section(
        &arguments,
        any_committing(&arguments),
        committing(&arguments),
        COMMITTING_HEADING,
    );
}

fn require_flag(arguments: &ArgMatches) {
    if !any_prerequisites(arguments) && !any_preparation(arguments) && !any_committing(arguments) {
        eprintln!("[Error] Use at least one include rule flag");
        exit(1);
    }
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

pub fn append_preamble() {
    let mut file: File = open(CONTRIBUTING);

    let preamble: &str = "# Contributing Rules

These are the contributing rules.\n\n";

    append(&mut file, preamble);
}

fn append_rule(rule: &Rule) {
    let mut file: File = open(CONTRIBUTING);

    if rule.flag {
        append(&mut file, rule.rule);
    }
}

fn append_link(arguments: &ArgMatches, rule: &Rule) {
    let verbose: bool = arguments.occurrences_of("verbose") > 0;

    let mut file: File = open(CONTRIBUTING);

    if rule.flag {
        append(&mut file, rule.link);

        if verbose {
            println!("{}", rule.verbose);
        }
    }
}

fn append_section(arguments: &ArgMatches, any_rules: bool, rules: Vec<Rule>, heading: &str) {
    let mut file: File = open(CONTRIBUTING);

    if any_rules {
        append(&mut file, heading);

        for r in &rules {
            append_rule(r);
        }

        append(&mut file, "\n");

        for r in &rules {
            append_link(&arguments, r);
        }
    }
}
