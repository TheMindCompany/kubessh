use structopt::StructOpt;
use structopt::clap::AppSettings::*;
pub mod cmdctl;
pub mod completions;
pub mod configuration;

use super::install_handler::Installer;
use super::completion_handler::CompletionProcess;
pub use cmdctl::*;
pub use configuration::Configurations;

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    global_settings = &[DisableVersion, DeriveDisplayOrder, VersionlessSubcommands],
)]
pub enum Commands {
    /// Completion scripts for various shells.
    #[structopt(name = "configuration")]
    Configurations(Configurations),
}

impl Commands {
    pub fn process(&self) {
        match self.clone() {
            Commands::Configurations(config) => {
                match config {
                    Configurations::AppInstall(installs) => {
                        Installer::new().set_script_list(installs.install_order()).run();
                    },
                    Configurations::Completions(completion) => {
                        CompletionProcess::run(completion);
                    },
                }
            },
        }
    }

    pub fn is_verbose(&self) -> bool {
        match self.to_owned() {
            Commands::Configurations(_) => {
                false
            },
        }
    }
}
