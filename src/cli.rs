use std::path::PathBuf;

use clap::Parser;

use crate::fabric;

#[derive(Parser, Debug)]
#[command(
    author = "Caleb Sacks",
    version = "0.1.0",
    about = "Create a new Fabric mod"
)]
struct Opts {
    #[clap(short = 'k', long = "kotlin", help = "Use Kotlin instead of Java")]
    kotlin: bool,

    path: PathBuf,
}

pub fn cli() {
    let opts = Opts::parse();
    fabric::create_mod(&opts.path, opts.kotlin).unwrap();
}