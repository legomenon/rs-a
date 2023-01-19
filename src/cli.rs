use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// generates new RSA key
    Generate {
        /// output destination
        #[arg(short, long)]
        output: Option<String>,
    },

    /// encrypt provided data
    Encrypt {
        /// file for encryption
        #[arg(short, long, conflicts_with("message"))]
        file: Option<String>,
        /// message for encryption
        #[arg(short, long)]
        message: Option<String>,
        /// output destination
        #[arg(short, long)]
        output: Option<String>,
        /// rsa key file
        #[arg(short, long)]
        key: String,
    },

    /// decrypt provided data
    Decrypt {
        /// file for decryption
        #[arg(short, long, conflicts_with("message"))]
        file: Option<String>,
        /// message for decryption
        #[arg(short, long)]
        message: Option<String>,
        /// output destination
        #[arg(short, long)]
        output: Option<String>,
        /// rsa key file
        #[arg(short, long)]
        key: String,
    },
}
