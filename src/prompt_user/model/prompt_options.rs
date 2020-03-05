#[derive(Debug, Default, Clone)]
pub struct PromptOptions {
    pub options: Vec<String>,
    pub default: String,
    pub name: String,
}

impl PromptOptions {

    pub fn new() -> PromptOptions {
        Default::default()
    }

    pub fn set_name(mut self, name: &str) -> PromptOptions {
        self.name = name.to_string();
        self
    }

    pub fn set_options(mut self, options: &[String]) -> PromptOptions {
        self.options = options.to_owned();
        self
    }

    pub fn set_default(mut self, default: &str) -> PromptOptions {
        self.default = default.to_string();
        self
    }

}
