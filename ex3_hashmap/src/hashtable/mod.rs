use std::collections::VecDeque;
use std::hash::Hash;

struct KeyValue<K, V> {
    key: K,
    value: V,
}

pub struct HashTable<K, V>
where
    K: Eq + Hash,
{
    buckets: Vec<VecDeque<KeyValue<K, V>>>,
    size: usize,
    compare: Box<dyn Fn(&K, &K) -> bool>,
    hash_function: Box<dyn Fn(&K) -> u64>,
}

pub fn hash_table_create<K: Eq + Hash + 'static, V>(
    compare: fn(&K, &K) -> bool,
    hash_function: fn(&K) -> u64,
) -> Box<HashTable<K, V>> {
    let mut buckets = Vec::with_capacity(16);
    buckets.resize_with(16, VecDeque::new);

    Box::new(HashTable {
        buckets,
        size: 0,
        compare: Box::new(compare),
        hash_function: Box::new(hash_function),
    })
}

pub fn hash_table_put<K: Eq + Hash, V>(table: &mut HashTable<K, V>, key: K, value: V) {
    let index = (table.hash_function)(&key) as usize % table.buckets.len();
    let bucket = &mut table.buckets[index];

    for kv in bucket.iter_mut() {
        if (table.compare)(&kv.key, &key) {
            kv.value = value;
            return;
        }
    }

    bucket.push_back(KeyValue { key, value });
    table.size += 1;
}

pub fn hash_table_get<K: Eq + Hash, V>(table: &mut HashTable<K, V>, key: K) -> Option<&V> {
    let index = (table.hash_function)(&key) as usize % table.buckets.len();
    let bucket = &table.buckets[index];

    for kv in bucket.iter() {
        if (table.compare)(&kv.key, &key) {
            return Some(&kv.value);
        }
    }
    None
}

pub fn hash_table_contains_key<K: Eq + Hash, V>(table: &HashTable<K, V>, key: &K) -> i32 {
    let index = (table.hash_function)(key) as usize % table.buckets.len();
    let bucket = &table.buckets[index];

    for kv in bucket.iter() {
        if (table.compare)(&kv.key, key) {
            return 1;
        }
    }

    0
}

pub fn hash_table_remove<K: Eq + Hash, V>(table: &mut HashTable<K, V>, key: K) {
    let index = (table.hash_function)(&key) as usize % table.buckets.len();
    let bucket = &mut table.buckets[index];

    let _original_size = bucket.len();
    let mut found = false;

    bucket.retain(|kv| {
        if (table.compare)(&kv.key, &key) {
            found = true;
            false
        } else {
            true
        }
    });

    if found {
        table.size -= 1;
    }
}

pub fn hash_table_size<K: Eq + Hash, V>(table: &HashTable<K, V>) -> usize {
    table.size
}

pub fn hash_table_keyset<K: Eq + Hash + Clone, V>(table: &HashTable<K, V>) -> Vec<K> {
    let mut keys = Vec::new();

    for bucket in &table.buckets {
        for kv in bucket.iter() {
            keys.push(kv.key.clone());
        }
    }

    keys
}

pub fn hash_table_free<K: Eq + Hash, V>(table: HashTable<K, V>) {
    for mut bucket in table.buckets {
        bucket.clear();
    }
}
