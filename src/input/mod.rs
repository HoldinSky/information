use std::collections::HashMap;
use std::fs::File;

use crate::statistic;
use crate::terminal::{take_input, take_line};
use crate::utils::{calculate_and_print, clear, parse_file};

pub fn process_terminal_input() {
    println!("Please input the message followed by hitting 'ctrl+d'");

    let buf = take_input();

    let mut map = HashMap::new();
    let mut size: i64 = 0;

    statistic::parse_chunk_for_unique_bytes(&mut map, &buf, &mut size);

    clear();
    calculate_and_print(&map, size);
}

pub fn process_file_input() {
    println!("Please input full path to file and hit 'enter'");

    let input = String::from_utf8(take_line()).unwrap();
    let path = input.trim();

    let path = match File::open(path) {
        Ok(_) => { path }
        Err(_) => {
            println!("File with this name does not exist");
            return;
        }
    };

    let (map, size) = parse_file(path);

    calculate_and_print(&map, size);
}