use clap::{App, Arg, ArgGroup};

use side_effects::setup_contributing;

fn main() {
    let arguments = App::new("setup-contributing")
        .version("0.1.0")
        .author("Sean Hutchings <seanhut@yandex.com>")
        .about("Setup contributing directory to use contributing-rules")
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
