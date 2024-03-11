use bytes::Bytes;
use http::{HeaderMap, HeaderName, HeaderValue, Response, StatusCode};
use http_body_util::Full;
use serde::Serialize;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct MockResponseBuilder {
    mime_type: String,
    status_code: Option<StatusCode>,
    headers: HeaderMap,
    body: Option<Vec<u8>>,
    delay: Option<Duration>,
}

impl MockResponseBuilder {
    pub fn new() -> Self {
        MockResponseBuilder {
            mime_type: String::new(),
            status_code: None,
            headers: HeaderMap::new(),
            body: None,
            delay: None,
        }
    }

    pub fn with_status_code<S>(mut self, status_code: S) -> Self
    where
        S: TryInto<StatusCode>,
        <S as TryInto<StatusCode>>::Error: std::fmt::Debug,
    {
        let status_code = status_code
            .try_into()
            .expect("Cannot convert field to HTTP Status Code");
        self.status_code = Some(status_code);
        self
    }

    pub fn with_header_append<K, V>(mut self, key: K, value: V) -> Self
    where
        K: TryInto<HeaderName>,
        <K as TryInto<HeaderName>>::Error: std::fmt::Debug,
        V: TryInto<HeaderValue>,
        <V as TryInto<HeaderValue>>::Error: std::fmt::Debug,
    {
        let key = key.try_into().expect("Cannot convert field to header name");
        let value = value
            .try_into()
            .expect("Cannot convert field to header value");
        self.headers.append(key, value);
        self
    }

    pub fn with_header_insert<K, V>(mut self, key: K, value: V) -> Self
    where
        K: TryInto<HeaderName>,
        <K as TryInto<HeaderName>>::Error: std::fmt::Debug,
        V: TryInto<HeaderValue>,
        <V as TryInto<HeaderValue>>::Error: std::fmt::Debug,
    {
        let key = key.try_into().expect("Cannot convert field to header name");
        let value = value
            .try_into()
            .expect("Cannot convert field to header value");
        self.headers.insert(key, value);
        self
    }

    pub fn with_body_bytes<B>(mut self, body: B) -> Self
    where
        B: TryInto<Vec<u8>>,
        <B as TryInto<Vec<u8>>>::Error: std::fmt::Debug,
    {
        let body = body.try_into().expect("Cannot convert body to byte stream");
        self.body = Some(body);
        self
    }

    pub fn with_body_json<S>(mut self, body: S) -> Self
    where
        S: Serialize,
    {
        let body = serde_json::to_vec(&body).expect("Cannot deserialize body to byte stream");

        self.body = Some(body);
        self.mime_type = "application/json".to_string();
        self
    }

    pub fn with_body_string<T>(mut self, body: T) -> Self
    where
        T: TryInto<String>,
        <T as TryInto<String>>::Error: std::fmt::Debug,
    {
        let body = body.try_into().expect("Cannot convert field to String");

        self.body = Some(body.into_bytes());
        self.mime_type = "text/plain".to_string();
        self
    }

    pub fn with_body<T>(mut self, body: T, mime_type: &str) -> Self
    where
        T: TryInto<Vec<u8>>,
        <T as TryInto<Vec<u8>>>::Error: std::fmt::Debug,
    {
        let body = body
            .try_into()
            .expect("Cannot convert field to byte stream");

        self.body = Some(body);
        self.mime_type = mime_type.to_string();
        self
    }

    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }

    pub(crate) fn build_response(&self) -> Response<Full<Bytes>> {
        let status_code = self.status_code.unwrap_or_else(|| StatusCode::OK);
        let mut headers = self.headers.clone();

        if !self.mime_type.is_empty() {
            headers.insert(http::header::CONTENT_TYPE, self.mime_type.parse().unwrap());
        }

        let body = self.body.clone().unwrap_or_default();

        let mut response_builder = Response::builder().status(status_code);

        *response_builder.headers_mut().unwrap() = headers;

        response_builder.body(body.into()).unwrap()
    }
}
