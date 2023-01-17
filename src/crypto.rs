use std::{char::from_u32, fs, io::Error};

use rand::Rng;

#[derive(Debug)]
pub struct RSA {
    pub pub_key: u32,
    pub priv_key: u32,
    pub module: u32,
}

impl RSA {
    pub fn new(bits: u32) -> Self {
        let mut p = gen_prime(bits);
        let mut q = gen_prime(bits);

        if p < q {
            let t = p;
            p = q;
            q = t;
        }

        let n = p * q;
        let fi = (p - 1) * (q - 1);
        let e = gen_prime(bits - 1);
        let d = multiplicative_inverse(e, fi);

        Self {
            pub_key: e,
            priv_key: d,
            module: n,
        }
    }

    pub fn encrypt(&self, message: String) -> String {
        message
            .chars()
            .map(|c| discrete_pow(c as u32, self.pub_key, self.module))
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn decrypt(&self, message: String) -> String {
        message
            .split(" ")
            .into_iter()
            .filter_map(|c| c.parse::<u32>().ok())
            .filter_map(|c| from_u32(discrete_pow(c, self.priv_key, self.module)))
            .collect()
    }

    pub fn read_from_file(file: &str) -> Result<Self, Error> {
        let keys = fs::read_to_string(file)?;
        let keys = keys
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .filter_map(|i| i.parse::<u32>().ok())
            .collect::<Vec<u32>>();
        let (pub_key, priv_key, module) = (keys[0], keys[1], keys[2]);

        Ok(RSA {
            pub_key,
            priv_key,
            module,
        })
    }
}

fn discrete_pow(num: u32, exp: u32, module: u32) -> u32 {
    let mut t: u128 = 1;
    let mut num: u128 = num as u128;
    let mut exp: u128 = exp as u128;
    let module: u128 = module as u128;

    while exp > 0 {
        if exp % 2 != 0 {
            t = (t * num) % module;
        }
        num = (num * num) % module;
        exp = exp / 2;
    }

    (t % module) as u32
}

fn gen_prime(bits: u32) -> u32 {
    let mut rng = rand::thread_rng();
    loop {
        let num: u32 = rng.gen_range(2u32.pow(bits - 1)..2u32.pow(bits));
        if is_prime(num) {
            return num;
        }
    }
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    let mut w = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += w;
        w = 6 - w;
    }
    return true;
}
fn multiplicative_inverse(e: u32, fi: u32) -> u32 {
    let mut k = 1;
    loop {
        let result = (1 + (k * fi)) as f64 / e as f64;
        let rounded = (result * 10_000f64).round() / 10_000f64;
        if (rounded % 1.0) == 0.0 {
            return result as u32;
        } else {
            k += 1;
        }
    }
}
