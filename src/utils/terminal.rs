use std::error::Error;
use std::ffi::OsString;
use std::fmt::{Display, Formatter};
use std::fs::DirEntry;
use std::io::{stdin, stdout, Read, Write};
use std::path::{Path, PathBuf};
use std::{env, fs};

pub fn get_input_from_user() -> Vec<u8> {
    print!("# ");
    stdout().flush().unwrap();

    let mut stdin = stdin();
    let mut buffer = Vec::new();

    stdin
        .read_to_end(&mut buffer)
        .expect("Could not read input");

    buffer
}

pub fn get_line_from_user() -> String {
    print!("# ");
    stdout().flush().unwrap();

    let stdin = stdin();
    let mut buffer = String::new();

    stdin.read_line(&mut buffer).expect("Could not read input");

    buffer
}

fn get_current_dir() -> Result<PathBuf, Box<dyn Error>> {
    let dir = env::current_dir()?;
    Ok(dir)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(true)
}

pub struct BadPathError {
    path: String,
}

impl BadPathError {
    pub fn new(path: &str) -> Self {
        Self {
            path: String::from(path),
        }
    }
}

impl Display for BadPathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not find directory: {}", self.path)
    }
}

pub fn change_dir(new_dir: &str) -> Result<(), BadPathError> {
    match env::set_current_dir(
        Path::new(new_dir)
            .canonicalize()
            .unwrap_or(env::current_dir().unwrap()),
    ) {
        Ok(_) => Ok(()),
        Err(_) => Err(BadPathError::new(new_dir)),
    }
}

pub fn list_files() -> Result<Vec<OsString>, Box<dyn Error>> {
    let current_dir = get_current_dir()?;

    let dir = fs::read_dir(current_dir)?;

    let visible_entries: Vec<_> = dir
        .filter_map(|entry| entry.ok())
        .filter(|e| !is_hidden(&e))
        .map(|e| e.file_name())
        .collect();

    Ok(visible_entries)
}
