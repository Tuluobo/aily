use aily::StatusCode;
use futures::StreamExt;
use std::{collections::HashMap, str::FromStr};
use worker::*;

#[event(fetch)]
async fn fetch(mut req: Request, env: Env, _ctx: Context) -> Result<Response> {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    // Hello World example
    let response = Router::new()
        .get("/", |_req, _ctx| {
            Response::from_json(&HashMap::from([(
                "message",
                "Please visite https://github.com/Tuluobo/aily",
            )]))
        })
        .run(req.clone()?, env)
        .await;
    if let Ok(resp) = response {
        if resp.status_code() != StatusCode::NOT_FOUND.as_u16() {
            return Ok(resp);
        }
    }

    // Forward request to aily
    let path = req.path();
    let method = aily::Method::from_str(req.method().as_ref()).map_err(|err| {
        log::error!("Error parsing method: {}", err);
        worker::Error::RustError("Method parse error".to_string())
    })?;
    let headers = req.headers().clone().into();
    let body = req.bytes().await?;
    let client = aily::Client::new();
    let response = client.request(&path, method, headers, body).await;
    log::debug!("Forwarding request to: {}", req.path());
    let resp_builder = Response::builder();
    match response {
        Ok(res) => {
            let status_code = res.status().as_u16();
            let headers = res.headers().into();
            let stream = res
                .bytes_stream()
                .map(|result| result.map_err(|err| worker::Error::RustError(err.to_string())));
            let response = resp_builder
                .with_status(status_code)
                .with_headers(headers)
                .from_stream(stream)?;
            Ok(response)
        }
        Err(error) => {
            log::error!("Error forwarding request: {}", error);
            resp_builder
                .with_status(StatusCode::INTERNAL_SERVER_ERROR.as_u16())
                .from_json(&HashMap::from([("message", error.to_string())]))
        }
    }
}
