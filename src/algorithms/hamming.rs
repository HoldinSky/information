use std::{cmp::min, io::Write};

pub fn add_parity(data: &[u8]) -> Vec<u8> {
    let mut message = data_to_message(data);
    let msg_len = message.len();

    // assigning parity bits
    let mut pbit = 0;
    let mut i = 1;
    while i <= msg_len {
        let pbit_pos = msg_len - i; // position of parity bit in array
        let mut ones_count = 0;

        for j in (0..pbit_pos).rev() {
            // next line checks if parity bit is responsible for j's element in array
            if (msg_len - j) & (1 << pbit) != 0 && message[j] == 1 {
                ones_count += 1;
            }
        }

        message[pbit_pos] = if ones_count & 1 == 0 { 0 } else { 1 };

        pbit += 1;
        i *= 2;
    }

    message
}

pub fn remove_parity(message: &mut [u8]) -> Vec<u8> {
    let syndrom = check_for_erros(message);

    correct_error(message, &syndrom);
    message_to_data(&message)
}

/// assumes that package length is exactly divisible by data_len
pub fn add_parity_package(package: &[u8], data_len: usize) -> Vec<u8> {
    let mut encoded = vec![];
    let pckg_len = package.len();

    for data_ptr in (0..pckg_len).step_by(data_len) {
        let data_to_encode = &package[data_ptr..min(data_ptr + data_len, pckg_len)];

        if data_to_encode.len() < data_len {
            let mut data_to_encode = Vec::from(data_to_encode);
            data_to_encode
                .write_all(&vec![0; data_len - data_to_encode.len()])
                .unwrap();

            encoded.append(&mut add_parity(&data_to_encode));
            continue;
        }

        encoded.append(&mut add_parity(data_to_encode));
    }

    encoded
}

pub fn remove_parity_package(package: &mut [u8], message_len: usize) -> Vec<u8> {
    let mut decoded = vec![];
    let pckg_len = package.len();

    for msg_ptr in (0..pckg_len).step_by(message_len) {
        let msg_to_decode = &mut package[msg_ptr..min(msg_ptr + message_len, pckg_len)];

        decoded.append(&mut remove_parity(msg_to_decode));
    }

    decoded
}

fn data_to_message(data: &[u8]) -> Vec<u8> {
    let data_len = data.len();
    let message_len = message_length(data_len);

    // composing message with data and parity bits
    let mut message = vec![u8::MAX; message_len];

    let mut data_ptr = 0;
    for i in 0..message_len {
        if (message_len - i - 1) & (message_len - i) != 0 {
            message[i] = data[data_ptr];
            data_ptr += 1;
        }
    }

    message
}

fn message_to_data(message: &[u8]) -> Vec<u8> {
    let message_len = message.len();
    let data_len = data_length(message_len);

    let mut data = vec![u8::MAX; data_len];
    let mut data_ptr = 0;

    // defining bits that should be omitted in resulting data
    let mut pbit = message_len - data_len - 1;
    let mut pbit_pos = message_len - 2_u32.pow(pbit as u32) as usize;

    for i in 0..message_len {
        if i == pbit_pos {
            pbit -= 1;
            // on pre-last parity bit we want to skip last parity bit
            if pbit == 0 {
                break;
            }
            pbit_pos = message_len - 2_u32.pow(pbit as u32) as usize;
            continue;
        }

        data[data_ptr] = message[i];
        data_ptr += 1;
    }

    data
}

fn check_for_erros(message: &[u8]) -> Vec<u8> {
    let mut syndrom: Vec<u8> = Vec::new();
    let message_len = message.len();

    let mut pbit = 0;
    let mut i = 1;
    while i <= message_len {
        let mut ones_count = 0;
        let pbit_pos = message_len - i; // position of parity bit in array

        for j in (0..pbit_pos).rev() {
            // next line checks if parity bit is responsible for j's element in array
            if (message_len - j) & (1 << pbit) != 0 {
                if message[j] == 1 {
                    ones_count += 1;
                }
            }
        }

        if ones_count & 1 == message[pbit_pos] {
            syndrom.push(0);
        } else {
            syndrom.push(1);
        }

        pbit += 1;
        i *= 2;
    }

    syndrom
}

fn correct_error(message: &mut [u8], syndrom: &Vec<u8>) {
    if !syndrom.contains(&1) {
        return;
    }

    let mut power_of_two = 1;
    let mut wrong_bit = 0;

    // converting binary form to decimal
    for i in 0..syndrom.len() {
        if syndrom[i] == 1 {
            wrong_bit += power_of_two;
        }
        power_of_two *= 2;
    }

    // reversing index
    let wrong_bit = message.len() - wrong_bit;

    message[wrong_bit] = (1 + message[wrong_bit]) % 2;
}

pub fn message_length(data_len: usize) -> usize {
    let mut redundant_count = 0;
    while 1 << redundant_count < data_len + redundant_count + 1 {
        redundant_count += 1;
    }

    redundant_count + data_len
}

pub fn data_length(message_len: usize) -> usize {
    let mut redundant_count = 0;
    while 1 << redundant_count < message_len + 1 {
        redundant_count += 1;
    }

    message_len - redundant_count
}
