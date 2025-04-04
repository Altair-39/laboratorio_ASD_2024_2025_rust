use inquire::{Select, Text};
use memmap2::Mmap;
use std::fs::{self, File};
use std::hash::{Hash, Hasher};
use std::io::{self};
use std::path::PathBuf;
use std::time::Instant;

use crate::hashtable::*;

mod hashtable;

fn default_compare<K: Eq>(a: &K, b: &K) -> bool {
    a == b
}

fn default_hash<K: Hash>(key: &K) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

fn read_file_and_extract_words(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;

    let mmap = unsafe { Mmap::map(&file)? };

    let content = String::from_utf8_lossy(&mmap);

    let words: Vec<String> = content
        .split_whitespace()
        .map(|word| word.to_lowercase())
        .collect();

    Ok(words)
}

fn filter_words_by_length(words: Vec<String>, min_length: usize) -> Vec<String> {
    words
        .into_iter()
        .filter(|word| word.len() >= min_length)
        .collect()
}

fn get_txt_files_in_directory(dir: &str) -> io::Result<Vec<PathBuf>> {
    Ok(fs::read_dir(dir)?
        .filter_map(|entry| match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.extension()?.to_str()? == "txt" && path.is_file() {
                    Some(path)
                } else {
                    None
                }
            }
            Err(_) => None,
        })
        .collect::<Vec<PathBuf>>())
}

fn main() {
    let mut table = hash_table_create(default_compare, default_hash);

    loop {
        let action = Select::new(
            "Choose an action",
            vec![
                "Put key-value pair",
                "Get value by key",
                "Check if key exists",
                "Remove key",
                "Check size",
                "Show all keys",
                "Free hash table",
                "Test",
                "Exit",
            ],
        )
        .prompt()
        .unwrap();

        match action {
            "Put key-value pair" => {
                let key = Text::new("Enter key:").prompt().unwrap();
                let value: i32 = Text::new("Enter value (integer):")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();
                hash_table_put(&mut table, key.clone(), value);
                println!("Added key-value pair: {} = {}", key, value);
            }
            "Get value by key" => {
                let key = Text::new("Enter key:").prompt().unwrap();
                match hash_table_get(&mut table, key) {
                    Some(value) => println!("Value: {}", value),
                    None => println!("Key not found."),
                }
            }
            "Check if key exists" => {
                let key = Text::new("Enter key:").prompt().unwrap();
                let exists = hash_table_contains_key(&table, &key);
                if exists == 1 {
                    println!("Key exists.");
                } else {
                    println!("Key does not exist.");
                }
            }
            "Remove key" => {
                let key = Text::new("Enter key to remove:").prompt().unwrap();
                hash_table_remove(&mut table, key);
                println!("Removed key.");
            }
            "Check size" => {
                println!("The size of the hashtable is: {}.", hash_table_size(&table));
            }
            "Show all keys" => {
                let keys = hash_table_keyset(&table);
                if keys.is_empty() {
                    println!("No keys in the hash table.");
                } else {
                    println!("Keys in the hash table: {:?}", keys);
                }
            }
            "Free hash table" => {
                hash_table_free(*table);
                println!("Hash table has been freed.");
                break;
            }
            "Test" => {
                let min_length: usize = Text::new("Enter minimum word length:")
                    .prompt()
                    .unwrap()
                    .parse()
                    .unwrap();

                let file_paths = match get_txt_files_in_directory("rsrc") {
                    Ok(paths) => paths,
                    Err(e) => {
                        println!("Error reading files: {}", e);
                        continue;
                    }
                };

                if file_paths.is_empty() {
                    println!("No .txt files found in the 'rsrc' directory.");
                } else {
                    let file_choices: Vec<String> = file_paths
                        .iter()
                        .map(|path| path.to_string_lossy().into_owned())
                        .collect();

                    let selected_file = Select::new("Select a file to process", file_choices)
                        .prompt()
                        .unwrap();

                    let selected_file_path = file_paths
                        .iter()
                        .find(|path| path.to_string_lossy() == selected_file)
                        .unwrap();

                    println!("Processing file: {}", selected_file_path.display());

                    let start_time = Instant::now();

                    match read_file_and_extract_words(selected_file_path.to_str().unwrap()) {
                        Ok(words) => {
                            let filtered_words = filter_words_by_length(words, min_length);
                            if filtered_words.is_empty() {
                                println!(
                                    "No words found with the specified minimum length in '{}'.",
                                    selected_file_path.display()
                                );
                            } else {
                                println!(
                                    "Words with a length of at least {} characters in '{}':",
                                    min_length,
                                    selected_file_path.display()
                                );
                                for word in filtered_words {
                                    println!("{}", word);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Error reading file {}: {}", selected_file_path.display(), e);
                        }
                    }
                    let elapsed_time = start_time.elapsed();
                    println!(
                        "File processing completed in: {}.{:03} seconds",
                        elapsed_time.as_secs(),
                        elapsed_time.subsec_millis()
                    );
                }
            }
            "Exit" => {
                break;
            }
            _ => {}
        }
    }
}
