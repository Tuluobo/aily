/// Anthropic provider
pub mod anthropic;
/// DeepSeek provider
pub mod deepseek;
/// OpenAI provider
pub mod openai;

pub trait ModelID {
    fn identify(&self) -> &'static str;
}

pub trait Provider {
    fn host(&self) -> &'static str;

    fn models(&self) -> Vec<Box<dyn ModelID>>;

    fn is_self(&self, body: Option<&[u8]>) -> bool {
        let Some(body) = body else {
            return false;
        };
        if body.is_empty() {
            return false;
        };
        match serde_json::from_slice::<serde_json::Value>(body) {
            Ok(body) => {
                let Some(model_name) = body.get("model").and_then(|model| model.as_str()) else {
                    return false;
                };
                self.models()
                    .iter()
                    .any(|m| model_name.starts_with(m.identify()))
            }
            Err(error) => {
                log::error!("body deserialize error: {:?}, body: {:?}", error, body);
                false
            }
        }
    }
}
