use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args);



    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: Vec<String>) -> Config {
        let mut args = args.into_iter();
        args.next();
        let query = args.next().expect("Missing query");
        let file_path = args.next().expect("Missing file path");
        Config { query, file_path }
    }
}