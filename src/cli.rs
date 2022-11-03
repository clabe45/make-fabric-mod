use std::path::PathBuf;

use clap::Parser;

use crate::{code::language::Language, fabric};

#[derive(Parser, Debug)]
#[command(
    author = "Caleb Sacks",
    version = "0.1.0",
    about = "Create a new Fabric mod"
)]
struct Opts {
    #[clap(
        short = 'i',
        long = "id",
        help = "Mod ID. Defaults to the name of the directory"
    )]
    mod_id: String,

    #[clap(short = 'n', long = "name", help = "Mod name")]
    name: String,

    #[clap(short = 'k', long = "kotlin", help = "Use Kotlin instead of Java")]
    kotlin: bool,

    #[clap(
        short = 'm',
        long = "main",
        help = "Package and class name of the main class",
        default_value = "net.fabricmc.example.ExampleMod"
    )]
    main_class: String,

    path: PathBuf,
}

pub fn cli() -> Result<(), fabric::Error> {
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

    fabric::create_mod(&opts.path, &mod_id, &language, &opts.main_class, &opts.name)?;
    Ok(())
}
