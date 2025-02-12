use core::str;
use std::{fs::File, io::BufRead, path::Path};
use sha2::{Sha256, Digest};
use rayon::prelude::*;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use lazy_static::lazy_static;
use clap::Parser;

mod args;
use args::CrackCommand;

lazy_static! {
    static ref PASSWORD_FOUND: AtomicBool = AtomicBool::new(false);
}

fn load_password_dictionary() -> Vec<String> {
    let path = Path::new("password-dictionary.txt");
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);
    let words: Vec<String> = reader
    .lines()                         
    .filter_map(Result::ok)           
    .map(|line| line.trim().to_string())
    .filter(|line| !line.is_empty())
    .collect();
    words
}

fn sha256_hasher(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result_bytes = hasher.finalize();
    result_bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}

fn dictionary_attack(hash_to_crack: String) -> Option<String> {
    let list = load_password_dictionary();

    if let Some(found_password) = list.into_par_iter().find_any(|password| sha256_hasher(&password) == hash_to_crack) {
        Some(found_password)
    } else {
        None
    }
}

fn generate_combinations(charset: &[char], length: usize, prefix: String, hash_to_crack: &str) {
    if PASSWORD_FOUND.load(Ordering::Relaxed) {
        return;
    }

    if prefix.len() == length {
        if sha256_hasher(&prefix) == hash_to_crack {
            println!("Found password: {}", prefix);
            PASSWORD_FOUND.store(true, Ordering::Relaxed);
            return;
        }
        return;
    }
    
    for &ch in charset {
        let mut new_prefix = prefix.clone();
        new_prefix.push(ch);
        generate_combinations(charset, length, new_prefix, hash_to_crack);
    }
}

fn brute_force_parallel(hash_to_crack: &str) {
    let charset: Vec<char> = "qwertyuiopasdfghjklzxcvbnm1234567890".chars().collect();
    let size = 3..=20;
    
    size.into_par_iter().for_each(|i| {
        if !PASSWORD_FOUND.load(Ordering::Relaxed) {
            generate_combinations(&charset, i, String::new(), hash_to_crack);
        }
    });
}

fn main() {
    let arg = CrackCommand::parse();
    let hash_to_crack = arg.hash;
    
    if let Some(found) = dictionary_attack(hash_to_crack.to_string()) {
        println!("Found password using dictionary attack: {}", found);
        return;
    } else {
        println!("Password not found in dictionary, trying brute force...");
    }
    
    brute_force_parallel(&hash_to_crack);
}