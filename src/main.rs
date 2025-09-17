// We need to use the std::env::args function from the Standard Library;
use std::env;
// fs library to use functions for reading and writing files
use std::fs;

use std::process;

use std::error::Error;

use minigrep::search;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.search_query);
    println!("In file: {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let file_contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.search_query, &file_contents) {
        println!("{line}");
    }

    Ok(())
}

struct Config {
    search_query: String,
    file_path: String,
}

impl Config {
    fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments given");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { search_query: query, file_path })
    }
}
