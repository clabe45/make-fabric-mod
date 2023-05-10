use std::path::PathBuf;

use clap::Parser;

use crate::{code::language::Language, fabric};

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<fabric::Error> for Error {
    fn from(error: fabric::Error) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    author = "Caleb Sacks",
    version = "0.1.1",
    about = "Create a new Fabric mod"
)]
struct Opts {
    #[clap(
        short = 'i',
        long = "id",
        help = "Mod ID. Defaults to the name of the directory",
        default_value = ""
    )]
    mod_id: String,

    #[clap(short = 'n', long = "name", help = "Human-friendly mod name")]
    name: String,

    #[clap(short = 'm', long = "minecraft", help = "Minecraft version (x.y)")]
    minecraft_version: String,

    #[clap(short = 'k', long = "kotlin", help = "Use Kotlin instead of Java")]
    kotlin: bool,

    #[clap(
        short = 'e',
        long = "entrypoint",
        help = "Main class (e.g., 'net.fabricmc.example.ExampleMod')"
    )]
    main_class: String,

    path: PathBuf,
}

pub fn cli() -> Result<(), Error> {
    let opts = Opts::parse();
    let mod_id = if opts.mod_id.is_empty() {
        opts.path.file_name().unwrap().to_str().unwrap().to_string()
    } else {
        opts.mod_id
    };
    let language = if opts.kotlin {
        Language::Kotlin
    } else {
        Language::Java
    };

    fabric::create_mod(&opts.path, &mod_id, &opts.minecraft_version, &language, &opts.main_class, &opts.name)?;
    Ok(())
}
