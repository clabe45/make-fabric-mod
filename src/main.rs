mod cli;
mod code;
mod fabric;
mod file;
mod git;

fn main() {
    cli::cli().unwrap();
}
