use clap::{App, Arg, ArgGroup};

fn main() {
    let arguments = App::new("setup-contributing")
        .version("0.1.0")
        .author("Sean Hutchings <seanhut@yandex.com>")
        .about("Setup contributing directory to use contributing-rules")
        .get_matches();
}
