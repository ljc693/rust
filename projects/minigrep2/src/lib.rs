use std::error::Error;
use std::{fs, env};


pub fn run(config: Config) -> Result<(),Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.case_sensitive{
        search_sensitive(&config.query, &contents)
    } else{
        search_insensitive(&config.query, &contents)
    };
    for line in results{
        println!("{}",line);
    }
    // println!("With test:\n{}",contents);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    case_sensitive: bool
}

impl Config {
    
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        args.next();
        let query = match args.next() {
            Some(arg) =>arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        // eprintln!("{0},{1}",query,file_path);
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, file_path ,case_sensitive})
    }
}
pub fn search_sensitive<'a> (query: &str,contents: &'a str) ->Vec<&'a str>{
    // let mut results = Vec::new();
    // for line in contents.lines(){
    //     if line.contains(query){
    //         results.push(line);
    //     }
    // }
    // results
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}
pub fn search_insensitive<'a> (query: &str,contents: &'a str) ->Vec<&'a str>{
    // let mut results = Vec::new();
    // let query = query.to_lowercase();

    // for line in contents.lines(){
    //     if line.to_lowercase().contains(&query){
    //         results.push(line);
    //     }
    // }
    // results
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_sensitive(query, contents));

    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
";

    assert_eq!(vec!["Rust:","Trust me."], search_insensitive(query, contents));
    }
}