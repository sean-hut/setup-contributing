mod directories;
mod rules;
mod side_effects;

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::process::exit;

use clap::ArgMatches;

use crate::directories::contributing::create_contributing_directory;
use crate::rules::{
    commit_preparation::{any_preparation, preparation, PREPARATION_HEADING},
    committing::{any_committing, committing, COMMITTING_HEADING},
    contributing_prerequisites::{any_prerequisites, prerequisites, PREREQUISITE_HEADING},
    rule::Rule,
};
use crate::side_effects::checks::{check_directory, require_flag};

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

    space_after_prerequisites(&arguments);

    append_section(
        &arguments,
        any_preparation(&arguments),
        preparation(&arguments),
        PREPARATION_HEADING,
    );

    space_after_preparation(&arguments);

    append_section(
        &arguments,
        any_committing(&arguments),
        committing(&arguments),
        COMMITTING_HEADING,
    );
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

fn space_after_prerequisites(arguments: &ArgMatches) {
    let mut file: File = open(CONTRIBUTING);

    if any_prerequisites(&arguments) && (any_preparation(&arguments) || any_committing(&arguments))
    {
        append(&mut file, "\n");
    }
}

fn space_after_preparation(arguments: &ArgMatches) {
    let mut file: File = open(CONTRIBUTING);

    if any_preparation(&arguments) && any_committing(&arguments) {
        append(&mut file, "\n");
    }
}
