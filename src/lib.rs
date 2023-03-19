use sha256::{digest};
use std::error::Error;
use std::process;

pub struct Config {
    pub query: String,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() == 1 {
            let query = digest("");
            println!("{}", query);
            process::exit(0);

            //return Err("not enough arguments");
        }

        let query = args[1].clone();

        Ok(Config { query })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let val = digest(config.query);
    println!("{}", val);
    Ok(())
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
let mut results = Vec::new();
    for line in contents.lines() {
        // do something with line
        if line.contains(query) {
            // do something with line
            results.push(line);
        }
    }
    results
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
}
