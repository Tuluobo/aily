use super::{ModelID, Provider};
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter)]
enum Model {
    Gpt4o,
    Gpt4,
    Gpt35,
    ChatGpt4o,
    O1Mini,
    O1Preview,
    DallE,
    Babbage,
    Whisper,
    OmniModeration,
    Tts1,
    Davinci,
    TextEmbedding,
}

impl ModelID for Model {
    fn identify(&self) -> &'static str {
        match self {
            Model::Gpt4o => "gpt-4o",
            Model::Gpt4 => "gpt-4",
            Model::Gpt35 => "gpt-3.5",
            Model::ChatGpt4o => "chatgpt-4o",
            Model::O1Mini => "o1-mini",
            Model::O1Preview => "o1-preview",
            Model::DallE => "dall-e",
            Model::Babbage => "babbage",
            Model::Whisper => "whisper",
            Model::OmniModeration => "omni-moderation",
            Model::Tts1 => "tts-1",
            Model::Davinci => "davinci",
            Model::TextEmbedding => "text-embedding",
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
