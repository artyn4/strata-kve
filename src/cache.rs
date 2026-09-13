use std::collections::{HashMap, VecDeque};

pub struct BlockCache {
    pub capacity: usize,
    pub map: HashMap<Vec<u8>, Vec<u8>>,
    pub lru_queue: VecDeque<Vec<u8>>,
}

impl BlockCache {
    pub fn new(capacity: usize) -> Self {
        println!("allocating block cache of size {}", capacity);
        BlockCache {
            capacity,
            map: HashMap::new(),
            lru_queue: VecDeque::new(),
        }
    }

    pub fn get(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        if self.map.contains_key(key) {
            println!("cache hit");
            self.lru_queue.retain(|k| k != key);
            self.lru_queue.push_back(key.to_vec());
            return self.map.get(key).cloned();
        }
        println!("cache miss");
        None
    }

    pub fn put(&mut self, key: Vec<u8>, val: Vec<u8>) {
        if self.map.len() >= self.capacity {
            if let Some(old_key) = self.lru_queue.pop_front() {
                self.map.remove(&old_key);
                println!("removed old item from cache");
            }
        }
        self.lru_queue.push_back(key.clone());
        self.map.insert(key, val);
    }
}