use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub file: Option<String>,
    #[arg(long, conflicts_with("file"))]
    pub pub_key: Option<u32>,
    #[arg(long, conflicts_with("file"))]
    pub priv_key: Option<u32>,
    #[arg(short, long, conflicts_with("file"))]
    pub module: Option<u32>,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    GenerateNewKey {
        #[arg(short, long)]
        output: Option<String>,
    },
    EncryptMessage {
        #[arg(short, long, conflicts_with("message"))]
        file: Option<String>,
        #[arg(short, long)]
        message: Option<String>,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(short, long)]
        key: String,
    },
    DecryptMessage {
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
