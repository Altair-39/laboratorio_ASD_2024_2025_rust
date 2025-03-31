use edit::edit_distance;
use edit::edit_distance_dyn;
use inquire::{Select, Text};
use memmap2::MmapOptions;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::str;
use std::time::Instant;

mod edit;

fn main() {
    let mode_options = vec!["Manual Input", "File Mode"];
    let mode_choice = Select::new("Choose the mode:", mode_options)
        .prompt()
        .unwrap();

    match mode_choice {
        "Manual Input" => manual_mode(),
        "File Mode" => file_mode(),
        _ => unreachable!(),
    }
}

fn manual_mode() {
    let options = vec!["edit_distance", "edit_distance_dyn"];
    let choice = Select::new("Choose the edit distance function:", options)
        .prompt()
        .unwrap();

    let word1 = Text::new("Enter the first word:").prompt().unwrap();
    let word2 = Text::new("Enter the second word:").prompt().unwrap();

    let start_time = Instant::now();
    let distance = match choice {
        "edit_distance" => edit_distance(&word1, &word2),
        "edit_distance_dyn" => edit_distance_dyn(&word1, &word2),
        _ => unreachable!(),
    };
    let duration = start_time.elapsed();

    println!(
        "The edit distance between '{}' and '{}' is: {} (calculated in {:.2?})",
        word1, word2, distance, duration
    );
}

fn build_len_index(dictionary: &[String]) -> HashMap<usize, Vec<&String>> {
    let mut len_index: HashMap<usize, Vec<&String>> = HashMap::new();
    for word in dictionary {
        len_index.entry(word.len()).or_default().push(word);
    }
    len_index
}

fn file_mode() {
    let total_start = Instant::now();

    println!("Loading dictionary...");
    let dict_load_start = Instant::now();
    let mut dictionary = match load_words("rsrc/dictionary.txt") {
        Ok(dict) => dict,
        Err(e) => {
            eprintln!("Error loading dictionary: {}", e);
            return;
        }
    };
    let dict_load_time = dict_load_start.elapsed();

    println!("Loading text to correct...");
    let correctme = match load_words("rsrc/correctme.txt") {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error loading correctme file: {}", e);
            return;
        }
    };

    println!("Sorting dictionary by length...");
    let sort_start = Instant::now();
    dictionary.sort_by_key(|a| a.len());
    let sort_time = sort_start.elapsed();

    println!("Building length index...");
    let index_start = Instant::now();
    let len_index = build_len_index(&dictionary);
    let index_time = index_start.elapsed();

    let options = vec!["edit_distance", "edit_distance_dyn"];
    let choice = Select::new("Choose the edit distance function:", options)
        .prompt()
        .unwrap();

    let re = Regex::new(r"\b[a-zA-Z]+\b").unwrap();
    let mut cache: HashMap<String, (Vec<String>, usize)> = HashMap::new();

    println!("Building cache...");
    let cache_start = Instant::now();
    for dict_word in &dictionary {
        let word_lower = dict_word.to_lowercase();
        cache.insert(word_lower, (vec![dict_word.clone()], 0));
    }
    let cache_time = cache_start.elapsed();

    println!("\nProcessing text...");
    let process_start = Instant::now();
    let mut total_words = 0;
    let mut cached_words = 0;
    let mut processed_words = 0;

    for sentence in &correctme {
        println!("\nProcessing sentence: {}", sentence);

        for word in re.find_iter(sentence) {
            total_words += 1;
            let word_lower = word.as_str().to_lowercase();
            let word_len = word_lower.len();

            if let Some(cached) = cache.get(&word_lower) {
                cached_words += 1;
                println!(
                    "  '{}' → Exact dictionary match: {:?}",
                    word_lower, cached.0
                );
                continue;
            }

            processed_words += 1;
            let word_start = Instant::now();
            let (closest_words, min_distance) =
                find_closest_words(&word_lower, word_len, &len_index, choice);

            cache.insert(word_lower.clone(), (closest_words.clone(), min_distance));
            println!(
                "  '{}' → Closest match(es): {:?} (distance: {}) in {:.2?}",
                word_lower,
                closest_words,
                min_distance,
                word_start.elapsed()
            );
        }
    }

    let process_time = process_start.elapsed();
    let total_time = total_start.elapsed();

    print_stats(
        dictionary.len(),
        total_words,
        cached_words,
        processed_words,
        dict_load_time,
        sort_time,
        index_time,
        cache_time,
        process_time,
        total_time,
    );
}

