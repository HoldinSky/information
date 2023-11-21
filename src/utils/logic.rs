use super::file_reader::FileReader;
use super::formulae::parse_chunk_for_unique_bytes;
use super::terminal::get_input_from_user;
use super::{clear, get_file, get_stats_and_print, parse_file};
use crate::bit_map::BitMap;
use crate::constants::{ARCHIVE_EXTENSION, DICTIONARY_END};
use crate::shannon_fano::encode;
use crate::types::FileInfo;
use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind, Write};
use std::ops::Add;
use std::os::unix::fs::FileExt;
use trie_rs::Trie;

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

    let (dictionary, file_size) = parse_file(&file.0);

    get_stats_and_print(&dictionary, file_size);
}

// Structure of encoded file:
// "Header" -> "Data"
// "Header": N entries of "Mapping"
// "Mapping": 1st byte - original symbol, 2nd byte - LENGTH of BIT CODE stored in next 'ceil(LENGTH / 8)' bytes of "Mapping"
// *Note. Last bits that are not filled in last byte of BIT CODE are set to 0

pub fn encode_file(default: Option<FileInfo>) -> Result<(), Error> {
    let (file, input_path) = match default {
        Some(f) => f,
        None => match get_file() {
            Ok(f) => f,
            Err(err) => return Err(Error::new(ErrorKind::NotFound, err)),
        },
    };

    let out_path = input_path.to_owned().add(ARCHIVE_EXTENSION);
    if let Err(_) = std::fs::remove_file(&out_path) {
        (); // probably could not delete file as it does not exist
    }

    let dictionary = encode(parse_file(&file));
    let mut encoded_file = File::create(&out_path).unwrap();

    if let Err(err) = create_dictionary_header(&dictionary, &mut encoded_file) {
        return Err(err);
    }
    if let Err(err) = write_compressed_file(&file, &mut encoded_file, &dictionary) {
        return Err(err);
    }

    println!("Compressing completed succesfully!");
    Ok(())
}

fn create_dictionary_header(dict: &HashMap<u8, Vec<bool>>, file: &mut File) -> Result<(), Error> {
    let write_error = "Could not parse directory into file";
    let mut bitmap = BitMap::new();

    for key in dict.keys() {
        // write original symbol
        if let Err(_) = file.write(&[key.clone()]) {
            return Err(Error::new(ErrorKind::Other, write_error));
        };

        let bit_sequence = dict.get(key).unwrap();
        // write code length
        if let Err(_) = file.write(&[bit_sequence.len() as u8]) {
            return Err(Error::new(ErrorKind::Other, write_error));
        };

        // get symbol's code and write it to file
        bitmap.add_sequence(bit_sequence);
        if let Err(_) = bitmap.flush_to_file(file) {
            return Err(Error::new(ErrorKind::Other, write_error));
        };
    }

    // mark delimiter that corresponds to the end of the file
    if let Err(_) = file.write(&DICTIONARY_END) {
        Err(Error::new(ErrorKind::Other, write_error))
    } else {
        Ok(())
    }
}

fn write_compressed_file(
    original_file: &File,
    encoded_file: &mut File,
    dict: &HashMap<u8, Vec<bool>>,
) -> Result<(), Error> {
    let mut reader = FileReader::new();
    let mut bitmap = BitMap::new();

    reader.read_file_in_chunks(&original_file, None, |buf, bytes_read| {
        for byte in &buf[..bytes_read] {
            if dict.contains_key(byte) {
                bitmap.add_sequence(dict.get(byte).unwrap());
            }
        }

        if let Err(_) = if bytes_read == buf.len() {
            bitmap.flush_filled_to_file(encoded_file)
        } else {
            bitmap.flush_to_file(encoded_file)
        } {
            return Err(Error::new(
                ErrorKind::BrokenPipe,
                "Error while writing to file",
            ));
        };

        Ok(())
    })
}

