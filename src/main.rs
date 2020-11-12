use clap::{App, Arg, ArgGroup};

use impure::setup_contributing;

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
                .value_name("PATH")
                .takes_value(true)
                .multiple(false)
                .display_order(1)
                .help("Include provide public key rule.  Also copy the public key and fingerprint in <PATH> into the contributing directory structure.  <PATH> should end with a /"),
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
            Arg::with_name("git-flow")
                .long("git-flow")
                .short("F")
                .multiple(false)
                .display_order(9)
                .help("Include git-flow rule"),
        )
        .arg(
            Arg::with_name("fetch-merge")
                .long("fetch-merge")
                .short("f")
                .multiple(false)
                .display_order(10)
                .help("Include fetch and merge rule"),
        )
        .arg(
            Arg::with_name("documentation")
                .long("documentation")
                .short("d")
                .multiple(false)
                .display_order(11)
                .help("Include update documentation rule"),
        )
        .arg(
            Arg::with_name("test")
                .long("test")
                .short("T")
                .multiple(false)
                .display_order(12)
                .help("Include update tests rule"),
        )
        .arg(
            Arg::with_name("make")
                .long("make")
                .short("m")
                .multiple(false)
                .display_order(13)
                .help("Include run make rule"),
        )
        .arg(
            Arg::with_name("small-commits")
                .long("small-commits")
                .short("s")
                .multiple(false)
                .display_order(14)
                .help("Include small commits rule"),
        )
        .arg(
            Arg::with_name("sign-commit")
                .long("sign-commit")
                .short("S")
                .multiple(false)
                .display_order(15)
                .help("Include sign commits rule"),
        )
        .arg(
            Arg::with_name("amaranth")
                .long("amaranth")
                .short("A")
                .multiple(false)
                .display_order(16)
                .help("Include Amaranth commit message format rule"),
        )
        .arg(
            Arg::with_name("tpope")
                .long("tpope")
                .short("t")
                .multiple(false)
                .display_order(17)
                .help("Include tpope commit message format rule"),
        )
        .group(ArgGroup::with_name("commit-message-format").args(&["amaranth", "tpope"]))
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
