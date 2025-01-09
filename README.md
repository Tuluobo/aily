# Aily

Aily is a library for building AI applications.

## Features

- Support for multiple AI providers
- Easy to use API
- Fast and efficient

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
aily = "0.1.0"
```

## Usage

```
use aily::{Client, Method};
use aily::header::HeaderMap;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let headers = HeaderMap::new();
    let body = "{\"model\":\"chat-4o\"}";
    let response = client.request("/v1/chat/completions", Method::POST, headers, body).await;
    println!("{:?}", response);
}
```

## License

This project is licensed under the MIT license. See [LICENSE](LICENSE) for more information.