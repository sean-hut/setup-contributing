use std::fs::{File, OpenOptions};
use std::io::Write;
use std::process::exit;

pub fn open(file_path: &str) -> File {
    match OpenOptions::new().append(true).create(true).open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("[Error] Can not open {}: {}", file_path, e);
            exit(1);
        }
    }
}

pub fn append(file: &mut File, text: &str) {
    match write!(file, "{}", text) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("[Error] Can not write to file. {}", e);
            exit(1);
        }
    }
}
