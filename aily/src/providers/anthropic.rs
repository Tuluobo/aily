use crate::{ModelID, Provider};
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter)]
enum Model {
    Claude35Sonnet,
    Claude35Haiku,
    Claude3Haiku,
    Claude3Opus,
    Claude3Sonnet,
    Claude21,
    Claude20,
}

impl ModelID for Model {
    fn identify(&self) -> &'static str {
        match self {
            Model::Claude35Sonnet => "claude-3-5-sonnet",
            Model::Claude35Haiku => "claude-3-5-haiku",
            Model::Claude3Haiku => "claude-3-haiku",
            Model::Claude3Opus => "claude-3-opus",
            Model::Claude3Sonnet => "claude-3-sonnet",
            Model::Claude21 => "claude-2.1",
            Model::Claude20 => "claude-2.0",
        }
    }
}

pub struct Anthropic;

impl Provider for Anthropic {
    fn host(&self) -> &'static str {
        "https://api.anthropic.com"
    }

    fn models(&self) -> Vec<Box<dyn ModelID>> {
        Model::iter()
            .map(|m| Box::new(m) as Box<dyn ModelID>)
            .collect::<Vec<_>>()
    }
}
