use std::{env, error::Error, process};

mod libs;
mod errors;

fn main() {
    let args: Vec<String> = env::args().collect(); // .collect() -> iterator to ds

    // dbg!(args);

    let config_res: Result<libs::Config, Box<dyn Error>> = libs::parse_args(&args);

    if let Ok(config) = config_res {
        let contents: String = libs::process_file(&config.filename);
        let res: Vec<&str>;
        
        if config.ignore_case {
            res = libs::search_text_ignorecase(&contents, &config.query);
        } else {
            res = libs::search_text(&contents, &config.query);
        }

        println!("Found in following lines: ");

        for lines in res {
            println!("{lines}");
        }
    } else {
        println!("Please pass <string to be searched> <filename>");
        process::exit(1);
    }
}
