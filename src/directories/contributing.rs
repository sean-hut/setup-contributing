use std::fs::create_dir;
use std::process::exit;

use clap::ArgMatches;

pub fn create_contributing_directory(arguments: &ArgMatches) {
    let verbose: bool = arguments.occurrences_of("verbose") > 0;

    match create_dir("CONTRIBUTING/") {
        Ok(_) => {
            if verbose {
                println!("Created CONTRIBUTING directory")
            }
        }
        Err(e) => {
            eprintln!("[Error] Could not create CONTRIBUTING directory. {}", e);
            exit(1);
        }
    }
}
