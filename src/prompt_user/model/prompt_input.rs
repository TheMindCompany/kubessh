#[derive(Debug, Default, Clone)]
pub struct PromptInput {
    pub default: String,
    pub name: String,
}

impl PromptInput {

    pub fn new() -> PromptInput {
        Default::default()
    }

    pub fn set_name(mut self, name: &str) -> PromptInput {
        self.name = name.to_string();
        self
    }

    pub fn set_default(mut self, default: &str) -> PromptInput {
        self.default = default.to_string();
        self
    }

}
