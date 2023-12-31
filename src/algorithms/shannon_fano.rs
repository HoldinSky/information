use std::collections::HashMap;

use crate::types::{FileStats, Probability};

use super::create_probability_map;

pub fn encode(stats: FileStats) -> HashMap<u8, Vec<u8>> {
    let probs = create_probability_map(&stats.0, stats.1);
    let probs = probs.as_slice();
    let mut codes_map = HashMap::new();

    // actually encoding
    enc(&probs, 0, probs.len() - 1, &mut codes_map);

    codes_map
}

fn enc(
    alphabet: &[Probability],
    set_start: usize,
    set_end: usize,
    codes_map: &mut HashMap<u8, Vec<u8>>,
) {
    if set_start.abs_diff(set_end) < 1 {
        return;
    }

    let split_index = partition(alphabet, set_start, set_end);

    let higher_set = &alphabet[set_start..=split_index];
    let lower_set = &alphabet[split_index + 1..=set_end];

    for (byte, _) in higher_set {
        let entry = codes_map.entry(*byte).or_insert(Vec::new());
        entry.push(0);
    }

    for (byte, _) in lower_set {
        let entry = codes_map.entry(*byte).or_insert(Vec::new());
        entry.push(1);
    }

    enc(&alphabet, set_start, split_index, codes_map);
    enc(&alphabet, split_index + 1, set_end, codes_map);
}

/// returns index of last element in higher set (marked as 0 in next stage) inclusively
fn partition(alphabet: &[Probability], low: usize, high: usize) -> usize {
    let mut low = low;
    let mut high = high;

    let mut low_probability = 0.0;
    let mut high_probability = 0.0;

    while low <= high {
        low_probability += alphabet[low].1;
        low += 1;

        while low_probability > high_probability {
            high_probability += alphabet[high].1;
            high -= 1;

            if low > high {
                break;
            }
        }
    }

    low - 1
}
