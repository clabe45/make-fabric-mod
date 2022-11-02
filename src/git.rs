use std::{
    path::{Path, PathBuf},
    process::Command,
};

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

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

pub struct Context {
    path: PathBuf,
}

impl Context {
    pub fn new(path: &Option<&Path>) -> Result<Self, Error> {
        Ok(Self {
            path: path
                .map(|path| path.to_path_buf())
                .unwrap_or_else(|| PathBuf::from(".")),
        })
    }

    pub fn git(&self, args: &[&str]) -> Result<String, Error> {
        let mut command = Command::new("git");
        command.current_dir(&self.path);
        command.args(args);

        let output = command.output()?;
        let stdout = String::from_utf8(output.stdout)?;
        let stderr = String::from_utf8(output.stderr)?;

        if !output.status.success() {
            return Err(Error {
                message: format!("git failed: {}", stderr),
            });
        }

        Ok(stdout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git() {
        let context = Context::new(&None).unwrap();
        let output = context.git(&["--version"]).unwrap();
        assert!(output.starts_with("git version"));
    }
}
