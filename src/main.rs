//bring code from lib.rs to the scope of main.rs
extern crate minigrep;
// using environment to collect(collect method) arguments
// in the vector
use std::env;
//for exit status process
use std::process;

use minigrep::Config;
fn main() {
    // Reading arguments value
    let args: Vec<String> = env::args().collect();

    // Saving the argument values in variables
    // let programname = &args[0];
    // let query = &args[1];
    // let filename = &args[2];

    //Extraction of parse_config function from main
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("In file {}", config.filename);
    // println!("Searching for {}", config.query);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

//get arguments from cli:
//              - What we want to search?
//              - Form the given file?
//              - program name if we want

//use vector to collect the info/data from the cli.

// use lib to access the file and read the content
