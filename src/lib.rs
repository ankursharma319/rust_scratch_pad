
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item=String>,) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        return Result::Ok(Config {
            query,
            file_path,
            ignore_case: std::env::var("IGNORE_CASE").is_ok(),
        });
    }
}

///
/// # Example
///
/// This doc example will not only be shown nicely in the html docs of the crate
/// but will also be run as part of cargo test. Insane!
///
/// ```rust
/// let config = rust_scratch_pad::Config { query: "hello".to_string(), file_path: "./README.md".to_string(), ignore_case: true};
/// assert!(rust_scratch_pad::run(config).is_ok());
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(config.file_path)?;
    let lines : std::vec::Vec<&str> = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in lines {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query:&str, contents:&'a str) -> Vec<&'a str> {
    return contents.lines().filter(|line| line.contains(&query)).collect();
}

fn search_case_insensitive<'a>(query:&str, contents:&'a str) -> Vec<&'a str> {
    return contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "
Rust:
safe, fast, productive.
Pick three.
Duct tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "
Rust:
safe, fast, productive.
Pick three.
rustling job";
        assert_eq!(vec!["Rust:", "rustling job"], search_case_insensitive(query, contents));
    }
}

