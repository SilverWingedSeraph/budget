//! System functions for `budget`
use app_dirs::*;
use std::path::PathBuf;

use std::io::prelude::*;
use std::fs::File;

/// Acquire the path to the config file
pub fn get_config_file_path(file_name: &str) -> Result<String, String> {
    // Try getting the home directory string, and concatenate it with the filename
    let mut path: PathBuf;
    match get_app_dir(AppDirType::UserConfig) {
        Ok(config_dir_path) => path = config_dir_path,
        Err(_) => return Err(String::from("Failed to get user config directory!")),
    }
    
    path.push(file_name);
    return Ok(path.to_string_lossy().into_owned());
}

pub fn read_file(file_path: &str) -> Result<String, String>{
    // Try to open the file
    let mut f = match File::open(file_path) {
        Ok(f_val) => f_val,
        Err(r) => return Err(String::from("Failed to open config file ") 
                             + file_path
                             + ": " + r.to_string().as_ref())
    };

    // Try to read the file into a buffer
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => return Ok(s),
        Err(r) => return Err(String::from("Failed to read config file ") 
                             + file_path
                             + ": " + r.to_string().as_ref())
    };
}


pub fn write_file(file_path: &str, data: &str) -> Result<(), String>{
    // Try to open the file
    let mut f = match File::create(file_path) {
        Ok(f_val) => f_val,
        Err(r) => return Err(String::from("Failed to open config file ") 
                             + file_path
                             + ": " + r.to_string().as_ref())
    };

    // Try to read the file into a buffer
    match f.write_all(data.as_bytes()) {
        Ok(_) => return Ok(()),
        Err(r) => return Err(String::from("Failed to write to config file ") 
                             + file_path
                             + ": " + r.to_string().as_ref())
    };
}
