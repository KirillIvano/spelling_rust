use crate::freq_check::freq_check;
use crate::levenstein::levenstein;
use crate::prepare_dict::get_dicts;

use crate::{author_check::author_check, utils::get_words_from_text};

pub fn speller(text: &str, author: &str) -> Result<(String, String), std::io::Error> {
    let words = get_words_from_text(text.to_string());

    let (freq_dict, relations_dict) = get_dicts(author)?;

    let word_index = words.iter().position(|x| !freq_dict.contains_key(x));

    if word_index.is_none() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "all words are correct"));
    }

    // unwrapping because of checks above
    let safe_index = word_index.unwrap();
    let word = words.get(safe_index).unwrap();

    println!("potential spelling error: {word}");

    let initial_candidates = levenstein(&freq_dict, &word);
    println!("initial candidates: {:?}", initial_candidates);

    if initial_candidates.len() == 1 {
        print!("result found on levenstein check: {}", initial_candidates[0]);
        return Ok((word.to_string(), initial_candidates[0].to_string()));
    }

    let left_neigh = safe_index.checked_sub(1).and_then(|ind| words.get(ind));
    let right_neigh = safe_index.checked_add(1).and_then(|ind| words.get(ind));

    let neighbours = [left_neigh, right_neigh]
        .iter()
        .filter(|x| x.is_some())
        // we can unwrap because none's are filtered
        .map(|x| x.unwrap().to_string())
        .collect();

    let authored_candidates = author_check(relations_dict, neighbours, initial_candidates);
    println!("authored candidates: {:?}", authored_candidates);

    if authored_candidates.len() == 1 {
        print!("result found on author check: {}", authored_candidates[0]);
        return Ok((word.to_string(), authored_candidates[0].to_string()));
    }

    print!(
        "result found on frequence check: {}",
        freq_check(&freq_dict, &authored_candidates).unwrap()
    );
    return Ok((word.to_string(), freq_check(&freq_dict, &authored_candidates).unwrap()));
}
