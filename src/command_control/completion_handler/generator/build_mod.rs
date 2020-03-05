use std::fs::File;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Clone)]
pub struct CompletionScriptModBuilder {}

impl CompletionScriptModBuilder {

    pub fn new() -> CompletionScriptModBuilder {
        CompletionScriptModBuilder {}
    }

    pub fn create(&self, completion_scripts: Vec<PathBuf>, templates: Vec<String>) {
        let file_path = Path::new("./src/command_control/completion_handler/completion_scripts.rs");

        File::create(&file_path).unwrap();
        for i in 0..5 {
            self.merge_files_to_completion(
                file_path.to_path_buf(),
                completion_scripts[i].to_path_buf(),
                templates[i].clone(),
            );
        }
        self.file_to_completion(file_path.to_path_buf(), templates[5].clone());
    }

    fn merge_files_to_completion(&self, out: PathBuf, script: PathBuf, template: String) {
        let mut tmp_script = String::new();
        let mut file_script = File::open(script).expect("");

        file_script.read_to_string(&mut tmp_script).expect("");

        let mut outfile = OpenOptions::new()
            .write(true)
            .append(true)
            .open(out)
            .unwrap();
        outfile
            .write_fmt(format_args!("{}{}", template, tmp_script))
            .expect("Error writing updated completions.rs module.");
    }

    fn file_to_completion(&self, out: PathBuf, template: String) {
        let mut outfile = OpenOptions::new()
            .write(true)
            .append(true)
            .open(out)
            .unwrap();
        outfile
            .write_fmt(format_args!("{}", template))
            .expect("Error writing updated completions.rs module.");
    }

}
