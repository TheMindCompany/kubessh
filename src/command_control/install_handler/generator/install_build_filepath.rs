use std::path::{PathBuf};

#[derive(Debug, Default, Clone)]
pub struct InstallScriptPath {
    paths: Vec<PathBuf>,
}

impl InstallScriptPath {

    pub fn new() -> InstallScriptPath {
        InstallScriptPath {
            paths: Vec::new()
        }
    }

    pub fn get_paths(mut self) -> Vec<PathBuf> {
        self.collect_script_paths();
        self.paths
    }

    pub fn collect_script_paths(&mut self) -> Vec<PathBuf> {
        let src_dir = String::from("./src/command_control/install_handler/generator/install_scripts/");
        let mut list = Vec::new();

        match std::fs::read_dir(&src_dir) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(s) => {
                            let file_path = s.path().clone();

                            if file_path.is_file() {
                                list.push(file_path.to_path_buf());
                            }
                        },
                        Err(err) => {eprintln!("err: {:#?}", err);},
                    }
                }
            },
            Err(err) => {
                eprintln!("err: {:#?}", err);
                std::process::exit(1);
            },
        }

        list
    }

}
