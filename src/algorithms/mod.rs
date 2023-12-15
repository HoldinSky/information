use crate::types::{Probability, Quantity};

pub mod huffman;
pub mod shannon_fano;

fn create_probability_map(alphabet: &[u64; 256], total_count: u64) -> Vec<Probability> {
    let mut probabilities = vec![];
    for byte in 0..alphabet.len() {
        if alphabet[byte] == 0 {
            continue;
        }
        probabilities.push((byte as u8, alphabet[byte] as f64 / total_count as f64));
    }

    probabilities.sort_by(|a, b| b.1.total_cmp(&a.1));

    probabilities
}

fn create_quantity_map(alphabet: &[u64; 256]) -> Vec<Quantity> {
    let mut probs = Vec::new();

    for i in 0..alphabet.len() {
        probs.push(Quantity {
            byte: i as u8,
            quantity: alphabet[i] as u128,
        })
    }

    probs
}
