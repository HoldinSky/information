use std::fs::File;
use std::io::{Error, ErrorKind, Write};

use super::file_reader::FileReader;
use super::terminal::{get_input_from_user, get_line_from_user};
use crate::bit_container::BitContainer;
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
        Ok(f) => f,
        Err(_) => return,
    };

    let (dictionary, file_size) = parse_file(&file);

    get_stats_and_print(&dictionary, file_size);
}

pub fn encode_file(default: Option<File>) -> Result<(), Error> {
    let file = match default {
        Some(f) => f,
        None => match get_file() {
            Ok(f) => f,
            Err(err) => return Err(Error::new(ErrorKind::NotFound, err)),
        },
    };

    let (dictionary, file_size) = parse_file(&file);

    let codes = encode(&dictionary, file_size);

    let mut bitmap = BitContainer::new();
    let mut reader = FileReader::new();
    let out_path = "/home/nazar/prg/rust/labs/tik/samples/out/test";

    if let Err(_) = std::fs::remove_file(out_path) {
        // probably could not delete file as it does not exist
        ();
    }

    let mut compressed_file = File::create(out_path).unwrap();

    let write_error = "Could not parse directory into file";
    for key in codes.keys() {
        // write original symbol
        if let Err(_) = compressed_file.write(&[key.clone()]) {
            return Err(Error::new(ErrorKind::Other, write_error));
        };

        // get symbol's code and write it to file
        bitmap.add_code(codes.get(key).unwrap());
        if let Err(_) = bitmap.flush_to_file(&mut compressed_file) {
            return Err(Error::new(ErrorKind::Other, write_error));
        };

        // place delimiter
        if let Err(_) = compressed_file.write(&[0]) {
            return Err(Error::new(ErrorKind::Other, write_error));
        }
    }

    if let Err(_) = compressed_file.write(&[0, 0]) {
        return Err(Error::new(ErrorKind::Other, write_error));
    }

    reader
        .read_file_in_chunks(&file, |buf| {
            for byte in buf {
                if !codes.contains_key(byte) {
                    continue;
                }
                bitmap.add_code(codes.get(byte).unwrap());
            }
            match bitmap.flush_to_file(&mut compressed_file) {
                Ok(_) => (),
                Err(err) => println!("{}", err),
            };
        })
        .unwrap();

    println!("Compressing completed succesfully!");

    Ok(())
}

fn get_file() -> Result<File, String> {
    println!("Please enter full path to file and hit 'enter'");

    let user_input = String::from_utf8(get_line_from_user().into_bytes()).unwrap();
    let path = user_input.trim();

    match File::open(path) {
        Ok(file) => Ok(file),
        Err(_) => {
            return Err("File with this path and name does not exist".to_owned());
        }
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use std::fs::File;

    #[test]
    fn test_encoding() {
        let file = File::open("/home/nazar/prg/rust/labs/tik/samples/en/test_1.txt").unwrap();

        super::encode_file(Some(file));
    }
}
