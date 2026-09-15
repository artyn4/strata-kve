use std::path::PathBuf;

pub struct ManifestManager {
    pub path: PathBuf,
}

impl ManifestManager {
    pub fn new(path: PathBuf) -> Self {
        println!("manifest tracking state at {:?}", path);
        ManifestManager { path }
    }

    pub fn save_state(&self) {
        println!("saving manifest state...");
    }
}