#![cfg(test)]
#![allow(unused)]

use crate::{
    algorithms::{hamming, huffman},
    bit_map::BitMap,
    types::{CodeType, EncodingSettings},
    utils::{clear, logic::encode_file, pause, print_entries_of_current_dir, terminal},
};
use std::{cmp::min, fs::File};

#[test]
fn test_bit_container() {
    let mut cont = BitMap::new();

    cont.add_bit(1);
    cont.add_bit(1);
    cont.add_bit(0);
    cont.add_bit(1);
    cont.add_bit(1);
    cont.add_bit(1);
    cont.add_bit(0);
    cont.add_bit(0);

    assert_eq!(cont.get_bytes(1), [59]);

    cont.add_bit(0);
    cont.add_bit(1);
    cont.add_bit(0);
    cont.add_bit(1);
    cont.add_bit(0);
    cont.add_bit(1);
    cont.add_bit(0);
    cont.add_bit(1);

    assert_eq!(cont.get_bytes(4), [59, 170]);

    cont.add_bit(1);
    cont.add_bit(1);
    cont.add_bit(0);
    cont.add_bit(1);
    cont.add_bit(0);
    cont.add_bit(1);
    cont.add_bit(1);
    cont.add_bit(1);

    assert_eq!(cont.get_bytes(4), [59, 170, 235]);

    cont.add_bit(1);
    cont.add_bit(1);

    assert_eq!(cont.get_bytes(4), [59, 170, 235]);
}

#[test]
fn test_flush() {
    let mut cont = BitMap::new();

    cont.add_bit(1);
    cont.add_bit(1);
    cont.add_bit(0);
    cont.add_bit(1);
    cont.add_bit(1);
    cont.add_bit(1);
    cont.add_bit(0);
    cont.add_bit(0);

    cont.add_bit(0);
    cont.add_bit(1);
    cont.add_bit(0);
    cont.add_bit(1);
    cont.add_bit(0);
    cont.add_bit(1);
    cont.add_bit(0);
    cont.add_bit(1);

    cont.add_bit(1);
    cont.add_bit(1);
    cont.add_bit(0);
    cont.add_bit(1);
    cont.add_bit(0);
    cont.add_bit(1);
    cont.add_bit(1);
    cont.add_bit(1);

    cont.add_bit(1);
    cont.add_bit(1);

    let mut file = File::create("/home/nazar/prg/rust/labs/tik/samples/test_bits").unwrap();
    cont.flush_to_file(&mut file).unwrap();
}

#[test]
fn test_shannon_fano_encoding() {
    let path = String::from("/home/nazar/prg/rust/labs/tik/samples/en/test_1.txt");
    let file = File::open(&path).unwrap();

    let settings = EncodingSettings {
        file_info: (file, path),
        code_type: CodeType::ShannonFano,
        hamming_code_length: None,
    };

    encode_file(settings);
}

#[test]
fn test_directory_change() {
    clear();
    print_entries_of_current_dir();

    pause("Press any key...");
    clear();
    terminal::change_dir("..");
    print_entries_of_current_dir();

    pause("Press any key...");
    clear();
    terminal::change_dir("..");
    print_entries_of_current_dir();

    pause("Press any key...");
    clear();
    terminal::change_dir("/opfwe");
    print_entries_of_current_dir();
    pause("Press any key...");
}

#[test]
fn test_huffman_encoding() {
    let mut stats = [0_u64; 256];
    stats[0] = 5;
    stats[1] = 9;
    stats[2] = 12;
    stats[3] = 13;
    stats[4] = 16;
    stats[5] = 45;

    let mut codes = huffman::encode((stats, 100));

    for (byte, code) in codes {
        print!("{byte} - {code:?}");
        println!();
    }
}

#[test]
fn test_hamming_encoding() {
    let data = vec![1, 0, 1, 1, 0, 0, 1];
    let mut msg = hamming::add_parity(&data);
    assert_eq!(data, hamming::remove_parity(&mut msg));

    let data = vec![1, 1, 1, 0, 0, 1, 0, 0, 1, 0];
    msg = hamming::add_parity(&data);
    msg[2] = (1 + msg[2]) % 2; // making an error in message with parity bits
    assert_eq!(data, hamming::remove_parity(&mut msg));

    let data = vec![1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0];
    msg = hamming::add_parity(&data);
    msg[3] = (1 + msg[3]) % 2;
    assert_eq!(data, hamming::remove_parity(&mut msg));

    let data = vec![1, 1, 1, 0, 0, 1, 1, 0];
    msg = hamming::add_parity(&data);
    msg[5] = (1 + msg[5]) % 2;
    assert_eq!(data, hamming::remove_parity(&mut msg));

    let data = vec![1, 1];
    msg = hamming::add_parity(&data);
    assert_eq!(data, hamming::remove_parity(&mut msg));
}

#[test]
fn playground() {
    let total = 1000;
    let step = 210;
    for i in (0..total).step_by(step) {
        println!(
            "Stepping by {step} in {total}: {} - {}",
            i,
            min(i + step, total)
        )
    }
}
