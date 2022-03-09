use std::{collections::HashMap};

extern crate edit_distance;

pub fn levenstein<'a>(dict: &HashMap<String, u32>, word: &str) -> Vec<String> {
    let dict_iter = dict.keys();
    let mut min_distance = std::u32::MAX;
    let mut words: Vec<String>= Vec::new();

    for key in dict_iter {
        let diff = u32::try_from(edit_distance::edit_distance(word, key)).unwrap();

        if diff < min_distance {
            min_distance = std::cmp::min(diff, min_distance);

            words.clear();
            words.push(key.to_owned());
        } else if diff == min_distance {
            words.push(key.to_owned());
        }
    }

    words
}

#[test]
fn test_levenstein() {
    let mapping = HashMap::from([
        ("ek".to_string(), 0),
        ("hek".to_string(), 1)
    ]);

    let mut result = levenstein(&mapping, "kek");

    assert_eq!(result.sort(), Vec::from(["hek", "ek"]).sort());
}