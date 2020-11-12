mod rules;
mod side_effects;

use clap::ArgMatches;

use crate::rules::{
    commit_preparation::{any_preparation, preparation, PREPARATION_HEADING},
    committing::{any_committing, committing, COMMITTING_HEADING},
    contributing_prerequisites::{any_prerequisites, prerequisites, PREREQUISITE_HEADING},
};
use crate::side_effects::{
    checks::{check_directory, require_flag},
    contributing_markdown::{
        append_preamble, append_section, space_after_preparation, space_after_prerequisites,
    },
    directories::create_contributing_directory,
};

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
