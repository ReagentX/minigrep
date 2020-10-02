use std::env;
use std::fs;

struct Config {
    needle: String,
    haystack: String,
}

impl Config {
    fn new() -> Config {
        // `args()` returns an Args struct, so transform into an iterator with `collect()`
        let mut args: Vec<String> = env::args().collect();
        if args.len() == 3 {
            let config = Config {
                needle: args.remove(1),
                haystack: args.remove(1),
            };
            println!("Searching for {} in {}", config.needle, config.haystack);
            return config;
        } else if args.len() <= 2 {
            panic!("Missing required parameters: needle, haystack")
        }
        panic!("Too many parameters passed: {:?}", args);
    }
}

fn main() {
    // Build config
    let config = Config::new();

    // Check if this is a file and if so, read it
    let content: String = get_content(&config.haystack);
    
    // Find matching lines
    print_matches(content, &config.needle)
}

fn print_matches(content: String, needle: &String) {
    for (line_no, line) in content.split("\n").enumerate() {
        // Reference to config.needle so we do not consume it
        if line.contains(needle) {
            println!("{}: {}", line_no + 1, line);
        }
    }
}

fn get_content(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Err(err) => panic!("{}", err),
        Ok(contents) => contents,
    }
}
