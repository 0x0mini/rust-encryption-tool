use clap::{App, Arg};
use std::fs;
use std::io::{self, Write};

mod encryption;

fn main() {
    let matches = App::new("File Encryption Tool")
        .version("1.0")
        .author("Your Name. <your_email@example.com>")
        .about("Encrypts or decrypts files")
        .arg(
            Arg::with_name("action")
                .help("The action to perform: encrypt or decrypt")
                .required(true)
                .possible_values(&["encrypt", "decrypt"])
                .index(1),
        )
        .arg(
            Arg::with_name("input_file")
                .help("The input file path")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("output_file")
                .help("The output file path")
                .required(true)
                .index(3),
        )
        .arg( // Adding a new argument for the encryption key
            Arg::with_name("key")
                .help("The encryption/decryption key")
                .required(true)
                .index(4),
        )
        .get_matches();

    let action = matches.value_of("action").unwrap();
    let input_file = matches.value_of("input_file").unwrap();
    let output_file = matches.value_of("output_file").unwrap();
    let key = matches.value_of("key").unwrap().as_bytes(); // Assuming a simple string key for XOR

    match action {
        "encrypt" => {
            match encrypt_file(input_file, output_file, key) {
                Ok(_) => println!("File encrypted successfully."),
                Err(e) => eprintln!("Error encrypting file: {}", e),
            }
        }
        "decrypt" => {
            match decrypt_file(input_file, output_file, key) {
                Ok(_) => println!("File decrypted successfully."),
                Err(e) => eprintln!("Error decrypting file: {}", e),
            }
        }
        _ => eprintln!("Invalid action. Use 'encrypt' or 'decrypt'."),
    }
}

fn encrypt_file(input_path: &str, output_path: &str, key: &[u8]) -> io::Result<()> {
    let data = fs::read(input_path)?;
    let encrypted_data = encryption::xor_encrypt_decrypt(&data, key);
    fs::write(output_path, encrypted_data)?;
    Ok(())
}

fn decrypt_file(input_path: &str, output_path: &str, key: &[u8]) -> io::Result<()> {
    let data = fs::read(input_path)?;
    let decrypted_data = encryption::xor_encrypt_decrypt(&data, key);
    fs::write(output_path, decrypted_data)?;
    Ok(())
}

mod encryption {
    pub fn xor_encrypt_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
        data.iter()
            .enumerate()
            .map(|(i, &byte)| byte ^ key[i % key.len()])
            .collect()
    }

    // You can keep these as fallback or simple methods for later comparison
    pub fn encrypt(data: &[u8]) -> Vec<u8> {
        data.iter().rev().cloned().collect()
    }

    pub fn decrypt(data: &[u8]) -> Vec<u8> {
        data.iter().rev().cloned().collect()
    }
}