use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::collections::BTreeMap;

pub struct SsTable {
    pub file_path: PathBuf,
    pub id: usize,
    pub index_blocks: Vec<(Vec<u8>, u64)>,
}

impl SsTable {
    pub fn create(path: PathBuf, id: usize, data: &BTreeMap<Vec<u8>, Vec<u8>>) -> Result<Self, Box<dyn std::error::Error>> {
        println!("flushing memtable to sstable file #{} ", id);
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .expect("sstable file creation failed :(");

        let mut index_blocks = Vec::new();
        let mut current_offset = 0u64;

        for (k, v) in data {
            index_blocks.push((k.clone(), current_offset));

            let k_len = k.len() as u32;
            let v_len = v.len() as u32;

            f.write_all(&k_len.to_le_bytes()).unwrap();
            f.write_all(&v_len.to_le_bytes()).unwrap();
            f.write_all(k).unwrap();
            f.write_all(v).unwrap();

            current_offset += 4 + 4 + k_len as u64 + v_len as u64;
        }

        f.sync_all().expect("disk sync failed");
        println!("wrote {} keys into sstable #{}", data.len(), id);

        Ok(SsTable {
            file_path: path,
            id,
            index_blocks,
        })
    }

    pub fn read_key(&self, target_key: &[u8]) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
        let idx = self.index_blocks.binary_search_by(|(k, _)| k.as_slice().cmp(target_key));

        let offset = match idx {
            Ok(i) => self.index_blocks[i].1,
            Err(i) => {
                if i == 0 { return Ok(None); }
                self.index_blocks[i - 1].1
            }
        };

        let mut f = File::open(&self.file_path).expect("couldnt open sstable");
        f.seek(SeekFrom::Start(offset)).unwrap();

        let mut len_buf = [0u8; 4];

        loop {
            if f.read_exact(&mut len_buf).is_err() { break; }
            let k_len = u32::from_le_bytes(len_buf);

            f.read_exact(&mut len_buf).unwrap();
            let v_len = u32::from_le_bytes(len_buf);

            let mut k_buf = vec![0u8; k_len as usize];
            f.read_exact(&mut k_buf).unwrap();

            let mut v_buf = vec![0u8; v_len as usize];
            f.read_exact(&mut v_buf).unwrap();

            if k_buf == target_key {
                return Ok(Some(v_buf));
            }
        }

        Ok(None)
    }
}