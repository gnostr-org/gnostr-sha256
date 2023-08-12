use sha256::{digest};
use std::error::Error;
use std::process;

pub struct Config {
    pub query: String,
}

pub fn print_type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

impl Config {

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() == 1 {

            println!("{}", digest("".to_string()));

            process::exit(0);

            //return Err("not enough arguments");
        }

        let query = args[1].clone();

        #[cfg(debug_assertions)]
        let s = &"hello world".to_string();

        #[cfg(debug_assertions)]
        let cloned_s = s.clone();

        #[cfg(debug_assertions)]
        println!("{:?}", print_type_of(&s));

        #[cfg(debug_assertions)]
        println!("{:?}", print_type_of(&cloned_s));

        Ok(Config { query })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{  }", digest(config.query));
    Ok(())
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
let mut results = Vec::new();
    for line in contents.lines() {
        // do something with line
        println!("{}", line);
        if line.contains(query) {
            // do something with line
            let val = digest(query);
            println!("{}", val);
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
    #[test]
    fn one_query() {
        let query = digest("");
        let contents = "\
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        assert_eq!(vec!["e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"], search(&query, contents));
    }
    #[test]
    #[should_panic]
    fn panic_query() {
        let query = digest("");
        let contents = "\
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 ";

        assert_eq!(vec!["e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"], search(&query, contents));
    }
}
