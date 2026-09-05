//borrow checker
pub mod bloom;
pub mod cache;
pub mod compaction;
pub mod diagnostics;
pub mod engine;
pub mod manifest;
pub mod memtable;
pub mod metrics;
pub mod sstable;
pub mod wal;

pub fn sanity_check() {
    println!("it compiled");
}