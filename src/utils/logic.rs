use std::fs::File;

use super::terminal::{get_input_from_user, get_line_from_user};
use crate::shannon_fano::encode;
use crate::utils::formulae::parse_chunk_for_unique_bytes;
use crate::utils::{clear, get_stats_and_print, parse_file};

pub fn calculate_user_input_stats() {
    println!("Please input the message followed by hitting 'ctrl+d'");

    let buf = get_input_from_user();

    let mut dictionary = [0; 256];
    let mut size = 0;

    parse_chunk_for_unique_bytes(&mut dictionary, &buf, &mut size);

    clear();
    get_stats_and_print(&dictionary, size);
}

pub fn calculate_file_stats() {
    let file = match get_file() {
        Some(f) => f,
        None => return,
    };

    let (dictionary, file_size) = parse_file(file);

    get_stats_and_print(&dictionary, file_size);
}

pub fn encode_file() {
    let file = match get_file() {
        Some(f) => f,
        None => return,
    };

    let (dictionary, file_size) = parse_file(file);

    let codes = encode(&dictionary, file_size);
}

fn get_file() -> Option<File> {
    println!("Please enter full path to file and hit 'enter'");

    let user_input = String::from_utf8(get_line_from_user().into_bytes()).unwrap();
    let path = user_input.trim();

    match File::open(path) {
        Ok(file) => Some(file),
        Err(_) => {
            println!("File with this path and name does not exist");
            return None;
        }
    }
}
