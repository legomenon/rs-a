use crate::cli::Args;
use crate::crypto::RSA;
use clap::Parser;

pub fn run() {
    let args = Args::parse();
    println!("{:?}", args);

    let rsa = RSA::new(10);
    let b = rsa.encrypt("hello world".to_owned());
    dbg!(&b);
    let c = rsa.decrypt(b);
    dbg!(c);
}
