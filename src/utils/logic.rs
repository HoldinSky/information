use super::constants::HAMMING_CODE_LENGTH_KEY;
use super::file_reader::FileReader;
use super::formulae::parse_chunk_for_unique_bytes;
use super::terminal::get_input_from_user;
use super::{clear, get_file, get_stats_and_print, parse_file};
use crate::algorithms::{hamming, huffman, shannon_fano};
use crate::bit_map::BitMap;
use crate::types::{CodeType, EncodingSettings, FileInfo};
use crate::utils::constants::{ARCHIVE_EXTENSION, DICTIONARY_END};
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

    let mut file_reader = FileReader::new(file.0);
    let (dictionary, file_size) = parse_file(&mut file_reader);

    get_stats_and_print(&dictionary, file_size);
}

// Structure of encoded file:
// "Header" -> "Data"
// "Header": N entries of "Mapping"
// "Mapping": 1st byte - original symbol, 2nd byte - LENGTH of BIT CODE stored in next 'ceil(LENGTH / 8)' bytes of "Mapping"
// *Note. Last bits that are not filled in last byte of BIT CODE are set to 0

pub fn encode_file(settings: EncodingSettings) -> Result<(), Error> {
    let (original_file, input_path) = settings.file_info;

    let out_path = input_path.to_owned().add(ARCHIVE_EXTENSION);
    if let Err(_) = std::fs::remove_file(&out_path) {
        (); // probably could not delete file as it does not exist
    }
    let mut file_reader = FileReader::new(original_file);

    let dictionary = match settings.code_type {
        CodeType::ShannonFano => shannon_fano::encode(parse_file(&mut file_reader)),
        CodeType::Huffman => huffman::encode(parse_file(&mut file_reader)),
    };

    let mut output_file = File::create(&out_path).unwrap();

    if let Err(err) =
        create_dictionary_header(&mut output_file, &dictionary, settings.hamming_code_length)
    {
        return Err(err);
    }

    file_reader.rewind();
    if let Err(err) = write_compressed_file(
        &mut file_reader,
        &mut output_file,
        &dictionary,
        settings.hamming_code_length,
    ) {
        return Err(err);
    }

    println!("Compressing completed succesfully!");
    Ok(())
}

fn create_dictionary_header(
    file: &mut File,
    dict: &HashMap<u8, Vec<u8>>,
    hamming_code_length: Option<u8>,
) -> Result<(), Error> {
    let write_error = "Could not parse directory into file";
    let mut bitmap = BitMap::new();

    if let Some(code_length) = hamming_code_length {
        file.write_all(&HAMMING_CODE_LENGTH_KEY)?;
        file.write_all(&[code_length])?;
    }

    for key in dict.keys() {
        // write original symbol
        file.write(&[key.clone()])?;

        let bit_sequence = dict.get(key).unwrap();

        // write code length
        file.write(&[bit_sequence.len() as u8])?;

        // get symbol's code and write it to file
        bitmap.add_bit_sequence(bit_sequence);

        if let Err(_) = bitmap.flush_to_file(file) {
            return Err(Error::new(ErrorKind::Other, write_error));
        };
    }

    // mark delimiter that corresponds to the end of the file
    file.write(&DICTIONARY_END)?;

    Ok(())
}

fn write_compressed_file(
    file_reader: &mut FileReader,
    output_file: &mut File,
    dictionary: &HashMap<u8, Vec<u8>>,
    hamming_code_length: Option<u8>,
) -> Result<(), Error> {
    let mut bitmap = BitMap::new();

    if let Some(hmcl) = hamming_code_length {
        let data_len = hamming::data_length(hmcl as usize);

        file_reader.read_file_in_chunks(|buf, end_of_file| {
            transform_data_to_hamming_codes(
                output_file,
                dictionary,
                &mut bitmap,
                buf,
                end_of_file,
                data_len,
            )
        })?;
    } else {
        file_reader.read_file_in_chunks(|buf, end_of_file| {
            transofrm_data_to_raw(output_file, dictionary, &mut bitmap, buf, end_of_file)
        })?;
    };

    Ok(())
}

fn transform_data_to_hamming_codes(
    output_file: &mut File,
    dictionary: &HashMap<u8, Vec<u8>>,
    bitmap: &mut BitMap,
    buf: &[u8],
    end_of_file: bool,
    data_len: usize,
) -> Result<(), Error> {
    let chunk_size = 1024;
    let buf_len = buf.len();

    let mut data_container = BitMap::new();

    for chunk_pos in (0..buf_len).step_by(chunk_size) {
        let valid_bytes_chunk = &buf[chunk_pos..min(chunk_pos + chunk_size, buf_len)];

        for byte in valid_bytes_chunk {
            if dictionary.contains_key(byte) {
                data_container.add_bit_sequence(dictionary.get(byte).unwrap());
            }
        }
    }

    let data_bits = data_container.get_all_bits();

    bitmap.add_bit_sequence(&hamming::add_parity_package(&data_bits, data_len));

    if let Err(_) = if end_of_file {
        bitmap.flush_to_file(output_file)
    } else {
        bitmap.flush_filled_to_file(output_file)
    } {
        return Err(Error::new(
            ErrorKind::BrokenPipe,
            "Error while writing to file",
        ));
    };

    Ok(())
}

