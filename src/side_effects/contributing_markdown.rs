use std::fs::{File, OpenOptions};
use std::io::Write;
use std::process::exit;

use clap::ArgMatches;

use crate::rules::{
    commit_preparation::any_preparation, committing::any_committing,
    contributing_prerequisites::any_prerequisites, rule::Rule,
};

const CONTRIBUTING: &str = "CONTRIBUTING/CONTRIBUTING.md";

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

pub fn append_preamble(arguments: &ArgMatches) {
    let mut file: File = open(CONTRIBUTING);

    let verbose: bool = arguments.occurrences_of("verbose") > 0;

    let preamble: &str = "# Contributing Rules

These are the contributing rules.\n\n";

    append(&mut file, preamble);

    if verbose {
        println!("[Info] Created CONTRIBUTING.md");
    }
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

pub fn append_section(arguments: &ArgMatches, any_rules: bool, rules: Vec<Rule>, heading: &str) {
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

pub fn space_after_prerequisites(arguments: &ArgMatches) {
    let mut file: File = open(CONTRIBUTING);

    if any_prerequisites(&arguments) && (any_preparation(&arguments) || any_committing(&arguments))
    {
        append(&mut file, "\n");
    }
}

pub fn space_after_preparation(arguments: &ArgMatches) {
    let mut file: File = open(CONTRIBUTING);

    if any_preparation(&arguments) && any_committing(&arguments) {
        append(&mut file, "\n");
    }
}
