use std::collections::HashMap;

pub fn parse_chunk_for_unique_bytes(map: &mut HashMap<u8, i64>, buffer: &[u8], counter: &mut i64) {
    for byte in buffer.iter().copied() {
        if byte < 32 {
            continue;
        }
        let count = map.entry(byte).or_insert(0);
        *count += 1;
        *counter += 1;
    }
}

pub fn calculate_information_amount(map: &HashMap<u8, i64>, size: i64) -> f64 {
    let mut sum = 0.0;

    for (_, count) in map {
        sum += sum_term(*count, size);
    }

    -size as f64 * sum
}

pub fn calculate_entropy(info_amount: f64, size: i64) -> f64 {
    info_amount / size as f64
}

pub fn calculate_max_entropy(unique_char_count: i64) -> f64 {
    (unique_char_count as f64).log2()
}

pub fn calculate_redundancy(entropy: f64, bytes_per_character: u8) -> f64 {
    (8.0 * bytes_per_character as f64) - entropy
}


fn sum_term(char_count: i64, size: i64) -> f64 {
    let probability = char_count as f64 / size as f64;

    probability * probability.log2()
}