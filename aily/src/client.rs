use crate::providers::{anthropic, deepseek, openai, Provider};
use reqwest::header::HeaderMap;
use reqwest::{Body, Method, Response, Url};

pub struct Client {
    inner: reqwest::Client,
    providers: Vec<Box<dyn Provider>>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            inner: reqwest::Client::new(),
            providers: vec![
                Box::new(deepseek::DeepSeek),
                Box::new(anthropic::Anthropic),
                Box::new(openai::OpenAI),
            ],
        }
    }

    pub fn add_provider(mut self, provider: Box<dyn Provider>) -> Self {
        self.providers.push(provider);
        self
    }

    pub async fn request<T: Into<Body>>(
        &self,
        path: &str,
        method: Method,
        mut headers: HeaderMap,
        body: T,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let body = body.into();
        let Some(provider) = self.providers.iter().find(|p| p.is_self(body.as_bytes())) else {
            return Err("No provider found".into());
        };

        let base_url = provider.host();
        let target_url = format!("{}{}", base_url, path);
        if let Ok(url) = Url::parse(&base_url) {
            if let Some(Ok(value)) = url.host_str().map(|host| host.parse()) {
                headers.insert(reqwest::header::HOST, value);
            }
        }
        log::debug!(
            "Method: {}, Target URL: {}, Headers: {:?}",
            method,
            target_url,
            headers
        );

        let request = self
            .inner
            .request(method, target_url)
            .headers(headers)
            .body(body);
        log::debug!("Request: {:?}", request);

        let response = request.send().await.map(|mut resp| {
            if let Ok(value) = "Aily".parse() {
                resp.headers_mut().insert("X-Forwarded-By", value);
            }
            resp
        })?;
        Ok(response)
    }
}
