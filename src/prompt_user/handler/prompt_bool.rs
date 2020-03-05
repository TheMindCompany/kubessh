use crate::prompt_user::model::PromptBool;

use console::Style;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirmation};

#[derive(Debug, Default, Clone)]
pub struct PromptBoolHandler {}

impl PromptBoolHandler {

    pub fn process(prompt: &PromptBool) -> bool {
        PromptBoolHandler::ask_bool(
            &prompt.question
        )
    }

    // Prompt with list of options.
    pub fn ask_bool(
        context: &str
    ) -> bool {
        let color_theme = ColorfulTheme {
            yes_style: Style::new().cyan(),
            no_style: Style::new().cyan(),
            ..ColorfulTheme::default()
        };
        Confirmation::with_theme(&color_theme)
            .with_text(&context)
            .interact()
            .unwrap()
    }

}
