use std::fs::File;
use std::io::{prelude::*, BufReader};
use leptos::{server, ServerFnError};
use regex::Regex;
use rand::{seq::{IteratorRandom}, thread_rng};

use crate::app::Difficulty;

#[server]
async fn load_words_from(file_path: String) -> Result<Vec<String>, ServerFnError> {
    let f = File::open(file_path)?;
    let reader = BufReader::new(f);
    Ok(reader.lines()
        .flat_map( |maybe_l| maybe_l.ok())
        .collect())
}

fn sanitize_filter(chars: &String) -> String {
    chars.chars().filter( |c| c.is_alphabetic()).collect()
}

#[server]
pub async fn get_word_pool(allowed_chars: Option<String>, num_words: usize, diff: Difficulty) -> Result<Vec<String>, ServerFnError> {
    let existing_words = load_words_from("wordlist/wordlist-ao-latest.txt".to_string()).await;

    Ok(match allowed_chars {
        None => existing_words
                    .into_iter()
                    .flatten()
                    .filter(|w| allowed_difficulty(w, &diff))
                    .choose_multiple(&mut thread_rng(), num_words),
        Some(chars) => {
            let allowed_regex = Regex::new(format!("^[{}]+$", sanitize_filter(&chars)).as_str()).unwrap();
            existing_words
                .into_iter()
                .flatten()
                .filter(|w| allowed_difficulty(w, &diff))
                .filter(|w| allowed_regex.is_match(w))
                .choose_multiple(&mut thread_rng(), num_words)
        },
    })
}

fn allowed_difficulty(w: &String, diff: &Difficulty) -> bool {
    match diff {
        Difficulty::Easiest => {
            w.len() < 6 && w.rfind("-").is_none()
        },
        Difficulty::Easy => {
            w.len() < 7 && w.len() > 2 && w.rfind("-").is_none()
        },
        Difficulty::Medium => {
            w.len() < 8 && w.len() > 4 && w.rfind("-").is_none()
        },
        Difficulty::Hard => {
            w.len() < 12 && w.len() > 5 && w.matches("-").collect::<Vec<&str>>().len() < 2
        },
        Difficulty::Hardest => {
            w.len() > 10
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filter() {
        let unsanitized = [ "Robert'); DROP TABLE Students;--".to_owned(), "aâæãée24ēçćcbbò".to_owned() ];

        assert_eq!(sanitize_filter(&unsanitized[0]), "RobertDROPTABLEStudents");
        assert_eq!(sanitize_filter(&unsanitized[1]), "aâæãéeēçćcbbò");
        
    }

    #[test]
    fn test_difficulty() {
        let easiest_words = [ "papa".to_owned(), "pote".to_owned() ];
        let easy_words = [ "Robert'); DROP TABLE Students;--".to_owned(), "aâæãée24ēçćcbbò".to_owned() ];
        let medium_words = [ "Robert'); DROP TABLE Students;--".to_owned(), "aâæãée24ēçćcbbò".to_owned() ];
        let hard_words = [ "Robert'); DROP TABLE Students;--".to_owned(), "aâæãée24ēçćcbbò".to_owned() ];
        let hardest_words = [ "anticonstitucionalissimamente".to_owned(), "aâæãée24ēçćcbbò".to_owned() ];

        let all_words = [easiest_words.clone(), easy_words.clone(), medium_words.clone(), hard_words.clone(), hardest_words.clone()].concat();

        let easiest_allowed: Vec<String> = all_words.into_iter().filter(|w| allowed_difficulty(w,&Difficulty::Easiest)).collect();
        assert_eq!(easiest_allowed, easiest_words);
    }
}