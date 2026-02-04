pub mod config;
pub mod traverse;
pub mod search;

use std::{
	error, path
};

use crate::{search::search, traverse::traverse, config::Config};

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let path = path::Path::new(&config.file_path);
    let contents = traverse(path)?;
    let results = search(
        &config.query, contents, config.ignore_case
    );

    for line in results {
        println!("{}", line);
    }

    Ok(())
}