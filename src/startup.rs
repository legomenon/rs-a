use crate::cli::Args;
use clap::Parser;

pub fn run() {
    let args = Args::parse();
    println!("{:?}", args);
}
