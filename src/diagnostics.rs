pub struct DiagnosticsTracker {
    pub crash_count: u32,
}

impl DiagnosticsTracker {
    pub fn new() -> Self {
        DiagnosticsTracker { crash_count: 0 }
    }

    pub fn log_stats(&self) {
        println!("stats look great");
    }
}