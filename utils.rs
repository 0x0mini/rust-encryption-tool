use std::env;
use std::fs;
use std::path::Path;
use std::io::{self, Write};

pub fn validate_file_path(path: &str) -> Result<(), String> {
    let path_obj = Path::new(path);
    if path_obj.exists() && path_obj.is_file() {
        Ok(())
    } else {
        Err(format!("Path does not exist or is not a file: {}", path))
    }
}

pub fn can_read_file(path: &str) -> Result<(), String> {
    fs::File::open(path)
        .map(|_| ())
        .map_err(|e| format!("Cannot read file: {}. Error: {}", path, e))
}

pub fn can_write_file(path: &str) -> Result<(), String> {
    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .map(|_| ())
        .map_err(|e| format!("Cannot write to file: {}. Error: {}", path, e))
}

pub fn log_info(message: &str) {
    match env::var("LOG_PATH") {
        Ok(path) => {
            let mut file = match fs::OpenOptions::new().append(true).create(true).open(&path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Failed to open log file at {}. Error: {}", path, e);
                    return;
                }
            };
            if writeln!(file, "INFO: {}", message).is_err() {
                eprintln!("Failed to write to log file at {}", path);
            }
        },
        Err(_) => println!("INFO: {}", message),
    }
}

pub fn log_error(message: &str) {
    match env::var("LOG_PATH") {
        Ok(path) => {
            let mut file = match fs::OpenOptions::new().append(true).create(true).open(&path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Failed to open log file for error logging at {}. Error: {}", path, e);
                    return;
                }
            };
            if writeln!(file, "ERROR: {}", message).is_err() {
                eprintln!("Failed to write error to log file at {}", path);
            }
        },
        Err(_) => eprintln!("ERROR: {}", message),
    }
}