use crate::cli::Args;
use crate::crypto::{gen_prime, RSA};
use clap::Parser;

pub fn run() {
    // let args = Args::parse();
    // println!("{:?}", args);
    // let a = gen_prime(26);
    // let b = gen_prime(26);
    // println!("{}\n{}", a, b);
    let rsa = RSA::new(8);
    rsa.encrypt("hello".to_owned());
    // println!("{:?}", rsa)
}