pub fn decode_file(default: Option<FileInfo>) -> Result<(), Error> {
    let (file, input_path) = match default {
        Some(f) => {
            if !f.1.ends_with(ARCHIVE_EXTENSION) {
                return Err(Error::new(ErrorKind::InvalidInput, "Unsuported file type."));
            } else {
                f
            }
        }
        None => match get_file() {
            Ok(f) => f,
            Err(err) => return Err(Error::new(ErrorKind::NotFound, err)),
        },
    };

    let out_path = &input_path[..input_path
        .rfind(ARCHIVE_EXTENSION)
        .unwrap_or(input_path.len())];
    let out_path = increment_file_index(out_path);

    if let Err(_) = std::fs::remove_file(&out_path) {
        // probably could not delete file as it does not exist
        ();
    }

    let mut decoded_file = match File::create(&out_path) {
        Ok(f) => f,
        Err(_) => return Err(Error::new(ErrorKind::Other, "Could not create output file")),
    };

    let mut dictionary: HashMap<Vec<bool>, u8> = HashMap::new();
    let mut offset: usize = 0;

    read_dictionary_header(&file, &mut dictionary, &mut offset);

    let trie = build_codes_trie(&dictionary);

    match write_decoded_file(&file, offset, &mut decoded_file, &trie, &dictionary) {
        Ok(_) => {
            println!("Decompressing completed successfully!");
            Ok(())
        }
        Err(err) => Err(err),
    }
}

fn read_dictionary_header(file: &File, dict: &mut HashMap<Vec<bool>, u8>, offset: &mut usize) {
    let mut buf = [0_u8; 2048]; // 2Kb buffer
    file.read_at(&mut buf, 0).unwrap();

    let mut bitmap = BitMap::new();
    // reading dictionary from header of the file
    while &buf[*offset..=*offset + 1] != DICTIONARY_END {
        // get original symbol code
        let symbol = buf[*offset];
        *offset += 1;

        // get bit sequence length
        let code_length = buf[*offset];
        *offset += 1;

        // get bit sequence
        let bytes_taken_by_code = code_length.div_ceil(8) as usize;
        for _ in 0..bytes_taken_by_code {
            bitmap.add_byte(buf[*offset]);
            *offset += 1;
        }

        dict.insert(bitmap.get_bits(code_length as usize), symbol);
        bitmap.clear();
    }

    *offset += 2;
}

fn build_codes_trie(dict: &HashMap<Vec<bool>, u8>) -> Trie<bool> {
    let mut builder = trie_rs::TrieBuilder::new();
    for code in dict.keys() {
        builder.push(code);
    }

    builder.build()
}

fn write_decoded_file(
    encoded_file: &File,
    content_offset: usize,
    decoded_file: &mut File,
    trie: &Trie<bool>,
    dictionary: &HashMap<Vec<bool>, u8>,
) -> Result<(), Error> {
    let write_error = "Could not parse directory into file";

    let mut reader = FileReader::new();
    let mut bitmap = BitMap::new();

    let chunk_length = 1000;
    let mut chunk_start = 0;
    let mut decoded_bytes = Vec::new();

    let mut bit_sequence = Vec::new();

    reader.read_file_in_chunks(encoded_file, Some(content_offset), |buf, bytes_read| {
        while chunk_start < bytes_read {
            let bytes_chunk = &buf[chunk_start..min(chunk_start + chunk_length, bytes_read)];
            bitmap.add_bytes(bytes_chunk);

            bit_sequence.append(&mut bitmap.get_all_bits());
            let length = bit_sequence.len();

            bitmap.clear();

            let mut ptr_lo = 0;
            let mut ptr_hi = 1;
            while ptr_hi < length {
                while !trie.exact_match(&bit_sequence[ptr_lo..ptr_hi]) && ptr_hi < length {
                    ptr_hi += 1;
                }
                if ptr_hi >= length {
                    break;
                }

                decoded_bytes.push(*dictionary.get(&bit_sequence[ptr_lo..ptr_hi]).unwrap());

                ptr_lo = ptr_hi;
                ptr_hi += 1;
            }

            if let Err(_) = decoded_file.write(decoded_bytes.as_slice()) {
                return Err(Error::new(ErrorKind::Other, write_error));
            };

            decoded_bytes.clear();

            bit_sequence = Vec::from(&bit_sequence[ptr_lo..]);
            chunk_start += chunk_length;
        }

        chunk_start = 0;
        Ok(())
    })
}

fn increment_file_index(filepath: &str) -> String {
    let mut dot_pos = filepath.find(".").unwrap_or(filepath.len());
    let ext = &filepath[dot_pos..];

    let ind = match filepath.find("_") {
        Some(underscore_pos) => {
            let index = filepath[underscore_pos + 1..dot_pos]
                .parse::<i32>()
                .unwrap_or(0)
                + 1;
            dot_pos = underscore_pos;
            index
        }
        None => 1,
    };

    filepath[..dot_pos]
        .to_owned()
        .add("_")
        .add(&ind.to_string())
        .add(ext)
}
