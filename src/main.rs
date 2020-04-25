use std::env::args;

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
    println!(
        "
error: Found argument '{}' which wasn't expected, or isn't valid in this context

USAGE:
    ikill

For more information try --help
",
        arg
    );
}

#[tokio::main]
async fn main() {
    match args().nth(1) {
        None => {
            ikill::run().await;
        }
        Some(arg) => {
            if arg == "-h" || arg == "--help" {
                println!("{}", USAGE);
            } else if arg == "-V" || arg == "--version" {
                println!("ikill v1.2.1");
            } else {
                unknown_args(arg);
                std::process::exit(1);
            }
        }
    };
}
