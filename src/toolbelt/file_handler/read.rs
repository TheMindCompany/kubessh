use std::fs;
use std::path::{PathBuf};

#[derive(Debug, Default, Clone)]
pub struct ReadFile {}

impl ReadFile {
    pub fn new() -> ReadFile {
        Default::default()
    }

    pub fn load(self, file: PathBuf) -> String {
        match self.read_into_string(file) {
            Ok(x) => x,
            Err(e) => {
                e
            }
        }
    }

    pub fn read_into_string(self, file: PathBuf) -> Result<String, String> {
        let f = fs::read_to_string(file.clone());
        match f {
            Ok(x) => Ok(x),
            Err(e) => {
                Err(format!("Failed to open file: {:?}\nError: {}", file.to_str().unwrap(), e))
            }
        }
    }
}
