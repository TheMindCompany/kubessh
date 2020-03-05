use crate::prompt_user::model::PromptInput;

use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input};

#[derive(Debug, Default, Clone)]
pub struct PromptInputHandler {}

impl PromptInputHandler {

    pub fn process(prompt: &PromptInput) -> String {
        PromptInputHandler::ask_input(
            &prompt.name,
            &prompt.default,
        )
    }

    // Prompt with list of options.
    pub fn ask_input(
        name: &str,
        _default: &str,
    ) -> String {
        let color_theme = ColorfulTheme::default();
        let question = format!("Set value for {}", name);
        let mut prompt = Input::with_theme(&color_theme);
        prompt.default("".to_owned());

        prompt
            .with_prompt(&question.as_str())
            .interact()
            .unwrap()
    }

}
