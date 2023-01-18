use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Generate {
        #[arg(short, long)]
        output: Option<String>,
    },
    Encrypt {
        #[arg(short, long, conflicts_with("message"))]
        file: Option<String>,
        #[arg(short, long)]
        message: Option<String>,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(short, long)]
        key: String,
    },
    Decrypt {
        #[arg(short, long, conflicts_with("message"))]
        file: Option<String>,
        #[arg(short, long)]
        message: Option<String>,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(short, long)]
        key: String,
    },
}
