use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: Option<String>,
    #[arg(long, conflicts_with("file"))]
    pub_key: Option<u32>,
    #[arg(long, conflicts_with("file"))]
    priv_key: Option<u32>,
    #[arg(short, long, conflicts_with("file"))]
    module: Option<u32>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    GenerateNewKey {
        #[arg(short)]
        output: Option<String>,
    },
    EncryptMessage {
        #[arg(short)]
        message: String,
    },
    DecryptMessage {
        #[arg(short)]
        message: String,
    },
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
