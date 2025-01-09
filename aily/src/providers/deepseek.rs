use crate::providers::{ModelID, Provider};
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter)]
enum Model {
    Chat,
}

impl ModelID for Model {
    fn identify(&self) -> &'static str {
        match self {
            Model::Chat => "deepseek-chat",
        }
    }
}

pub struct DeepSeek;
impl Provider for DeepSeek {
    fn host(&self) -> &'static str {
        "https://api.deepseek.com"
    }

    fn models(&self) -> Vec<Box<dyn ModelID>> {
        Model::iter()
            .map(|m| Box::new(m) as Box<dyn ModelID>)
            .collect::<Vec<_>>()
    }
}
