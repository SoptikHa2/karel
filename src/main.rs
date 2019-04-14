extern crate clap;
use clap::{App, Arg};

mod lib;
mod syntax;
mod core;

fn main() {
    let matches = App::new("Karel")
                    .version("0.1")
                    .author("Petr Šťastný <petr.stastny01@gmail.com>")
                    .about("Karel programming language interpreter")
                    .arg(Arg::with_name("interactive")
                        .long("interactive")
                        .short("i")
                        .multiple(false)
                        .help("Start interactive session. This is not designed to communicate with other programs. User cannot create new methods, they have to be loaded with --lib.")
                        .required_unless("file")
                        .conflicts_with("file")
                        .takes_value(false))
                     .arg(Arg::with_name("ignore")
                        .long("ignore")
                        .multiple(false)
                        .help("Ignore runtime errors in program. Program will not halt, it will skip runtime errors.")
                        .takes_value(false))
                    .arg(Arg::with_name("file")
                        .multiple(false)
                        .takes_value(true)
                        .help("Specify source file. The file has to contain main. You can add additional files as libraries.")
                        .required_unless("interactive")
                        .conflicts_with("interactive"))
                    .arg(Arg::with_name("lib")
                        .multiple(true)
                        .help("Loaded library source code from file(s). Library source can only contain method definitions. Library cannot contain main.")
                        .takes_value(true));
    let matches = matches.get_matches();

    lib::run(
        matches.is_present("interactive"),
        matches.value_of("file"),
        matches.values_of("lib").map(|v| v.collect()),
        matches.is_present("ignore"));
}
