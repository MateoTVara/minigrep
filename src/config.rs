use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_builds_correctly() {
        let args = vec![
            "program_name".to_string(),
            "search_term".to_string(),
            "file.txt".to_string(),
        ];
        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(config.query, "search_term");
        assert_eq!(config.file_path, "file.txt");
        assert_eq!(config.ignore_case, false);
    }

    #[test]
    fn config_missing_arguments() {
        let args = vec!["program_name".to_string()];
        let result = Config::build(args.into_iter());
        assert!(result.is_err());
    }
}