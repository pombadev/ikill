use std::{env, process};

mod ikill;

const USAGE: &str = "
ikill - Interactively kill processes

USAGE:
    ikill

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
";

fn unknown_args(arg: String) {
    eprintln!(
        "
error: Found argument '{}' which wasn't expected

USAGE:
    ikill

For more information try --help
",
        arg
    );
}

fn main() {
    smol::block_on(async {
        match env::args().nth(1) {
            None => {
                if let Err(error) = ikill::run().await {
                    eprintln!("Error: {}", error);
                }
            }
            Some(arg) => {
                match arg.as_str() {
                    "-h" | "--help" => println!("{}", USAGE),
                    "-V" | "--version" => {
                        println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                    }
                    _ => {
                        unknown_args(arg);
                        process::exit(1);
                    }
                };
            }
        }
    });
}
