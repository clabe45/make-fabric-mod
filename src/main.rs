use std::io::stdout;

use crossterm::{cursor, ExecutableCommand};

mod cli;
mod code;
mod fabric;
mod git;

fn main() {
    let mut stdout = stdout();
    stdout.execute(cursor::Hide).unwrap();
    cli::cli().unwrap();
    stdout.execute(cursor::Show).unwrap();
}
