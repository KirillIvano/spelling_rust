mod types;

mod author_check;
mod freq_check;
mod levenstein;
mod prepare_dict;
mod speller;
mod utils;

fn main() {
    speller::speller("чисто поли", "dostoevski")
        .expect("Не удалось расчитать ошибку написания или в тексте нет ошибок");
}
