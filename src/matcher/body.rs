use crate::core::mock::Match;
use crate::net::request::Request;
use assert_json_diff::{assert_json_matches_no_panic, CompareMode};
use log::debug;
use serde::Serialize;
use serde_json::Value;

enum Body {
    Bytes(Vec<u8>),
    Json(Value),
}

pub struct BodyExactMatcher(Body);

impl BodyExactMatcher {
    pub fn string<T>(body: T) -> Self
    where
        T: Into<String>,
    {
        let body = body.into();
        Self(Body::Bytes(body.into_bytes()))
    }

    pub fn bytes<T>(body: T) -> Self
    where
        T: Into<Vec<u8>>,
    {
        let body = body.into();
        Self(Body::Bytes(body))
    }

    pub fn json<T>(body: T) -> Self
    where
        T: Serialize,
    {
        let body = serde_json::to_value(body).expect("Cannot parse object as JSON");
        Self(Body::Json(body))
    }

    pub fn json_string<T>(body: T) -> Self
    where
        T: AsRef<[u8]>,
    {
        let body = serde_json::from_slice::<Value>(body.as_ref())
            .expect("Cannot parse field as JSON string");
        Self(Body::Json(body))
    }
}

impl Match for BodyExactMatcher {
    fn matches(&self, request: &Request) -> bool {
        match &self.0 {
            Body::Bytes(bytes) => request.body == *bytes,
            Body::Json(json) => {
                if let Ok(body) = serde_json::from_slice::<Value>(&request.body) {
                    body == *json
                } else {
                    false
                }
            }
        }
    }
}

pub struct BodyContainsMatcher(Vec<u8>);

impl BodyContainsMatcher {
    pub fn string<T>(body: T) -> Self
    where
        T: Into<String>,
    {
        Self(body.into().as_bytes().into())
    }
}

impl Match for BodyContainsMatcher {
    fn matches(&self, request: &Request) -> bool {
        let body = match std::str::from_utf8(&request.body) {
            Ok(body) => body.to_string(),
            Err(e) => {
                debug!("Cannot convert request body to string : {}", e);
                return false;
            }
        };

        let part = match std::str::from_utf8(&self.0) {
            Ok(part) => part,
            Err(e) => {
                debug!("Cannot convert expected body to string : {}", e);
                return false;
            }
        };

        body.contains(part)
    }
}

pub struct BodyPartialJsonMatcher(Value);

impl BodyPartialJsonMatcher {
    pub fn json<T>(body: T) -> Self
    where
        T: Serialize,
    {
        Self(serde_json::to_value(body).expect("Cannot serialize to JSON"))
    }

    pub fn json_string<T>(body: T) -> Self
    where
        T: AsRef<str>,
    {
        Self(serde_json::from_str(body.as_ref()).expect("Cannot deserialize to JSON"))
    }
}

impl Match for BodyPartialJsonMatcher {
    fn matches(&self, request: &Request) -> bool {
        if let Ok(body) = serde_json::from_slice::<Value>(&request.body) {
            let config = assert_json_diff::Config::new(CompareMode::Inclusive);
            // Assert Request JSON includes expected JSON
            assert_json_matches_no_panic(&body, &self.0, config).is_ok()
        } else {
            false
        }
    }
}
