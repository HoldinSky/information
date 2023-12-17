pub fn add_parity(data: &Vec<u8>) -> Vec<u8> {
    let mut message = data_to_message(&data);
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

pub fn remove_parity(message: &mut Vec<u8>) -> Vec<u8> {
    let syndrom = check_for_erros(message);

    correct_error(message, &syndrom);
    message_to_data(&message)
}

fn data_to_message(data: &Vec<u8>) -> Vec<u8> {
    let data_len = data.len();
    let parity_bit_count = parity_bits_for_data(data_len);

    // composing message with data and parity bits
    let mut message = vec![u8::MAX; data_len + parity_bit_count];
    let message_len = message.len();

    let mut data_ptr = 0;
    for i in 0..message_len {
        if (message_len - i - 1) & (message_len - i) != 0 {
            message[i] = data[data_ptr];
            data_ptr += 1;
        }
    }

    message
}

fn message_to_data(message: &Vec<u8>) -> Vec<u8> {
    let message_len = message.len();
    let parity_bit_count = parity_bits_in_message(message_len);

    let mut data = vec![u8::MAX; message_len - parity_bit_count];
    let mut data_ptr = 0;

    // defining bits that should be omitted in resulting data
    let mut pbit = parity_bit_count - 1;
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

fn check_for_erros(message: &Vec<u8>) -> Vec<u8> {
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

fn correct_error(message: &mut Vec<u8>, syndrom: &Vec<u8>) {
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

fn parity_bits_for_data(data_len: usize) -> usize {
    let mut redundant_count = 0;
    while 1 << redundant_count < data_len + redundant_count + 1 {
        redundant_count += 1;
    }

    redundant_count
}

pub fn parity_bits_in_message(message_len: usize) -> usize {
    let mut redundant_count = 0;
    while 1 << redundant_count < message_len + 1 {
        redundant_count += 1;
    }

    redundant_count
}
