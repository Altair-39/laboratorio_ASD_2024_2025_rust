use inquire::{Select, Text};
use std::hash::{Hash, Hasher};

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
            "Exit" => {
                break;
            }
            _ => {}
        }
    }
}
