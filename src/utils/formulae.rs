pub fn parse_chunk_for_unique_bytes(dictionary: &mut [u64; 256], buffer: &[u8], counter: &mut u64) {
    for byte in buffer.iter().copied() {
        if byte < 32 {
            continue;
        }
        dictionary[byte as usize] += 1;
        *counter += 1;
    }
}

pub fn calculate_information_amount(dictionary: &[u64; 256], size: u64) -> f64 {
    let mut sum = 0.0;

    for i in 0..dictionary.len() {
        sum += sum_term(dictionary[i], size);
    }

    -(size as f64 * sum)
}

pub fn calculate_entropy(info_amount: f64, size: u64) -> f64 {
    info_amount / size as f64
}

pub fn calculate_max_entropy(unique_char_count: u64) -> f64 {
    (unique_char_count as f64).log2()
}

pub fn calculate_redundancy(entropy: f64, bytes_per_character: u8) -> f64 {
    (8.0 * bytes_per_character as f64) - entropy
}

fn sum_term(char_count: u64, size: u64) -> f64 {
    let probability = char_count as f64 / size as f64;

    probability * probability.log2()
}
