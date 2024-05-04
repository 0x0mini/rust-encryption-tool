use std::env;
use std::fs;
use std::path::Path;
use std::io::{self, Write};

pub fn is_valid_file_path(file_path: &str) -> Result<(), String> {
    let path_obj = Path::new(file_path);
    if path_obj.exists() && path_obj.is_file() {
        Ok(())
    } else {
        Err(format!("Path does not exist or is not a file: {}", file_path))
    }
}

pub fn check_file_readable(file_path: &str) -> Result<(), String> {
    fs::File::open(file_path)
        .map(|_| ())
        .map_err(|e| format!("Cannot read file: {}. Error: {}", file_path, e))
}

pub fn check_file_writable(file_path: &str) -> Result<(), String> {
    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)
        .map(|_| ())
        .map_err(|e| format!("Cannot write to file: {}. Error: {}", file_path, e))
}

pub fn log_information(message: &str) {
    match env::var("LOG_PATH") {
        Ok(log_file_path) => {
            let mut log_file = match fs::OpenOptions::new().append(true).create(true).open(&log_file_path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Failed to open log file at {}. Error: {}", log_file_path, e);
                    return;
                }
            };
            if writeln!(log_file, "INFO: {}", message).is_err() {
                eprintln!("Failed to write to log file at {}", log_file_path);
            }
        },
        Err(_) => println!("INFO: {}", message),
    }
}

pub fn log_error_message(message: &str) {
    match env::var("LOG_PATH") {
        Ok(log_file_path) => {
            let mut log_file = match fs::OpenOptions::new().append(true).create(true).open(&log_file_path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Failed to open log file for error logging at {}. Error: {}", log_file_path, e);
                    return;
                }
            };
            if writeln!(log_file, "ERROR: {}", message).is_err() {
                eprintln!("Failed to write error to log file at {}", log_file_path);
            }
        },
        Err(_) => eprintln!("ERROR: {}", message),
    }
}