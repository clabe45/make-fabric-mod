use std::path::Path;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

fn is_dir_empty(path: &Path) -> Result<bool, Error> {
    std::fs::read_dir(path)?
        .next()
        .map_or(Ok(true), |_| Ok(false))
}

pub fn remove_empty_parent_dirs(path: &Path) -> Result<(), Error> {
    let mut path = path.to_path_buf();
    while let Some(parent) = path.parent() {
        if is_dir_empty(parent)? {
            std::fs::remove_dir(parent)?;
            path = parent.to_path_buf();
        } else {
            break;
        }
    }
    Ok(())
}

fn is_text_file(path: &Path) -> bool {
    let extension = path.extension().unwrap_or_default();
    match extension.to_str() {
        Some("gradle") => true,
        Some("java") => true,
        Some("json") => true,
        Some("kt") => true,
        Some("properties") => true,
        _ => false,
    }
}

fn replace_in_file(path: &Path, from: &str, to: &str) -> Result<(), Error> {
    if !is_text_file(path) {
        return Ok(());
    }

    let mut file = std::fs::read_to_string(path)?;
    file = file.replace(from, to);
    std::fs::write(path, file)?;
    Ok(())
}

pub fn recursive_replace(path: &Path, old: &str, new: &str) -> Result<(), Error> {
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            recursive_replace(&path, old, new)?;
        } else {
            replace_in_file(&path, old, new)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs, io::Write, path::Path};

    use super::*;

    fn create_text_file(path: &Path, content: &str) {
        // Create the directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        // Create the file
        let mut file = fs::File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    fn create_binary_file(path: &Path) {
        // Create the directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        // Create the file
        let mut file = fs::File::create(path).unwrap();
        file.write_all(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    }

    #[test]
    fn test_recursive_replace() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("test_file.properties");
        create_text_file(&test_file, "old old old");
        create_binary_file(&temp_dir.path().join("test_file.bin"));

        recursive_replace(&temp_dir.path(), "old", "new").unwrap();

        let content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(content, "new new new");
    }
}
