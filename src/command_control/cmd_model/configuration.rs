use structopt::StructOpt;
use super::completions::Completions;

#[derive(StructOpt, Debug, Clone)]
pub enum Configurations {
    /// Install packages needed to run this cli wrapper.
    #[structopt(name = "install")]
    AppInstall(AppInstall),

    /// Completion scripts for various shells.
    #[structopt(name = "completions")]
    Completions(Completions),
}

#[derive(StructOpt, Debug, Default, Clone)]
pub struct AppInstall {
    /// Available installation packages.
    ///
    /// python3, ansible, awstools, kubectl, pip, ruby, gitcrypt, hubflow, packer, jdk, busbar, mongodb, redis, hub

    #[structopt(default_value = "all")]
    pub packages: Vec<String>,
}

impl AppInstall {
    pub fn install_order(self) -> Vec<String> {
        let mut reordered_list = Vec::new();
        let installs = vec![
            "awstools".to_string(),
            "kubectl".to_string(),
        ];

        if self.packages[0] == "all" {
            reordered_list = installs;
        } else {
            for installation in installs {
                if self.packages.contains(&installation) {
                    reordered_list.push(installation);
                }
            }
        }

        reordered_list
    }
}
