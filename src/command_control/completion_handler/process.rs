use super::cmd_model::completions::Completions;

pub struct CompletionProcess {}

impl CompletionProcess {
    pub fn run(cli_settings: Completions) {
        match cli_settings {
            Completions::Bash(_) => {
                super::CompletionScript::bash();
            }
            Completions::Fish(_) => {
                super::CompletionScript::fish();
            }
            Completions::Zsh(_) => {
                super::CompletionScript::zsh();
            }
            Completions::PowerShell(_) => {
                super::CompletionScript::powershell();
            }
            Completions::Elvish(_) => {
                super::CompletionScript::elvish();
            }
        }
    }
}
