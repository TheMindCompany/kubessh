use crate::prompt_user::model::PromptValue;

use dialoguer::theme::ColorfulTheme;
use dialoguer::{Select};

#[derive(Debug, Default, Clone)]
pub struct PromptValueHandler {}

impl PromptValueHandler {

    pub fn process(prompt: &PromptValue) -> Option<String> {
        PromptValueHandler::ask_with_options(
            &prompt.context,
            &prompt.default,
            &prompt.options,
        )
    }

    // Prompt with list of options.
    pub fn ask_with_options(
        context: &str,
        default: &str,
        value_options: &Option<Vec<String>>,
    ) -> Option<String> {
        let color_theme = ColorfulTheme::default();
        match value_options {
            Some(x) => {
                let response = Select::with_theme(&color_theme)
                    .with_prompt(&context)
                    .items(&x[..])
                    .default(x.iter().position(|x| x == default).unwrap_or(0))
                    .interact()
                    .unwrap();

                Some(x[response].to_owned())
            }
            None => {
                None
            }
        }
    }

}