fn transofrm_data_to_raw(
    output_file: &mut File,
    dictionary: &HashMap<u8, Vec<u8>>,
    bitmap: &mut BitMap,
    buf: &[u8],
    end_of_file: bool,
) -> Result<(), Error> {
    for byte in buf {
        if dictionary.contains_key(byte) {
            bitmap.add_bit_sequence(dictionary.get(byte).unwrap());
        }
    }

    if let Err(_) = if end_of_file {
        bitmap.flush_to_file(output_file)
    } else {
        bitmap.flush_filled_to_file(output_file)
    } {
        return Err(Error::new(
            ErrorKind::BrokenPipe,
            "Error while writing to file",
        ));
    };

    Ok(())
}

pub fn decode_file(file_info: Option<FileInfo>) -> Result<(), Error> {
    let (encoded_file, input_path) = match file_info {
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

    let mut dictionary: HashMap<Vec<u8>, u8> = HashMap::new();

    // actual algorithm of decoding starts here
    let (header_offset, hamming_code_len) = read_dictionary_header(&encoded_file, &mut dictionary);

    let mut file_reader = FileReader::new(encoded_file);
    let trie = build_codes_trie(&dictionary);
    file_reader.set_offset(header_offset);

    match write_decoded_file(
        &mut file_reader,
        &mut decoded_file,
        &trie,
        &dictionary,
        hamming_code_len,
    ) {
        Ok(_) => {
            println!("Decompressing completed successfully!");
            Ok(())
        }
        Err(err) => Err(err),
    }
}

fn read_dictionary_header(file: &File, dict: &mut HashMap<Vec<u8>, u8>) -> (usize, Option<usize>) {
    let mut buf = [0_u8; 2048]; // 2Kb buffer
    let mut offset = 0;
    file.read_at(&mut buf, 0).unwrap();

    let mut bitmap = BitMap::new();

    let hamming_code_len = if &buf[..HAMMING_CODE_LENGTH_KEY.len()] == HAMMING_CODE_LENGTH_KEY {
        offset += HAMMING_CODE_LENGTH_KEY.len() + 1;
        Some(buf[HAMMING_CODE_LENGTH_KEY.len()] as usize)
    } else {
        None
    };

    // reading dictionary from header of the file
    while &buf[offset..=offset + 1] != DICTIONARY_END {
        // get original symbol code
        let symbol = buf[offset];
        offset += 1;

        // get bit sequence length
        let code_length = buf[offset];
        offset += 1;

        // get bit sequence
        let bytes_taken_by_code = code_length.div_ceil(8) as usize;
        for _ in 0..bytes_taken_by_code {
            bitmap.add_byte(buf[offset]);
            offset += 1;
        }

        dict.insert(bitmap.get_bits(code_length as usize), symbol);
        bitmap.clear();
    }

    offset += 2;
    (offset, hamming_code_len)
}

fn build_codes_trie(dict: &HashMap<Vec<u8>, u8>) -> Trie<u8> {
    let mut builder = trie_rs::TrieBuilder::new();
    for code in dict.keys() {
        builder.push(code);
    }

    builder.build()
}

fn write_decoded_file(
    file_reader: &mut FileReader,
    decoded_file: &mut File,
    trie: &Trie<u8>,
    dictionary: &HashMap<Vec<u8>, u8>,
    hamming_code_length: Option<usize>,
) -> Result<(), Error> {
    let write_error = "Could not parse directory into file";

    let mut bitmap = BitMap::new();

    let chunk_length = 1024;
    let mut chunk_start = 0;

    let mut decoded_bit_stream = Vec::new();
    let mut connecting_bits = Vec::new();

    file_reader.read_file_in_chunks(|buf, _| {
        let buf_len = buf.len();
        while chunk_start < buf_len {
            let bytes_chunk = &buf[chunk_start..min(chunk_start + chunk_length, buf_len)];
            bitmap.add_bytes(bytes_chunk);

            // if hamming codes are used
            if let Some(msg_len) = hamming_code_length {
                let mut bits = Vec::from(&connecting_bits[..]);
                bits.append(&mut bitmap.get_all_bits());

                let connection_pos = bits.len() - bits.len() % msg_len;

                for b in &bits[connection_pos..] {
                    connecting_bits.push(b.clone());
                }

                bitmap.clear();
                bitmap.add_bit_sequence(&mut hamming::remove_parity_package(
                    &mut bits[..connection_pos],
                    msg_len,
                ));
            }

            decoded_bit_stream.append(&mut bitmap.get_all_bits());
            bitmap.clear();

            let mut decoded_bytes = Vec::new();
            let length = decoded_bit_stream.len();

            let mut ptr_lo = 0;
            let mut ptr_hi = 1;
            while ptr_hi < length {
                while !trie.exact_match(&decoded_bit_stream[ptr_lo..ptr_hi]) && ptr_hi < length {
                    ptr_hi += 1;
                }
                if ptr_hi >= length {
                    break;
                }

                decoded_bytes.push(*dictionary.get(&decoded_bit_stream[ptr_lo..ptr_hi]).unwrap());

                ptr_lo = ptr_hi;
                ptr_hi += 1;
            }

            if let Err(_) = decoded_file.write(decoded_bytes.as_slice()) {
                return Err(Error::new(ErrorKind::Other, write_error));
            };

            decoded_bit_stream = Vec::from(&decoded_bit_stream[ptr_lo..]);
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
