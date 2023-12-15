pub mod constants;
pub mod file_reader;
pub mod formulae;
pub mod logic;
pub mod terminal;

use self::formulae::parse_chunk_for_unique_bytes;
use crate::types::{FileInfo, FileStats};
use crate::utils::file_reader::FileReader;
use crate::utils::formulae::{
    calculate_entropy, calculate_information_amount, calculate_max_entropy, calculate_redundancy,
};
use crate::utils::terminal::get_line_from_user;
use std::env;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::os::unix::ffi::OsStringExt;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn parse_file(file: &File) -> FileStats {
    let mut reader = FileReader::new();

    let mut dictionary: [u64; 256] = [0; 256];
    let mut file_size: u64 = 0;

    reader
        .read_file_in_chunks(&file, None, |buf, bytes_read| {
            parse_chunk_for_unique_bytes(&mut dictionary, &buf[..bytes_read], &mut file_size);
            Ok(())
        })
        .unwrap();

    (dictionary, file_size)
}

pub fn get_stats_and_print(dictionary: &[u64; 256], file_size: u64) {
    let mut distinct_count = 0;
    dictionary.map(|b| {
        if b != 0 {
            distinct_count += 1
        }
    });

    let info_amount = calculate_information_amount(dictionary, file_size);
    let entropy = calculate_entropy(info_amount, file_size);
    let max_entropy = calculate_max_entropy(distinct_count);
    let redundancy = calculate_redundancy(entropy, (entropy as u8) / 8 + 1);

    println!("Input is {} bytes long and contain {} unique characters. Information amount={:.2}, Entropy={:.2}, Max.Entropy={:.2}, Redundancy={:.2}",
             file_size,
             distinct_count,
             info_amount,
             entropy,
             max_entropy,
             redundancy
    );
}

pub fn print_entries_of_current_dir() {
    println!("Entries in {:?}:", env::current_dir().unwrap());
    for file in terminal::list_files().unwrap() {
        println!("{}", String::from_utf8(file.into_vec()).unwrap());
    }
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

fn get_file() -> Result<FileInfo, String> {
    println!("Please enter full path to file and hit 'enter'");

    let user_input = String::from_utf8(get_line_from_user().into_bytes()).unwrap();
    let path = user_input.trim();

    match File::open(path) {
        Ok(file) => Ok((file, path.to_owned())),
        Err(_) => {
            return Err("File by specified full path does not exist".to_owned());
        }
    }
}
