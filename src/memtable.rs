use std::collections::BTreeMap;

pub struct MemTable {
    pub data: BTreeMap<Vec<u8>, Vec<u8>>,
    size_bytes: usize,
}

impl MemTable {
    pub fn new() -> Self {
        println!("making a new memtable pog");
        MemTable {
            data: BTreeMap::new(),
            size_bytes: 0,
        }
    }

    pub fn insert(&mut self, k: Vec<u8>, v: Vec<u8>) {
        self.size_bytes += k.len() + v.len();
        self.data.insert(k, v);
    }

    pub fn lookup(&self, k: &[u8]) -> Option<Vec<u8>> {
        self.data.get(k).cloned()
    }

    pub fn estimated_size(&self) -> usize {
        self.size_bytes
    }
}