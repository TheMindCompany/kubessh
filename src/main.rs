#[allow(unused_imports)]
#[macro_use]
extern crate slog;
#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate run_script;
extern crate sys_info;
extern crate regex;
extern crate dialoguer;
extern crate console;
extern crate dirs;

extern crate structopt;

mod command_control;
mod kube_ssh;
mod toolbelt;
mod prompt_user;

use kube_ssh::KubeSSHRunner;
use toolbelt::logs::RootLog;

use structopt::StructOpt;

fn main() {
    // This is the collection of settings sent from the request.
    let cli_options = command_control::CmdCtl::from_args();

    // This should be passed to any underlying modules and follow verbose logic rules for CLI.
    let log_config = RootLog::get_logger(
        cli_options.is_verbose()
    );

    match cli_options.commands {
        Some(command_control::Commands::Configurations(_)) => {
            cli_options.run_command_process();
        },
        None => {
            KubeSSHRunner::run(&cli_options);
        }
    }
}
