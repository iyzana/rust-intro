use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let search = if config.case_sensitive {
        search
    } else {
        search_case_insensitive
    };

    search(&config.query, &contents)
        .iter()
        .for_each(|line| println!("{}", line));

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, String> {
        // ignore executable name
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("no query specified")),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("no file specified")),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE")
            .map(|_v| Ok(false))
            .unwrap_or_else(|_v| match args.next().as_ref().map(|s| s.as_ref()) {
                Some("-i") => Ok(false),
                Some(p) => Err(format!("unknown parameter {}", p)),
                _ => Ok(true),
            })?;

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

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
            search_case_insensitive(query, contents)
        );
    }
}
