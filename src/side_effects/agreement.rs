use std::fs::File;

use clap::ArgMatches;

use crate::agreement::version_0_2_0::contributor_agreement_0_2_0;
use crate::side_effects::append::{append, open};

pub fn write_contributor_agreement(arguments: &ArgMatches) {
    let mut file: File = open("CONTRIBUTING/contributor-agreement.txt");

    if arguments.is_present("contributor-agreement") {
        append(&mut file, &contributor_agreement_0_2_0(&arguments)[..]);

        if arguments.is_present("verbose") {
            println!("[Info] Added contributor agreement");
        }
    }
}
