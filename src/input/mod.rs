use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, stdin};

use crate::statistic;
use crate::utils::{calculate_and_print, clear, parse_file};

pub fn process_terminal_input() {
    println!("Please input the message followed by hitting 'ctrl+d'");

    let mut stdin = stdin();
    let mut buffer = String::new();

    stdin.read_to_string(&mut buffer).unwrap();

    let mut map = HashMap::new();
    let mut size: i64 = 0;

    statistic::parse_chunk_for_unique_bytes(&mut map, buffer.as_bytes(), &mut size);

    clear();
    calculate_and_print(&map, size);
}

pub fn process_file_input() {
    println!("Please input full path to file and hit 'enter'");

    let mut stdin = stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();

    let path = match File::open(input.trim()) {
        Ok(_) => { input.trim() }
        Err(_) => {
            println!("File with this name does not exist");
            return;
        }
    };

    let (map, size) = parse_file(path);

    calculate_and_print(&map, size);
}