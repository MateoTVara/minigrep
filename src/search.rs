pub fn search<'a> (
	query: &str,
	contents: Vec<String>,
	ignore_case: bool
) -> Vec<String>
{
	let query = if ignore_case {
		query.to_ascii_lowercase()
	} else {
		query.to_string()
	};

	contents
		.iter()
		.flat_map(|file_content| {
			file_content
				.lines()
				.filter(|line| {
					if ignore_case {
						line.to_ascii_lowercase().contains(&query)
					} else {
						line.contains(&query)
					}
				})
				.map(|line| line.to_string())
		})
		.collect()
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = vec!["\
Rust:
safe, fast, productive.
Pick three.
Duct tape.".to_string()];

		let results: Vec<_> = search(query, contents, false);

        assert_eq!(
			vec!["safe, fast, productive."],
			results
		);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = vec!["\
Rust:
safe, fast, productive.
Pick three.
Trust me.".to_string()];

		let results: Vec<_> = search(query, contents, true);
		
        assert_eq!(
            vec!["Rust:", "Trust me."],
			results
        );
    }
}