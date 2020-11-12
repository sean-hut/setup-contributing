use clap::{App, Arg, ArgGroup};

use side_effects::setup_contributing;

fn main() {
    let arguments = App::new("setup-contributing")
        .version("0.1.0")
        .author("Sean Hutchings <seanhut@yandex.com>")
        .about("Setup contributing directory to use contributing-rules")
        .arg(
            Arg::with_name("remove")
                .long("remove")
                .short("R")
                .multiple(false)
                .display_order(1)
                .help("Remove CONTRIBUTING directory if it exists.  Then the CONTRIBUTING directory is recreated."),
        )
        .arg(
            Arg::with_name("elliptic-curve")
                .long("elliptic-curve")
                .short("e")
                .multiple(false)
                .display_order(2)
                .help("Include elliptic-curve signing key rule"),
        )
        .arg(
            Arg::with_name("public-key")
                .long("public-key")
                .short("P")
                .multiple(false)
                .display_order(3)
                .help("Include provide public key rule"),
        )
        .arg(
            Arg::with_name("contributor-agreement")
                .long("contributor-agreement")
                .short("a")
                .multiple(false)
                .display_order(4)
                .help("Include contributor agreement rule"),
        )
        .arg(
            Arg::with_name("git-config-standard")
                .long("git-config-standard")
                .short("C")
                .multiple(false)
                .display_order(5)
                .help("Inclued standard git configuration rule"),
        )
        .arg(
            Arg::with_name("git-config-gpg-signoff")
                .long("git-config-gpg-signoff")
                .short("g")
                .multiple(false)
                .display_order(6)
                .help("Include git configuration rule with gpg signing and signoff"),
        )
        .group(
            ArgGroup::with_name("git-config")
                .args(&["git-config-standard", "git-config-gpg-signoff"]),
        )
        .arg(
            Arg::with_name("pre-commit-hook")
                .long("pre-commit-hook")
                .short("p")
                .multiple(false)
                .display_order(7)
                .help("Include git pre commit hook rule"),
        )
        .arg(
            Arg::with_name("commit-message-hook")
                .long("commit-message-hook")
                .short("M")
                .multiple(false)
                .display_order(8)
                .help("Include git commit msg hook rule"),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .multiple(false)
                .display_order(18)
                .help("Verbose output"),
        )
        .get_matches();

    setup_contributing(arguments);
}
