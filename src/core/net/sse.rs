use async_gen::AsyncIter;
use reqwest::{header, Client};
use tokio_stream::Stream;

/// A client for working with Server-Sent Events.
pub struct SseClient;

impl SseClient {
    pub fn post(url: &str, body: Option<String>) -> impl Stream<Item = String> {
        let client = Client::new();
        let mut req = Client::post(&client, url)
            .header(header::ACCEPT, "text/event-stream")
            .header(header::CACHE_CONTROL, "no-cache")
            .header(header::CONNECTION, "keep-alive")
            .header(header::CONTENT_TYPE, "application/json");
        if let Some(body) = body {
            req = req.body(body);
        }
        let req = req.build().unwrap();

        AsyncIter::from(async_gen::gen! {
            let mut conn = client.execute(req).await.unwrap();
            while let Some(event) = conn.chunk().await.unwrap() {
                yield std::str::from_utf8(&event).unwrap().to_owned();
            }
        })
    }
}
