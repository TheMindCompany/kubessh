use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{PathBuf};

#[derive(Debug, Default, Clone)]
pub struct CreateFile {}

impl CreateFile {
    pub fn new() -> CreateFile {
        Default::default()
    }

    pub fn create(self, filepath: PathBuf, text: &str) {
        let f = match File::create(filepath) {
            Ok(x) => x,
            Err(x) => {
                eprintln!("{:#?}", x);
                std::process::exit(1);
            }
        };
        let mut f = BufWriter::new(f);
        f.write_all(text.as_bytes()).expect("Unable to write data");
    }
}
