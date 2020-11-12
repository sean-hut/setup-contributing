use std::fs::File;

use crate::{append, open};

pub fn append_preamble() {
    let contributing: &str = "CONTRIBUTING/CONTRIBUTING.md";

    let mut file: File = open(contributing);

    let preamble: &str = "# Contributing Rules

These are the contributing rules.\n\n";

    append(&mut file, preamble);
}
