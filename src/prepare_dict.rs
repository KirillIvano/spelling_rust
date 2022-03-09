use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::utils::get_words_from_text;
use crate::{
    types::{FreqDict, RelationDict},
    utils::filter_by_median,
};

fn read_paths() -> Result<Vec<PathBuf>, std::io::Error> {
    return fs::read_dir("resources").and_then(|content| {
        return content.map(|x| x.map(|res| res.path())).collect();
    });
}

fn create_relations_dict(
    words: &Vec<String>,
    freq_dict: &FreqDict,
) -> Result<RelationDict, std::io::Error> {
    let filtering_dict = filter_by_median(&freq_dict);

    let len = words.len();

    let mut result_dict: HashMap<String, HashMap<String, bool>> = HashMap::new();
    for i in 0..len {
        if filtering_dict.contains_key(&words[i]) {
            if !result_dict.contains_key(&words[i]) {
                result_dict.insert(words[i].to_string(), HashMap::new());
            }

            let current_dict = result_dict.get_mut(&words[i].to_string()).unwrap();

            if i > 0 {
                current_dict.insert(words[i - 1].to_string(), true);
            }
            if i < len - 1 {
                current_dict.insert(words[i + 1].to_string(), true);
            };
        }
    }

    Ok(result_dict)
}

fn create_freq_dict(words: &Vec<String>) -> Result<FreqDict, std::io::Error> {
    let mut res_dict: FreqDict = HashMap::new();

    for word in words {
        res_dict.insert(word.to_string(), res_dict.get(word).unwrap_or(&0) + 1);
    }

    return Ok(res_dict);
}

fn get_path_by_author<'a>(author_name: &str, paths: &'a Vec<PathBuf>) -> &'a PathBuf {
    let path = paths
        .iter()
        .find(
            |item| match item.file_stem().and_then(|name| name.to_str()) {
                None => false,
                Some(x) => x == author_name,
            },
        )
        .expect("There is no such a name");

    path
}

pub fn get_dict_from_cache(author_name: &str) -> Result<(FreqDict, RelationDict), std::io::Error> {
    let cache_name = format!("cache/{}", author_name);
    let file = fs::read_to_string(cache_name);

    return match file {
        Ok(x) => Ok(serde_json::from_str::<(FreqDict, RelationDict)>(&x)?),
        Err(x) => Err(x)
    }
}

fn calculate_dict(author_name: &str) -> Result<(FreqDict, RelationDict), std::io::Error> {
    let paths = read_paths()?;
    let path = get_path_by_author(author_name, &paths);

    let file_content = fs::read_to_string(path)?;
    let words = get_words_from_text(file_content);

    let freq_dict = create_freq_dict(&words)?;
    let relation_dict = create_relations_dict(&words, &freq_dict)?;

    Ok((freq_dict, relation_dict))
}

/// создает частотный и относительный словари
pub fn get_dicts(author_name: &str) -> Result<(FreqDict, RelationDict), std::io::Error> {
    get_dict_from_cache(author_name).or_else(|_| calculate_dict(author_name))
}

#[test]
fn test_prepare_dict() {
    let result = get_dicts("dostoevski").unwrap();
    let json_res = serde_json::to_string_pretty(&result).unwrap();

    fs::write("cache/dostoevski.json", json_res);
}
