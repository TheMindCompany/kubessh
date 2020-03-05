pub mod scripts;

use run_script;
use run_script::ScriptOptions;

#[derive(Debug, Default, Clone)]
pub struct Installer {
    pub scripts:  Vec<(String, String)>,
}

impl Installer {
    pub fn new() -> Installer {
        Default::default()
    }

    pub fn run(&mut self) {
        for installer_script in self.scripts.clone() {
            println!("Adding package: {} {:#<10}\n", installer_script.0, " ");
            self.install(installer_script.1.clone());
            println!();
        }
    }

    pub fn set_script_list(&mut self, install_scripts_request: Vec<String>) -> Installer {
        let mut install_scripts = Vec::new();
        for install in install_scripts_request {
            install_scripts.push((install.clone(), scripts::InstallScript::get_script_named(install)));
        }
        self.scripts = install_scripts;
        self.clone()
    }

    pub fn install(&mut self, installer_script: String) {
        let options = self.get_options();

        run_script::run(
            &installer_script,
            &vec![],
            &options
        ).unwrap();
    }


    pub fn get_options(&mut self) -> ScriptOptions {
        let mut options = ScriptOptions::new();

        options.runner = None;
        options.exit_on_error = true;
        options.output_redirection = run_script::IoOptions::Inherit;
        options.print_commands = false;

        options
    }
}
