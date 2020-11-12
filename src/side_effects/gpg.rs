use std::fs::copy;
use std::process::exit;

use clap::ArgMatches;

use crate::side_effects::directories::directory_structure;

enum GPG {
    PublicKey,
    Fingerprint,
}

pub fn copy_gpg_files(arguments: &ArgMatches) {
    copy_gpg_file(&arguments, GPG::PublicKey);
    copy_gpg_file(&arguments, GPG::Fingerprint);
}

fn copy_gpg_file(arguments: &ArgMatches, file_type: GPG) {
    let path = match arguments.value_of("public-key") {
        Some(path) => path,
        None => {
            eprintln!("[Error] <PATH> for --public-key option must be provided");
            exit(1);
        }
    };

    let source: String = match file_type {
        GPG::PublicKey => format!("{}{}", path, "public-key.gpg"),
        GPG::Fingerprint => format!("{}{}", path, "fingerprint.txt"),
    };

    let destination: String = match file_type {
        GPG::PublicKey => format!("{}{}", directory_structure(&arguments), "public-key.gpg"),
        GPG::Fingerprint => format!("{}{}", directory_structure(&arguments), "fingerprint.txt"),
    };

    let file_output: &str = match file_type {
        GPG::PublicKey => "public-key.gpg",
        GPG::Fingerprint => "fingerprint.txt",
    };

    match copy(source, destination) {
        Ok(_) => println!("[Info] Added {}", file_output),
        Err(e) => {
            eprintln!("[Error] Could not copy {}. {}", file_output, e);
            exit(1);
        }
    }
}
