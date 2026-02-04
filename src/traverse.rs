use std::{io, fs, path};

pub fn traverse(path: &path::Path) -> io::Result<impl Iterator<Item = path::PathBuf>> {
	let mut files_paths = Vec::new();
	collect_files_paths(&path, &mut files_paths)?;

	Ok(files_paths.into_iter())
}

pub fn collect_files_paths(
	path: &path::Path,
	files: &mut Vec<path::PathBuf>
) -> io::Result<()>
{
	if path.is_file() {
		files.push(path.to_path_buf());
	} else {
		for entry in fs::read_dir(path)? {
			let entry = entry?;
			let entry_path = entry.path();
			collect_files_paths(&entry_path, files)?;
		}
	}

	Ok(())
}
