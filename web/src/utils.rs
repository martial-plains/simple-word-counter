use std::time::Duration;

use regex::Regex;

pub fn word_count(text: &str) -> usize {
    let pattern = Regex::new(r"\w+").unwrap();
    let matches = pattern.find_iter(text);
    let words: Vec<&str> = matches.map(|m| m.as_str()).collect();

    words.len()
}

pub fn sentence_count(text: &str) -> usize {
    let pattern = Regex::new(r"(?i)[^.!?]+[.!?]").unwrap();
    let matches = pattern.find_iter(text);
    let sentences: Vec<&str> = matches.map(|m| m.as_str()).collect();

    sentences.len()
}

pub fn paragraph_count(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }

    let pattern = Regex::new(r"\n\s*\n").unwrap();
    let paragraphs: Vec<&str> = pattern.split(text).collect();

    paragraphs.len()
}

pub fn calculate_duration(word_count: usize, words_per_minute: u32) -> Duration {
    let minutes = f64::from(word_count as u32) / f64::from(words_per_minute);
    let seconds = minutes * 60.0;

    Duration::from_secs(seconds as u64)
}
