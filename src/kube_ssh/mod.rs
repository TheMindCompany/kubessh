mod line_formatter;
mod list;
mod run;

use list::KubeObjectListPrompt;
use crate::command_control::cmd_model::cmdctl::CmdCtl;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct KubeSSHRunner {}

impl KubeSSHRunner {
    pub fn run(request: &CmdCtl) {
        let dry_run = request.dry_run;
        let verbose = request.verbose;
        let request = KubeObjectListPrompt::new().process(request.clone());

        // Generate command string from user request options.
        let command = line_formatter::format_request(&request);

        // Run command.  Let it handle dry-run.
        run::process(command, dry_run, verbose);
    }
}
