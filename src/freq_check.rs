use crate::types::FreqDict;

pub fn freq_check(freq_dict: &FreqDict, words: &Vec<String>) -> Option<String> {
    let word = words
        .iter()
        .max_by(|x, y| freq_dict[*x].cmp(&freq_dict[*y]))?
        .to_string();

    Some(word)
}

#[test]
fn test_freq_check() {
    use std::collections::HashMap;

    let mapping = HashMap::from([("ek".to_string(), 1), ("hek".to_string(), 2)]);

    println!(
        "result: {} !!!",
        freq_check(&mapping, &Vec::from(["hek".to_string(), "ek".to_string()])).unwrap()
    );
}
