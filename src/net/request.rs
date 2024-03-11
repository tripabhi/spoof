use http::{HeaderMap, Method};
use http_body_util::BodyExt;
use url::Url;

#[derive(Debug, Clone)]
pub struct Request {
    pub url: Url,
    pub method: Method,
    pub headers: HeaderMap,
    pub body: Vec<u8>,
}

impl Request {
    pub(crate) async fn from_hyper(request: hyper::Request<hyper::body::Incoming>) -> Self {
        let (parts, body) = request.into_parts();
        let url = match parts.uri.authority() {
            Some(_) => parts.uri.to_string(),
            None => format!("http://localhost{}", parts.uri),
        }
        .parse()
        .unwrap();

        let body = body
            .collect()
            .await
            .expect("Failed to read request body.")
            .to_bytes();

        Self {
            url,
            method: parts.method,
            headers: parts.headers,
            body: body.to_vec(),
        }
    }
}
