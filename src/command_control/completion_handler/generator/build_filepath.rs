use std::path::{Path, PathBuf};

#[derive(Debug, Default, Clone)]
pub struct CompletionScriptPath {
    name: &'static str,
    paths: Vec<PathBuf>,
}

impl CompletionScriptPath {

    pub fn new(name: &'static str) -> CompletionScriptPath {
        CompletionScriptPath {
            name,
            paths: Vec::new()
        }
    }

    pub fn get_paths(mut self) -> Vec<PathBuf> {
        self.collect_script_paths();
        self.paths
    }

    fn collect_script_paths(&mut self) {
        let src_dir = String::from("./src/command_control/completion_handler/");

        // Supported shells.
        self.paths.push(create_pathbuf(self.bash_string_path(&mut src_dir.clone()).clone()));
        self.paths.push(create_pathbuf(self.fish_string_path(&mut src_dir.clone()).clone()));
        self.paths.push(create_pathbuf(self.zsh_string_path(&mut src_dir.clone()).clone()));
        self.paths.push(create_pathbuf(self.ps_string_path(&mut src_dir.clone()).clone()));
        self.paths.push(create_pathbuf(self.elvish_string_path(&mut src_dir.clone()).clone()));
    }

    fn bash_string_path(&self, bash_dir: &mut String) -> String {
        bash_dir.push_str(self.name);
        bash_dir.push_str(".bash");
        bash_dir.to_string()
    }

    fn fish_string_path(&self, fish_dir: &mut String) -> String {
        fish_dir.push_str(self.name);
        fish_dir.push_str(".fish");
        fish_dir.to_string()
    }

    fn zsh_string_path(&self, zsh_dir: &mut String) -> String {
        zsh_dir.push_str("_");
        zsh_dir.push_str(self.name);
        zsh_dir.to_string()
    }

    fn ps_string_path(&self, ps_dir: &mut String) -> String {
        ps_dir.push_str("_");
        ps_dir.push_str(self.name);
        ps_dir.push_str(".ps1");
        ps_dir.to_string()
    }

    fn elvish_string_path(&self, elvish_dir: &mut String) -> String {
        elvish_dir.push_str(self.name);
        elvish_dir.push_str(".elv");
        elvish_dir.to_string()
    }

}

fn create_pathbuf(string_path: String) -> PathBuf {
    Path::new(&string_path).to_path_buf()
}
