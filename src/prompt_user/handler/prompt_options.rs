use crate::prompt_user::model::PromptOptions;

use dialoguer::theme::ColorfulTheme;
use dialoguer::{Select};

#[derive(Debug, Default, Clone)]
pub struct PromptOptionsHandler {}

impl PromptOptionsHandler {

    pub fn process(prompt: &PromptOptions) -> Option<String> {
        PromptOptionsHandler::ask_with_options(
            &prompt.name,
            &prompt.default,
            &prompt.options,
        )
    }

    // Prompt with list of options.
    pub fn ask_with_options(
        name: &str,
        default: &str,
        value_options: &[String],
    ) -> Option<String> {
        let color_theme = ColorfulTheme::default();
        if !value_options.is_empty() {
                let mut context = "Select a ".to_string();
                context.push_str(&name);
                let response = Select::with_theme(&color_theme)
                    .with_prompt(&context)
                    .items(&value_options[..])
                    .default(value_options.iter().position(|x| x == default).unwrap_or(0))
                    .interact()
                    .unwrap();

                Some(value_options[response].to_owned())
        } else {
            None
        }
    }

}
