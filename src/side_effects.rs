mod directories;

use clap::ArgMatches;

use crate::directories::contributing::create_contributing_directory;

pub fn setup_contributing(arguments: ArgMatches) {
    create_contributing_directory(&arguments);
}
