use std::collections::HashMap;
use std::io::Error;

use crate::freq_check::freq_check;
use crate::levenstein::levenstein;
use crate::prepare_dict::get_dicts;

use crate::types::{FreqDict, RelationDict};
use crate::{author_check::author_check, utils::get_words_from_text};

/// исправляет ошибку в слове
pub fn speller(
    freq_dict: &FreqDict,
    relations_dict: &RelationDict,
    words: &Vec<String>,
    word: String,
    index: usize,
) -> Result<(String, String), std::io::Error> {
    println!("potential spelling error: {word}");

    // считаем первичных кандидатов по Левенштейну
    let initial_candidates = levenstein(&freq_dict, &word);
    println!("initial candidates: {:?}", initial_candidates);

    // возвращаем ответ если кандидат только один
    if initial_candidates.len() == 1 {
        println!(
            "result found on levenstein check: {}",
            initial_candidates[0]
        );
        return Ok((word.to_string(), initial_candidates[0].to_string()));
    }

    // ищем соседей слова в предложении
    let left_neigh = index.checked_sub(1).and_then(|ind| words.get(ind));
    let right_neigh = index.checked_add(1).and_then(|ind| words.get(ind));

    let neighbours = [left_neigh, right_neigh]
        .iter()
        .filter(|x| x.is_some())
        // we can unwrap because none's are filtered
        .map(|x| x.unwrap().to_string())
        .collect();

    // ищем лучшее исправление из контекста
    let authored_candidates = author_check(relations_dict, neighbours, initial_candidates);
    println!("authored candidates: {:?}", authored_candidates);

    // возвращаем ответ если кандидат только один
    if authored_candidates.len() == 1 {
        println!("result found on author check: {}", authored_candidates[0]);
        return Ok((word.to_string(), authored_candidates[0].to_string()));
    }

    // берем наиболее частое слово как итоговый результат
    return Ok((
        word.to_string(),
        freq_check(&freq_dict, &authored_candidates).unwrap(),
    ));
}

pub fn arrange_spelling(text: &str, author: &str) -> Result<HashMap<String, String>, Error> {
    let mut words = get_words_from_text(text.to_string());
    let (freq_dict, relations_dict) = get_dicts(author)?;

    let word_index = words.iter().position(|x| !freq_dict.contains_key(x));

    if word_index.is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "all words are correct",
        ));
    }

    let mut prev_correct = 0;
    while prev_correct < words.len() && freq_dict.get(&words[prev_correct]).is_none() {
        prev_correct = prev_correct + 1;
    }

    let mut response: HashMap<String, String> = HashMap::new();

    // first fix spelling in first incorrect words
    for i in (0..prev_correct).rev() {
        println!("{}", i);

        let cur = speller(&freq_dict, &relations_dict, &words, words[i].to_string(), i)?;

        // обновляем слово в предложении
        words[i] = cur.1.to_string();
        response.insert(cur.0, cur.1);
    }

    if prev_correct != words.len() {
        for i in (prev_correct + 1)..words.len() {
            let cur = speller(&freq_dict, &relations_dict, &words, words[i].to_string(), i)?;

            // обновляем слово в предложении
            words[i] = cur.1.to_string();
            response.insert(cur.0, cur.1);    
        }    
    }

    Ok(response)
}
