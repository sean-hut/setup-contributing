use std::fs::File;

use clap::ArgMatches;

use crate::rules::contributing_prerequisites::{
    any_prerequisites, prerequisites, PREREQUISITE_HEADING,
};

use crate::{append, append_link, append_rule, open};

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
