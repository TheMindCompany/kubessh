#[macro_use]
extern crate run_script;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate sys_info;
extern crate clap;
extern crate structopt;
extern crate handlebars;

include!("command_control/mod.rs");
include!("command_control/install_handler/generator/mod.rs");
include!("command_control/completion_handler/generator/mod.rs");

use clap::Shell;
use std::path::Path;

fn main() {
    let app_name: Option<&'static str> = option_env!("CARGO_PKG_NAME");

    create_completion_scripts(app_name.unwrap_or("app"));
    create_completion_mod(app_name.unwrap_or("app"));
    clean(app_name.unwrap_or("app"));

    InstallScriptModBuilder::new().create()
}

fn create_completion_scripts(name: &'static str) {
    let out_dir = Path::new("./src/command_control/completion_handler/");
    let mut app = CmdCtl::clap();

    app.gen_completions(name, Shell::Bash, out_dir);
    app.gen_completions(name, Shell::Fish, out_dir);
    app.gen_completions(name, Shell::Zsh, out_dir);
    app.gen_completions(name, Shell::PowerShell, out_dir);
    app.gen_completions(name, Shell::Elvish, out_dir);
}

fn create_completion_mod(name: &'static str) {
    let builder =  CompletionScriptModBuilder::new();
    let completion_scripts = CompletionScriptPath::new(name).get_paths();
    let templates = vec![
        ModTemplate::get_top_template(),
        ModTemplate::get_fish_template(),
        ModTemplate::get_zsh_template(),
        ModTemplate::get_ps1_template(),
        ModTemplate::get_elvish_template(),
        ModTemplate::get_bottom_template(),
    ];

    builder.create(completion_scripts, templates);
}

fn clean(name: &'static str) {
    let files = CompletionScriptPath::new(name).get_paths();
    for i in 0..5 {
        match std::fs::remove_file(files[i].to_path_buf()) {
            Ok(x) => println!("{:#?}", x),
            Err(e) => eprint!("Error removing file: {}", e),
        }
    }
}
