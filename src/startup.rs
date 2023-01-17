use crate::cli::{Args, Commands};
use crate::crypto::RSA;
use clap::Parser;
use std::fs::{read_to_string, File};
use std::io::{Error, Write};

pub fn run() -> Result<(), Error> {
    let args = Args::parse();
    match args.command {
        Some(command) => match command {
            Commands::GenerateNewKey { output } => match output {
                Some(file) => {
                    let rsa = RSA::new(10);
                    let mut output = File::create(file)?;
                    let data = format!("{} {} {}", rsa.pub_key, rsa.priv_key, rsa.module,);
                    write!(output, "{}", &data)?;
                }
                None => {
                    let rsa = RSA::new(10);
                    let data = format!(
                        "{}\n\n {} {} {}\n\n{}",
                        "__________________Begin Rsa Keys________________",
                        rsa.pub_key,
                        rsa.priv_key,
                        rsa.module,
                        "__________________End Rsa Keys__________________"
                    );
                    println!("{}", data)
                }
            },
            Commands::EncryptMessage {
                file,
                message,
                output,
                key,
            } => match (&file, &message, &output, key) {
                (_, _, _, k) => {
                    let message = match (file, message) {
                        (Some(f), None) => read_to_string(f)?,
                        (None, Some(m)) => m,
                        _ => "".to_owned(),
                    };
                    let keys = RSA::read_from_file(&k)?;
                    let encrypted_data = keys.encrypt(message);
                    match output {
                        Some(o) => {
                            let mut output = File::create(o)?;
                            let data = format!("{}", encrypted_data);
                            write!(output, "{}", &data)?;
                        }
                        None => println!("{}", encrypted_data),
                    }
                }
            },
            Commands::DecryptMessage {
                file,
                message,
                output,
                key,
            } => match (&file, &message, &output, key) {
                (_, _, _, k) => {
                    let message = match (file, message) {
                        (Some(f), None) => read_to_string(f)?,
                        (None, Some(m)) => m,
                        _ => "".to_owned(),
                    };
                    let keys = RSA::read_from_file(&k)?;
                    let decrypted_data = keys.decrypt(message);
                    match output {
                        Some(o) => {
                            let mut output = File::create(o)?;
                            let data = format!("{}", decrypted_data);
                            write!(output, "{}", &data)?;
                        }
                        None => println!("{}", decrypted_data),
                    }
                }
            },
        },
        None => todo!(),
    }
    Ok(())
}