fn find_closest_words(
    word: &str,
    word_len: usize,
    len_index: &HashMap<usize, Vec<&String>>,
    choice: &str,
) -> (Vec<String>, usize) {
    let mut min_distance = usize::MAX;
    let mut closest_words = Vec::new();
    let max_dist = 3;
    let mut search_range = 1;

    fn get_distance_results<'a>(
        word: &'a str,
        dict_words: &'a [&'a String],
        choice: &'a str,
    ) -> Vec<(&'a String, usize)> {
        dict_words
            .par_iter()
            .filter_map(|dict_word| {
                let dict_word = *dict_word;
                if dict_word.len() < word.len().saturating_sub(1) {
                    return None; // Skip words that are too short
                }

                let distance = match choice {
                    "edit_distance" => edit_distance(word, dict_word),
                    "edit_distance_dyn" => edit_distance_dyn(word, dict_word),
                    _ => unreachable!(),
                };

                Some((dict_word, distance))
            })
            .collect()
    }

    loop {
        let mut found_match_in_range = false;

        for len in (word_len.saturating_sub(search_range))..=(word_len + search_range) {
            if let Some(words) = len_index.get(&len) {
                let results = get_distance_results(word, words, choice);

                for (dict_word, distance) in results {
                    match distance.cmp(&min_distance) {
                        std::cmp::Ordering::Less => {
                            min_distance = distance;
                            closest_words = vec![dict_word.to_string()];
                        }
                        std::cmp::Ordering::Equal => {
                            closest_words.push(dict_word.to_string());
                        }
                        _ => {}
                    }
                }

                if !closest_words.is_empty() {
                    found_match_in_range = true;
                }
            }
        }

        if found_match_in_range || search_range >= max_dist {
            break;
        }

        search_range += 1;
    }

    if search_range == 2 {
        if let Some(words) = len_index.get(&word_len) {
            let results = get_distance_results(word, words, choice);

            for (dict_word, distance) in results {
                if distance < min_distance {
                    min_distance = distance;
                    closest_words = vec![dict_word.to_string()]; // Found a better match, reset.
                } else if distance == min_distance {
                    closest_words.push(dict_word.to_string()); // Same distance, add the word.
                }
            }
        }
    }

    (closest_words, min_distance)
}

fn load_words(filename: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;

    let mmap = unsafe { MmapOptions::new().map(&file)? };

    let content =
        str::from_utf8(&mmap).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let words: Vec<String> = content.split_whitespace().map(|s| s.to_string()).collect();

    Ok(words)
}

fn print_stats(
    dict_size: usize,
    total_words: usize,
    cached_words: usize,
    processed_words: usize,
    dict_load_time: std::time::Duration,
    sort_time: std::time::Duration,
    index_time: std::time::Duration,
    cache_time: std::time::Duration,
    process_time: std::time::Duration,
    total_time: std::time::Duration,
) {
    println!("\n=== Performance Summary ===");
    println!("Dictionary:");
    println!("  - Words: {}", dict_size);
    println!("  - Load time: {:.2?}", dict_load_time);
    println!("  - Sort time: {:.2?}", sort_time);
    println!("  - Index build time: {:.2?}", index_time);
    println!("  - Cache build time: {:.2?}", cache_time);
    println!("\nText Processing:");
    println!("  - Total words: {}", total_words);
    println!(
        "  - Cached words: {} ({:.1}%)",
        cached_words,
        (cached_words as f32 / total_words as f32) * 100.0
    );
    println!("  - Processed words: {}", processed_words);
    println!("  - Processing time: {:.2?}", process_time);
    println!(
        "  - Average time per word: {:.2?}",
        process_time / std::cmp::max(processed_words, 1) as u32
    );
    println!("\nTotal execution time: {:.2?}", total_time);
}
