use std::{fs, io::{self, BufRead}, path};

pub fn search (
	query: &str,
	file_paths: impl Iterator<Item = path::PathBuf>,
	ignore_case: bool
) -> impl Iterator<Item = String>
{
	let query = if ignore_case {
		query.to_ascii_lowercase()
	} else {
		query.to_string()
	};

	file_paths.flat_map(move |path| {
		fs::File::open(&path)
			.ok()
			.map(io::BufReader::new)
			.into_iter()
			.flat_map(|reader| reader.lines())
			.filter_map(|line| line.ok())
			.filter({
				let query = query.clone();
				move |line| {
					if ignore_case {
						line.to_ascii_lowercase().contains(&query)
					} else {
						line.contains(&query)
					}
				}
			})
	})
}