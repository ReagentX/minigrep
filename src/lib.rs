use std::env;
use std::error::Error;
use std::fs;
use std::process;

pub struct Config {
    needle: String,
    haystack: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        // `args()` returns an Args struct, so transform into an iterator with `collect()`
        let mut args = env::args();
        args.next(); // First element is our program name

        let needle = match args.next() {
            Some(x) => x,
            None => return Err("missing required parameter `needle`"),
        };
        let haystack = match args.next() {
            Some(x) => x,
            None => return Err("missing required parameter `haystack`"),
        };

        let config = Config {
            needle: needle,
            haystack: haystack,
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        };

        eprintln!("Searching for {} in {}", config.needle, config.haystack);
        return Ok(config);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Check if this is a file and if so, read it
    let content: String = get_content(&config.haystack);

    // Find matching lines
    let matched_lines = if config.case_sensitive {
        search(&content, &config.needle)
    } else {
        search_case_insensitive(&content, &config.needle)
    };
    print_matches(&matched_lines);

    // Successful exit
    Ok(())
}

fn search<'a>(content: &'a str, needle: &str) -> Vec<String> {
    let mut results = Vec::new();
    for (line_no, line) in content.split("\n").enumerate() {
        if line.contains(needle) {
            let match_with_line = format!("{}: {}", line_no + 1, line);
            results.push(match_with_line);
        }
    }
    return results;
}

fn search_case_insensitive<'a>(content: &'a str, needle: &str) -> Vec<String> {
    let mut results = Vec::new();
    let query = needle.to_lowercase();
    for (line_no, line) in content.split("\n").enumerate() {
        if line.to_lowercase().contains(&query) {
            let match_with_line = format!("{}: {}", line_no + 1, line);
            results.push(match_with_line);
        }
    }
    return results;
}

fn print_matches(matched_rows: &Vec<String>) {
    if matched_rows.len() == 0 {
        eprintln!("Pattern not found!");
    } else {
        for line in matched_rows {
            println!("{}", line);
        }
    }
}

fn get_content(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Err(err) => {
            eprintln!("Unable to open file: {}", err);
            process::exit(1)
        }
        Ok(contents) => contents,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["2: safe, fast, productive."], search(contents, query));
    }

    #[test]
    fn case_sensitive_no_result() {
        let query = "nothere";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(0, search(contents, query).len());
    }

    #[test]
    fn case_sensitive_all_result() {
        let query = "a";
        let contents = "\
abicus
anteater
atlas";

        assert_eq!(
            vec!["1: abicus", "2: anteater", "3: atlas"],
            search(contents, query)
        );
    }

    #[test]
    fn case_insensitive_one_result() {
        let query = "DuCt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["2: safe, fast, productive."],
            search_case_insensitive(contents, query)
        );
    }

    #[test]
    fn case_insensitive_no_result() {
        let query = "NO";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(0, search_case_insensitive(contents, query).len());
    }

    #[test]
    fn case_insensitive_all_result() {
        let query = "A";
        let contents = "\
abicus
anteater
atlas";

        assert_eq!(
            vec!["1: abicus", "2: anteater", "3: atlas"],
            search_case_insensitive(contents, query)
        );
    }
}
