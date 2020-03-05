use std::process::{Command};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CommandLineHandler {
    exit_on_error: bool
}

impl CommandLineHandler {
    pub fn new() -> CommandLineHandler {
        Default::default()
    }

    pub fn set_exit_on_error(&mut self, toggle: bool) {
        self.exit_on_error = toggle;
    }

    pub fn run_cmd(&mut self, cmd: &str, dry_run: bool, verbose: bool) -> String {
        if dry_run {
            println!("dry_run: {}", cmd);
            "".to_string()
        } else {
            if verbose {
                println!("running: {}", cmd);
                "".to_string();
            }
            let output = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                            .args(&["/C", &cmd])
                            .output()
                            .expect("failed to execute process")
                } else {
                    Command::new("sh")
                            .arg("-c")
                            .arg(&cmd)
                            .output()
                            .expect("failed to execute process")
                };

            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).to_ascii_lowercase()
            } else {
                let mut response = String::new();
                if !output.stderr.is_empty() {
                    match String::from_utf8(output.stderr) {
                        Ok(x) => {
                            if self.exit_on_error {
                                eprintln!("{}", x);
                            } else {
                                response.push_str(&x);
                            }
                        },
                        Err(e) => eprintln!("{:#?}", e),
                    }
                }

                if !output.stdout.is_empty() {
                    match String::from_utf8(output.stdout) {
                        Ok(x) => {
                            if self.exit_on_error {
                                eprintln!("{}", x);
                            } else {
                                response.push_str(&x);
                            }
                        },
                        Err(e) => eprintln!("{:#?}", e),
                    }
                }

                if self.exit_on_error {
                    std::process::exit(1);
                } else {
                    response
                }

            }
        }
    }

}
