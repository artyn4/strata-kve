use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Write;

pub struct Wal {
    pub file_path: PathBuf,
}

impl Wal {
    pub fn new(p: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        println!("setting up wal at {:?}", p);
        Ok(Wal { file_path: p })
    }

    pub fn append(&mut self, k: &[u8], v: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .expect("wal file broken");

        f.write_all(&(k.len() as u32).to_le_bytes()).unwrap();
        f.write_all(&(v.len() as u32).to_le_bytes())?;
        f.write_all(k).unwrap();
        f.write_all(v).expect("failed writing value :(");
        f.flush()?;
        Ok(())
    }
}