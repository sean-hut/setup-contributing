use std::fs::File;

use clap::ArgMatches;

use crate::rules::commit_preparation::{any_preparation, preparation, PREPARATION_HEADING};

use crate::{append, append_link, append_rule, open};

pub fn append_prepartion(arguments: &ArgMatches) {
    let contributing: &str = "CONTRIBUTING/CONTRIBUTING.md";

    let mut file: File = open(contributing);

    if any_preparation(&arguments) {
        append(&mut file, PREPARATION_HEADING);

        for r in preparation(&arguments) {
            append_rule(r);
        }

        append(&mut file, "\n");

        for r in preparation(&arguments) {
            append_link(&arguments, r);
        }
    }
}
