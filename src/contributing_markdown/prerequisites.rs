use std::fs::File;

use clap::ArgMatches;

use crate::rules::contributing_prerequisites::{
    any_prerequisites, prerequisites, PREREQUISITE_HEADING,
};
use crate::rules::rule::Rule;
use crate::{append, open};

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

pub fn append_prerequisites(arguments: &ArgMatches) {
    let contributing: &str = "CONTRIBUTING/CONTRIBUTING.md";

    let mut file: File = open(contributing);

    if any_prerequisites(&arguments) {
        append(&mut file, PREREQUISITE_HEADING);

        for r in prerequisites(&arguments) {
            append_rule(r);
        }

        append(&mut file, "\n");

        for r in prerequisites(&arguments) {
            append_link(&arguments, r);
        }
    }
}
