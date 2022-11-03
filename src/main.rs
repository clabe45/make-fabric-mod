use std::io::stdout;

use crossterm::{cursor, ExecutableCommand};

mod cli;
mod code;
mod fabric;
mod git;

fn main() {
    cli::cli().unwrap();
}
