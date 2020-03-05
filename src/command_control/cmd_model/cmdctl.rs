use structopt::StructOpt;
use structopt::clap::AppSettings::*;
use super::Commands;

#[derive(Debug, StructOpt, Default, Clone)]
#[structopt(
    global_settings = &[DisableVersion, DeriveDisplayOrder, VersionlessSubcommands],
    about = "\nMerge two schema bodies together."
)]
pub struct CmdCtl {
    /// Pod target.
    ///
    pub pod: Option<String>,

    /// Container target.
    ///
    pub container: Option<String>,

    /// Filter container list. ( ie: my-deployment-name )
    #[structopt(short = "f", long = "filter")]
    pub filter: Option<String>,

    /// Namespace target. ( ie: Environment )
    #[structopt(short = "n", long = "namespace")]
    pub namespace: Option<String>,

    /// Cluster target. 
    #[structopt(short = "c", long = "context")]
    pub context: Option<String>,

    /// Update token for eks using aws profile.
    #[structopt(long = "eks")]
    pub eks: Option<String>,

    /// Enable verbose logging.
    #[structopt(long = "verbose", short = "v")]
    pub verbose: bool,

    /// Perform dry-run analysis.
    #[structopt(long = "dry-run")]
    pub dry_run: bool,

    #[structopt(subcommand)]
    pub commands: Option<Commands>,
}

impl CmdCtl {

    pub fn run_command_process(self) -> CmdCtl {
        match &self.commands {
            Some(commands) => {
                commands.process();
                self
            },
            None => self
        }
    }

    pub fn is_verbose(&self) -> bool {
        match self.commands.clone() {
            Some(commands) => commands.is_verbose(),
            None => self.verbose
        }
    }

}
