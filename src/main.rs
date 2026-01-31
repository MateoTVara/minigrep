use std::{env, fs, process, error::Error};
use minigrep::{search};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Since run() only returns () on the succesful case
    // what only matters here is to capture the error
    // so if let syntax is preferred
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str>
    {
        args.next();
        let query = args.next().ok_or("Missing query")?;
        let file_path = args.next().ok_or("Missing file path")?;
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = search(
        &config.query, &contents, config.ignore_case
    );

    for line in results {
        println!("{}", line);
    }

    Ok(())
}