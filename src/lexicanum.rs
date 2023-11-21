use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use leptos::{server, ServerFnError};
use regex::Regex;
use rand::{seq::{SliceRandom, IteratorRandom}, thread_rng};

fn load_words_from(file_path: String) -> Result<Vec<String>, ServerFnError> {
    let f = File::open(file_path)?;
    let reader = BufReader::new(f);
    Ok(reader.lines()
        .flat_map( |maybe_l| maybe_l.ok())
        .collect())
}

fn sanitize_filter(chars: String) -> String {
    chars.chars().filter( |c| c.is_alphabetic()).collect()
}

#[server]
pub async fn get_word_pool(allowed_chars: Option<String>, num_words: usize) -> Result<Vec<String>, ServerFnError> {
    // let existing_words = vec! [ "pata", "batata", "pena", "Pedro", "Papá", "Tia", "touro", "tempo"];
    let existing_words = load_words_from("wordlist/wordlist-ao-latest.txt".to_string());

    Ok(match allowed_chars {
        None => existing_words?
                    .choose_multiple(&mut thread_rng(), num_words)
                    .map(|s| s.to_owned())
                    .collect(),
        Some(chars) => {
            let allowed_regex = Regex::new(format!("^[{}]+$", sanitize_filter(chars)).as_str()).unwrap();
            existing_words
                .into_iter()
                .flatten()
                .filter(|word| allowed_regex.is_match(word))
                .choose_multiple(&mut thread_rng(), num_words)
        },
    })
}