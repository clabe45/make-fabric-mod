use std::path::Path;

use crate::{code::refactor, git};

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<git::Error> for Error {
    fn from(error: git::Error) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

impl From<refactor::Error> for Error {
    fn from(error: refactor::Error) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

pub fn create_mod(path: &Path, kotlin: bool, main_class: &str) -> Result<(), Error> {
    // Clone the Kotlin example mod
    let template_url: &str;
    if kotlin {
        template_url = "https://github.com/clabe45/fabric-example-mod-kotlin";
    } else {
        template_url = "https://github.com/FabricMC/fabric-example-mod";
    }

    let global = git::Context::new(&None)?;
    global.git(&["clone", template_url, path.to_str().unwrap()])?;

    // Remove the .git directory
    let git_dir = path.join(".git");
    std::fs::remove_dir_all(git_dir)?;

    // Re-initialize the git repository
    let repo = git::Context::new(&Some(path))?;
    repo.git(&["init"])?;

    // Rename the package
    let old_package = "net.fabricmc.example";
    let new_package = main_class[..main_class.rfind('.').unwrap()].to_string();
    refactor::rename_package(path, kotlin, &old_package, &new_package)?;

    // Rename the class
    let old_class = new_package + ".ExampleMod";
    let new_class = main_class;
    refactor::rename_class(path, kotlin, &old_class, &new_class)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fabric;

    #[test]
    fn test_create_mod_creates_git_repo() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("test_create_mod_creates_git_repo");
        fabric::create_mod(&path, false, "net.fabricmc.example.ExampleMod").unwrap();

        let git_dir = path.join(".git");
        assert!(git_dir.exists());
    }

    #[test]
    fn test_create_mod_moves_entrypoint() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("test_create_mod_moves_entrypoint");
        fabric::create_mod(&path, false, "net.fabricmc.example2.ExampleMod2").unwrap();

        let entrypoint = path.join("src/main/java/net/fabricmc/example2/ExampleMod2.java");
        assert!(entrypoint.exists());
    }
}
