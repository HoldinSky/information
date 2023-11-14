use std::collections::HashMap;

type Probability = (u8, f64);

pub fn encode(alphabet: &[u64; 256], total_count: u64) -> HashMap<u8, String> {
    let probs = create_probability_map(&alphabet, total_count);
    let probs = probs.as_slice();
    let mut codes_map = HashMap::new();

    // actualy encoding
    enc(&probs, 0, probs.len() - 1, &mut codes_map);

    codes_map
}

fn enc(
    alphabet: &[Probability],
    set_start: usize,
    set_end: usize,
    codes_map: &mut HashMap<u8, String>,
) {
    if set_start.abs_diff(set_end) < 1 {
        return;
    }

    let split_index = partition(alphabet, set_start, set_end);

    let higher_set = &alphabet[set_start..=split_index];
    let lower_set = &alphabet[split_index + 1..=set_end];

    for (byte, _) in higher_set {
        let entry = codes_map.entry(*byte).or_insert(String::new());
        entry.push('0');
    }

    for (byte, _) in lower_set {
        let entry = codes_map.entry(*byte).or_insert(String::new());
        entry.push('1');
    }

    enc(&alphabet, set_start, split_index, codes_map);
    enc(&alphabet, split_index + 1, set_end, codes_map);
}

fn create_probability_map(&alphabet: &[u64; 256], total_count: u64) -> Vec<Probability> {
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

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_partition() {
        let bytes = [
            (1, 0.2),
            (2, 0.18),
            (3, 0.16),
            (4, 0.13),
            (5, 0.11),
            (6, 0.09),
            (7, 0.08),
            (8, 0.05),
        ];

        let mut overall_probability = 0.0_f64;
        for (_, prob) in bytes {
            overall_probability += prob;
        }

        assert!((overall_probability - 1.0).abs() < 1e-10);
        println!("{}", partition(&bytes, 0, bytes.len() - 1));
    }

    #[test]
    fn test_encode() {
        let mut arr = [0; 256];
        let mut total = 0;
        for i in 0..20 {
            arr[i] = 20 - i as u64;
            total += arr[i];
        }

        encode(&arr, total);
    }
}
