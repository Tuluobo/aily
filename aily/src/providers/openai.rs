use super::{ModelID, Provider};
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter)]
enum Model {
    GPT4o,
    GPT4oMini,
    GPT4,
    GPT35turbo,
}

impl ModelID for Model {
    fn identify(&self) -> &'static str {
        match self {
            Model::GPT4o => "gpt-4o",
            Model::GPT4oMini => "gpt-4o-mini",
            Model::GPT4 => "gpt-4",
            Model::GPT35turbo => "gpt-3.5-turbo",
        }
    }
}

pub struct OpenAI;

impl Provider for OpenAI {
    fn host(&self) -> &'static str {
        "https://api.openai.com"
    }

    fn models(&self) -> Vec<Box<dyn ModelID>> {
        Model::iter()
            .map(|m| Box::new(m) as Box<dyn ModelID>)
            .collect::<Vec<_>>()
    }
}
