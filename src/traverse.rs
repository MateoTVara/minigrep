use std::{io, fs, path};

pub fn traverse(path: &path::Path) -> io::Result<Vec<String>> {
	let mut results = vec![];
	if path.is_dir() {
		for entry in fs::read_dir(&path)? {
			let entry = entry?;
			let path = entry.path();
			results.extend(traverse(&path)?);
		}
	} else {
		let file_result = fs::read_to_string(path);
		if file_result.is_ok() {
			results.push(file_result?);
		}
	}
	Ok(results)
}