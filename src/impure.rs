mod agreement;
mod rules;
mod side_effects;

use clap::ArgMatches;

use crate::rules::{
    commit_preparation::{any_preparation, preparation, PREPARATION_HEADING},
    committing::{any_committing, committing, COMMITTING_HEADING},
    contributing_prerequisites::{any_prerequisites, prerequisites, PREREQUISITE_HEADING},
};
use crate::side_effects::{
    agreement::write_contributor_agreement,
    checks::{check_directory, check_project_name, gpg_related_flag_checks, require_flag},
    contributing_markdown::{
        append_preamble, append_section, space_after_preparation, space_after_prerequisites,
    },
    directories::create_contributing_directory_structure,
    gpg::copy_gpg_files,
};

pub fn setup_contributing(arguments: ArgMatches) {
    require_flag(&arguments);

    gpg_related_flag_checks(&arguments);

    check_project_name(&arguments);

    check_directory(&arguments);

    create_contributing_directory_structure(&arguments);

    append_preamble(&arguments);

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

    copy_gpg_files(&arguments);

    write_contributor_agreement(&arguments);
}
