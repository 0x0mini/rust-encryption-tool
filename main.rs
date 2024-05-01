use clap::{App, Arg};
use std::fs;
use std::io::{self};

mod encryption;

fn main() {
    let matches = App::new("File Encryption Tool")
        .version("1.0")
        .author("Your Name. <your_email@example.com>")
        .about("Encrypts or decrypts files")
        .arg(
            Arg::new("action")
                .help("The action to perform: encrypt or decrypt")
                .required(true)
                .possible_values(&["encrypt", "decrypt"])
                .index(1),
        )
        .arg(
            Arg::new("input_file")
                .help("The input file path")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("output_file")
                .help("The output file path")
                .required(true)
                .index(3),
        )
        .arg(
            Arg::new("key")
                .help("The encryption/decryption key")
                .required(true)
                .index(4),
        )
        .get_matches();

    let action = matches.value_of("action").expect("Required action argument missing");
    let input_file = matches.value_of("input_file").expect("Required input file argument missing");
    let output_file = matches.value_of("output_file").expect("Required output file argument missing");
    let key = matches.value_of("key").expect("Required key argument missing").as_bytes();

    if let Err(e) = process_file(action, input_file, output_file, key) {
        eprintln!("Error processing file: {}", e);
    }
}

fn process_file(action: &str, input_file: &str, output_file: &str, key: &[u8]) -> io::Result<()> {
    match action {
        "encrypt" => encrypt_file(input_file, output_file, key).map(|_| println!("File encrypted successfully.")),
        "decrypt" => decrypt_file(input_file, output_file, key).map(|_| println!("File decrypted successfully.")),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Invalid action. Use 'encrypt' or 'decrypt'.")),
    }
}

fn encrypt_file(input_path: &str, output_path: &str, key: &[u8]) -> io::Result<()> {
    let data = fs::read(input_path)?;
    let encrypted_data = encryption::xor_encrypt_decrypt(&data, key);
    fs::write(output_path, encrypted_data)
}

fn decrypt_file(input_path: &str, output_path: &str, key: &[u8]) -> io::Result<()> {
    let data = fs::read(input_path)?;
    let decrypted_data = encryption::xor_encrypt_decrypt(&data, key);
    fs::write(output_path, decrypted_data)
}

mod encryption {
    pub fn xor_encrypt_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
        data.iter()
            .enumerate()
            .map(|(i, &byte)| byte ^ key[i % key.len()])
            .collect()
    }
}