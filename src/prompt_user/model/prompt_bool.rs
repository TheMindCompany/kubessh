#[derive(Debug, Default, Clone)]
pub struct PromptBool {
    pub question: String,
}

impl PromptBool {

    pub fn new() -> PromptBool {
        Default::default()
    }

    pub fn set_question(mut self, question: &str) -> PromptBool {
        self.question = question.to_string();
        self
    }

}
