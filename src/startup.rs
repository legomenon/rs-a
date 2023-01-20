use crate::cli::{Args, Commands};
use crate::crypto::RSA;
use clap::Parser;
use std::fs::{read_to_string, File};
use std::io::{Error, ErrorKind, Write};

pub fn run() -> Result<(), Error> {
    let args = Args::parse();
    match args.command {
        Some(command) => match command {
            Commands::Generate { output } => match output {
                Some(file) => {
                    let rsa = RSA::new(12);
                    let mut output = File::create(file)?;
                    let data = format!("{} {} {}", rsa.pub_key, rsa.priv_key, rsa.module,);
                    write!(output, "{}", &data)?;
                }
                None => {
                    let rsa = RSA::new(12);
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
            Commands::Encrypt {
                file,
                message,
                output,
                key,
            } => {
                let message = match (file, message) {
                    (Some(f), None) => read_to_string(f)?,
                    (None, Some(m)) => m,
                    _ => "".to_owned(),
                };
                let keys = RSA::read_from_file(&key)?;
                let encrypted_data = keys.encrypt(&message);
                match output {
                    Some(o) => {
                        let mut output = File::create(o)?;
                        write!(output, "{}", encrypted_data)?;
                    }
                    None => println!("{}", encrypted_data),
                }
            }
            Commands::Decrypt {
                file,
                message,
                output,
                key,
            } => {
                let message = match (file, message) {
                    (Some(f), None) => read_to_string(f)?,
                    (None, Some(m)) => m,
                    _ => "".to_owned(),
                };
                let keys = RSA::read_from_file(&key)?;
                let decrypted_data = keys.decrypt(&message);
                match output {
                    Some(o) => {
                        let mut output = File::create(o)?;
                        write!(output, "{}", decrypted_data)?;
                    }
                    None => println!("{}", decrypted_data),
                }
            }
        },
        None => {
            return Err(Error::new(
                ErrorKind::Other,
                "Arguments must be provided, for detail information see --help | -h",
            ))
        }
    }
    Ok(())
}
