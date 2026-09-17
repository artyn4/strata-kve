pub struct CompactionEngine {
    pub running: bool,
}

impl CompactionEngine {
    pub fn new() -> Self {
        println!("compaction engine starting");
        CompactionEngine { running: true }
    }

    pub fn run_background_merge(&self) {
        if self.running {
            println!("merging sstables");
        }
    }
}