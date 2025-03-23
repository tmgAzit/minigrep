//trait in std lib for working with errors
use std::error::Error;
// handle a file
use std::fs;
//contains various useful traits for doing I/O
// use std::io::prelude::*;

//functions for working with environment variables
use std::env;

//defing struct
pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

//parsing function
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string."),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }
}

//extracting a run function containg the rest of the program logic
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //Access the file
    // let mut f = File::open(config.filename)?;
    let contents = fs::read_to_string(config.filename)?;

    //reading the file - read_to_string method
    // let mut contents = String::new();
    // f.read_to_string(&mut contents)?;

    // println!("Program name is {}", programname);
    // println!("With text:\n{}", contents);

    let results = if config.ignore_case {
        search_case_insensitivity(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    //return Ok
    Ok(())
}

//adding test modules for the testing
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitivity(query, contents)
        );
    }
}

// defining the search function
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //let mut results = Vec::new();

    // iterating through each line in contents
    // for line in contents.lines() {
    //     if line.trim().contains(query) {
    //         results.push(line.trim());
    //     }
    // }
    //results

    contents
        .lines()
        .filter(|line| line.trim().contains(query))
        .collect()
}

pub fn search_case_insensitivity<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line.trim());
        }
    }
    results
}
