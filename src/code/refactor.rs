use std::path::Path;

use super::language::Language;

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

fn recursive_replace(path: &Path, old: &str, new: &str) -> Result<(), Error> {
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            recursive_replace(&path, old, new)?;
        } else {
            let content = std::fs::read_to_string(&path)?;
            let content = content.replace(old, new);
            std::fs::write(&path, content)?;
        }
    }
    Ok(())
}

pub fn rename_package(
    path: &Path,
    language: &Language,
    old_package: &str,
    new_package: &str,
) -> Result<(), Error> {
    // Move the entrypoint to the correct location
    let base_path = path.join("src/main").join(language.to_string());

    let old_package_path = base_path.join(old_package.replace(".", "/"));
    let new_package_path = base_path.join(new_package.replace(".", "/"));

    // Create the new package directory
    std::fs::create_dir_all(&new_package_path)?;

    // Move the old package directory to the new package directory
    std::fs::rename(&old_package_path, &new_package_path)?;

    // Update the package name in each source file
    recursive_replace(&base_path, old_package, new_package)?;

    Ok(())
}

pub fn rename_class(
    path: &Path,
    language: &Language,
    old_class: &str,
    new_class: &str,
) -> Result<(), Error> {
    // Move the entrypoint to the correct location
    let base_path = path.join("src/main").join(language.to_string());

    let old_class_path = base_path.join(old_class.replace(".", "/") + "." + language.extension());
    let new_class_path = base_path.join(new_class.replace(".", "/") + "." + language.extension());

    // Create the directory if it doesn't exist
    if let Some(parent) = new_class_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Rename the file
    std::fs::rename(&old_class_path, &new_class_path)?;

    // Update the class name in each source file
    let old_class_name = old_class.split('.').last().unwrap();
    let new_class_name = new_class.split('.').last().unwrap();
    recursive_replace(&base_path, old_class_name, new_class_name)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use std::fs;
    use std::io::Write;

    fn create_test_file(path: &Path, content: &str) {
        // Create the directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        // Create the file
        let mut file = fs::File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    #[test]
    fn test_recursive_replace() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("test_file.txt");
        create_test_file(&test_file, "Hello, world!");

        recursive_replace(&temp_dir.path(), "world", "universe").unwrap();

        let content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(content, "Hello, universe!");
    }

    #[rstest]
    #[case(Language::Java)]
    #[case(Language::Kotlin)]

    fn test_rename_package(#[case] language: Language) {
        let temp_dir = tempfile::tempdir().unwrap();
        let old_file = temp_dir
            .path()
            .join("src/main")
            .join(language.to_string())
            .join("net/fabricmc/example/ExampleMod.".to_string() + language.extension());

        create_test_file(
            &old_file,
            "package net.fabricmc.example;

public class ExampleMod {}",
        );

        rename_package(
            &temp_dir.path(),
            &language,
            "net.fabricmc.example",
            "com.example",
        )
        .unwrap();

        let new_file = temp_dir
            .path()
            .join("src/main")
            .join(language.to_string())
            .join("com/example/ExampleMod.".to_string() + language.extension());

        let content = fs::read_to_string(&new_file).unwrap();
        assert_eq!(
            content,
            "package com.example;

public class ExampleMod {}"
        );
    }

    #[rstest]
    #[case(Language::Java)]
    #[case(Language::Kotlin)]
    fn test_rename_class(#[case] language: Language) {
        let temp_dir = tempfile::tempdir().unwrap();
        let old_file = temp_dir
            .path()
            .join("src/main")
            .join(language.to_string())
            .join("net/fabricmc/example/ExampleMod.".to_string() + language.extension());

        create_test_file(
            &old_file,
            "package net.fabricmc.example;

public class ExampleMod {}",
        );

        rename_class(
            &temp_dir.path(),
            &language,
            "net.fabricmc.example.ExampleMod",
            "com.example.ExampleMod2",
        )
        .unwrap();

        let new_file = temp_dir
            .path()
            .join("src/main")
            .join(language.to_string())
            .join("com/example/ExampleMod2.".to_string() + language.extension());

        let content = fs::read_to_string(&new_file).unwrap();
        assert_eq!(
            content,
            "package net.fabricmc.example;

public class ExampleMod2 {}"
        );
    }
}
