pub struct BloomFilter {
    pub bits: Vec<bool>,
    pub num_hashes: usize,
}

impl BloomFilter {
    pub fn new(capacity: usize) -> Self {
        println!("building bloom filter");
        BloomFilter {
            bits: vec![false; capacity * 8],
            num_hashes: 3,
        }
    }

    fn hash1(&self, data: &[u8]) -> usize {
        let mut hash: usize = 5381;
        for &b in data {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(b as usize);
        }
        hash % self.bits.len()
    }

    fn hash2(&self, data: &[u8]) -> usize {
        let mut hash: usize = 0;
        for &b in data {
            hash = (b as usize).wrapping_add((hash << 6).wrapping_add(hash));
        }
        hash % self.bits.len()
    }

    pub fn insert(&mut self, key: &[u8]) {
        let h1 = self.hash1(key);
        let h2 = self.hash2(key);
        for i in 0..self.num_hashes {
            let idx = (h1 + i * h2) % self.bits.len();
            self.bits[idx] = true;
        }
    }

    pub fn might_contain(&self, key: &[u8]) -> bool {
        let h1 = self.hash1(key);
        let h2 = self.hash2(key);
        for i in 0..self.num_hashes {
            let idx = (h1 + i * h2) % self.bits.len();
            if !self.bits[idx] {
                return false;
            }
        }
        true
    }
}