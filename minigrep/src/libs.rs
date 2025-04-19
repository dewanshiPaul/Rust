use std::{env, error::Error, fs};
use crate::errors;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

pub fn parse_args(args: &Vec<String>) -> Result<Config, Box<dyn Error>> {
    if args.len() != 3 {
        return Err(Box::new(errors::CustomError::InsufficientSize(2)));
    }
    
    let query: String = args[1].clone();
    let filename: String = args[2].clone();
    let ignore_case: bool = env::var("IGNORE_CASE").is_ok();

    Ok(Config {query, filename, ignore_case})
}

pub fn process_file(filename: &String) -> String {
    println!("Started reading {filename}");

    let contents = fs::read_to_string(filename)
                            .expect("Should have been able to read file");

    contents
}

// passing contents as lifetime to make sure that result lives as long as ubput lives
pub fn search_text<'a>(contents: &'a String, query: &String) -> Vec<&'a str> {
    let mut res: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }

    res
}

pub fn search_text_ignorecase<'a>(contents: &'a String, query: &String) -> Vec<&'a str> {
    let mut res: Vec<&'a str> = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        // we pass query by reference here because contains expects referenced string 
        // .to_lowercase() returns string
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }

    res
}