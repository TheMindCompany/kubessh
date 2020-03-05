use run_script;
use run_script::ScriptOptions;

pub fn run(cmd: &str, dry_run: bool, verbose: bool) {
    let options = get_options();

    if dry_run {
        println!("dry-run: {}", cmd);
    } else {
        if verbose {
            println!("running: {}", cmd);
        }
        run_script::run(
            &cmd,
            &vec![],
            &options
        ).unwrap();
    }
}


pub fn get_options() -> ScriptOptions {
    let mut options = ScriptOptions::new();

    options.runner = None;
    options.exit_on_error = true;
    options.output_redirection = run_script::IoOptions::Inherit;
    options.print_commands = false;

    options
}

pub fn process(command: String, dry_run: bool, verbose: bool) {
    run(&command, dry_run, verbose);
}
