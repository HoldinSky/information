use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use crate::file_reader::FileReader;
use crate::statistic;
use crate::statistic::{calculate_entropy, calculate_information_amount, calculate_max_entropy, calculate_redundancy};

pub fn parse_file(file_path: &str) -> (HashMap<u8, i64>, i64) {
    let mut reader = FileReader::new();

    let mut map: HashMap<u8, i64> = HashMap::new();
    let mut size: i64 = 0;
    reader.read_file_in_chunks(file_path,
                               |buf| statistic::parse_chunk_for_unique_bytes(&mut map, &buf, &mut size),
    ).unwrap();

    (map, size)
}

pub fn calculate_and_print(map: &HashMap<u8, i64>, size: i64) {
    let unique_char_count = map.len() as i64;

    let info_amount = calculate_information_amount(&map, size);
    let entropy = calculate_entropy(info_amount, size);
    let max_entropy = calculate_max_entropy(unique_char_count);
    let redundancy = calculate_redundancy(entropy, (entropy as u8) / 8 + 1);

    println!("Input is {} bytes long and contain {} unique characters. Information amount={:.2}, Entropy={:.2}, Max.Entropy={:.2}, Redundancy={:.2}",
             size,
             map.len(),
             info_amount,
             entropy,
             max_entropy,
             redundancy
    );
}

pub fn clear() {
    print!("{esc}c", esc = 27 as char);
}

pub fn pause(message: &str) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    print!("{message}");
    stdout.flush().unwrap();
    stdin().events().next();
}