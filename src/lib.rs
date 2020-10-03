use std::env;
use std::error::Error;
use std::fs;
use std::process;

pub struct Config {
    needle: String,
    haystack: String,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        // `args()` returns an Args struct, so transform into an iterator with `collect()`
        let mut args: Vec<String> = env::args().collect();
        if args.len() == 3 {
            let config = Config {
                needle: args.remove(1),
                haystack: args.remove(1),
            };
            println!("Searching for {} in {}", config.needle, config.haystack);
            return Ok(config);
        } else if args.len() <= 2 {
            return Err("missing required parameters: needle, haystack");
        }
        return Err("too many parameters passed!");
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Check if this is a file and if so, read it
    let content: String = get_content(&config.haystack);

    // Find matching lines
    print_matches(content, &config.needle);

    // Successful exit
    Ok(())
}

fn print_matches(content: String, needle: &String) {
    for (line_no, line) in content.split("\n").enumerate() {
        if line.contains(needle) {
            println!("{}: {}", line_no + 1, line);
        }
    }
}

fn get_content(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Err(err) => {
            println!("Unable to open file: {}", err);
            process::exit(1)
        }
        Ok(contents) => contents,
    }
}