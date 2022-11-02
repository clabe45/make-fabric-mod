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

pub fn create_mod(path: &Path, kotlin: bool) -> Result<(), Error> {
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

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::fabric;

    #[test]
    fn test_create_mod() {
        let path = PathBuf::from("test");
        fabric::create_mod(&path, false).unwrap();

        let git_dir = path.join(".git");
        assert!(git_dir.exists());

        std::fs::remove_dir_all(path).unwrap();
    }
}
