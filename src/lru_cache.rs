use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

pub struct LRUCache<K, V> {
    pub capacity: usize,
    pub usage: VecDeque<K>,
    pub map: HashMap<K, V>,
}

impl<K: Hash + Eq + Clone, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            usage: VecDeque::with_capacity(capacity),
            map: HashMap::new(),
        }
    }

    fn promote_key(&mut self, key: &K) {
        if let Some(pos) = self.usage.iter().position(|k| k == key) {
            self.usage.remove(pos);
            self.usage.push_front(key.clone());
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(&key) {
            self.promote_key(&key);
            self.map.get(&key)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.promote_key(&key);
            self.map.insert(key, value);
        } else {
            if self.map.len() == self.capacity {
                if let Some(least_used) = self.usage.pop_back() {
                    self.map.remove(&least_used);
                }
            }
            self.usage.push_front(key.clone());
            self.map.insert(key, value);
        }
    }
}

pub fn main_lru() {
    let mut cache = LRUCache::new(2);
    cache.put(1, "one");
    cache.put(2, "two");
    println!("{:?}", cache.get(&1));
    cache.put(3, "three");
    println!("{:?}", cache.get(&2));
}
