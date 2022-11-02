use std::path::Path;

use crate::git;

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

pub fn create_mod(path: &Path) -> Result<(), Error> {
    // Clone the Kotlin example mod
    let global = git::Context::new(&None)?;
    global.git(&[
        "clone",
        "https://github.com/clabe45/fabric-example-mod-kotlin.git",
        path.to_str().unwrap(),
    ])?;

    // Remove the .git directory
    let git_dir = path.join(".git");
    std::fs::remove_dir_all(git_dir)?;

    // Re-initialize the git repository
    let repo = git::Context::new(&Some(path))?;
    repo.git(&["init"])?;

    Ok(())
}
