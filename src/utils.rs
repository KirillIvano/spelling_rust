use std::collections::HashMap;

use crate::types::FreqDict;

fn find_freq_median(dict: &HashMap<String, u32>) -> u32 {
    let mut frequences = dict.values().cloned().collect::<Vec<u32>>();
    frequences.sort();

    let mid = frequences.len() >> 1;

    frequences[mid]
}

#[test]
fn test_freq_median() {
    let result = find_freq_median(&HashMap::from([
        (String::from("a"), 3),
        (String::from("b"), 10),
        (String::from("c"), 8),
        (String::from("d"), 9),
        (String::from("e"), 10),
    ]));

    assert_eq!(result, 9);
}

const SEPARATORS: [char; 11] = ['.', ':', ';', ' ', '\n', ',', '!', '!', '…', '-', '?'];

pub fn get_words_from_text(text: String) -> Vec<String> {
    return text
        .split(SEPARATORS)
        .filter(|word| !word.is_empty())
        .map(|word| word.to_lowercase())
        .collect();
}

/// отфильтровывает лишние значения из частотного словаря
pub fn filter_by_median(dict: &HashMap<String, u32>) -> HashMap<String, u32> {
    let median = find_freq_median(&dict);

    let mut res: HashMap<String, u32> = HashMap::new();

    for (key, value) in dict.iter() {
        if *value >= median {
            res.insert(key.to_string(), *value);
        }
    }

    res
}
