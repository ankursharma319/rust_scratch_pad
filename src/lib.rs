
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Result::Err("Too few args, need query and file_path");
        }
        return Result::Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case: std::env::var("IGNORE_CASE").is_ok(),
        });
    }
}

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
    let mut results: std::vec::Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    return results;
}

fn search_case_insensitive<'a>(query:&str, contents:&'a str) -> Vec<&'a str> {
    let mut results: std::vec::Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }
    return results;
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

