#[derive(Debug, Default, Clone)]
pub struct PromptValue {
    pub options: Option<Vec<String>>,
    pub default: String,
    pub context: String,
}

impl PromptValue {

    pub fn new() -> PromptValue {
        Default::default()
    }

    pub fn set_name(mut self, context: &str) -> PromptValue {
        self.context = context.to_string();
        self
    }

    pub fn set_options(mut self, options: Option<Vec<String>>) -> PromptValue {
        self.options = options;
        self
    }

    pub fn set_default(mut self, default: &str) -> PromptValue {
        self.default = default.to_string();
        self
    }

}
