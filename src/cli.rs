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
    path: PathBuf,
}

pub fn cli() {
    let opts = Opts::parse();
    fabric::create_mod(&opts.path).unwrap();
}
