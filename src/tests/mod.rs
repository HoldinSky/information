#![cfg(test)]
#![allow(unused)]

use crate::{
    algorithms::huffman,
    bit_map::BitMap,
    types::CodeType,
    utils::{clear, logic::encode_file, pause, print_entries_of_current_dir, terminal},
};
use std::fs::File;

#[test]
fn test_bit_container() {
    let mut cont = BitMap::new();

    cont.add_bit(true);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(true);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(false);

    assert_eq!(cont.get_bytes(1), [59]);

    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);

    assert_eq!(cont.get_bytes(4), [59, 170]);

    cont.add_bit(true);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(true);
    cont.add_bit(true);

    assert_eq!(cont.get_bytes(4), [59, 170, 235]);

    cont.add_bit(true);
    cont.add_bit(true);

    assert_eq!(cont.get_bytes(4), [59, 170, 235]);
}

#[test]
fn test_flush() {
    let mut cont = BitMap::new();

    cont.add_bit(true);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(true);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(false);

    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);

    cont.add_bit(true);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(true);
    cont.add_bit(true);

    cont.add_bit(true);
    cont.add_bit(true);

    let mut file = File::create("/home/nazar/prg/rust/labs/tik/samples/test_bits").unwrap();
    cont.flush_to_file(&mut file).unwrap();
}

#[test]
fn test_shannon_fano_encoding() {
    let path = String::from("/home/nazar/prg/rust/labs/tik/samples/en/test_1.txt");
    let file = File::open(&path).unwrap();

    encode_file(Some((file, path)), CodeType::ShannonFano);
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
        print!("{byte} - ");
        for bit in code {
            print!("{}", if bit { 1 } else { 0 });
        }
        println!();
    }
}
