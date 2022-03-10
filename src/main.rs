mod types;

mod author_check;
mod freq_check;
mod levenstein;
mod prepare_dict;
mod speller;
mod utils;

fn main() {
    let result = speller::arrange_spelling("праподает судьбя человеческая", "dostoevski")
        .expect("Не удалось расчитать ошибку написания или в тексте нет ошибок");

    println!("Исправления: {:?}", serde_json::to_string_pretty(&result));
}
