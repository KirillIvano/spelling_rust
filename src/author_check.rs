use crate::types::RelationDict;

pub fn author_check(
    relations_dict: RelationDict,
    neighbours: Vec<String>,
    candidats: Vec<String>,
) -> Vec<String> {
    let mut best_candidates: Vec<String> = Vec::new();
    let mut max_size = 0;

    for candidat in candidats {
        let candidat_relations = relations_dict.get(&candidat).unwrap();
        let intersection_size = neighbours
            .iter()
            .filter(|neigh| candidat_relations.contains_key(*neigh))
            .count();

        if intersection_size > max_size {
            max_size = intersection_size;
            best_candidates.clear();
            best_candidates.push(candidat);
        } else if intersection_size == max_size {
            best_candidates.push(candidat);
        }
    }

    best_candidates
}

#[test]
fn test_author_check() {
    use std::fs;
    use crate::types::FreqDict;

    let cache = fs::read_to_string("cache/tolstoy.json").unwrap();
    let (_, relation_dict) = serde_json::from_str::<(FreqDict, RelationDict)>(&cache).unwrap();

    let neighbours = Vec::from(["the".to_string()]);

    let candidats = Vec::from(["military".to_string(), "in".to_string()]);

    author_check(relation_dict, neighbours, candidats);
}
