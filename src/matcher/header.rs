use crate::core::mock::Match;
use crate::net::request::Request;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use http::{HeaderName, HeaderValue};
use regex::Regex;
use std::collections::HashSet;

pub struct HeaderExactMatcher(HeaderName, Vec<HeaderValue>);

impl HeaderExactMatcher {
    pub fn new<K, V>(key: K, values: Vec<V>) -> Self
    where
        K: TryInto<HeaderName>,
        <K as TryInto<HeaderName>>::Error: std::fmt::Debug,
        V: TryInto<HeaderValue>,
        <V as TryInto<HeaderValue>>::Error: std::fmt::Debug,
    {
        let key = key.try_into().expect("Cannot parse field to header name");
        let values = values
            .into_iter()
            .map(|value| {
                value
                    .try_into()
                    .expect("Cannot parse field to header value")
            })
            .collect();

        Self(key, values)
    }
}

impl Match for HeaderExactMatcher {
    fn matches(&self, request: &Request) -> bool {
        let header_values = request
            .headers
            .get_all(&self.0)
            .iter()
            .filter_map(|val| val.to_str().ok())
            .flat_map(|val| {
                val.split(',')
                    .map(str::trim)
                    .filter_map(|val| HeaderValue::from_str(val).ok())
            })
            .collect::<Vec<_>>();

        header_values == self.1
    }
}

pub struct HeaderExistsMatcher(HeaderName);

impl HeaderExistsMatcher {
    pub fn new<K>(key: K) -> Self
    where
        K: TryInto<HeaderName>,
        <K as TryInto<HeaderName>>::Error: std::fmt::Debug,
    {
        let key = key.try_into().expect("Cannot parse field to header name");
        Self(key)
    }
}

impl Match for HeaderExistsMatcher {
    fn matches(&self, request: &Request) -> bool {
        request.headers.get(&self.0).is_some()
    }
}

pub struct HeaderValueRegexMatcher(HeaderName, Regex);

impl HeaderValueRegexMatcher {
    pub fn new<K>(key: K, value: &str) -> Self
    where
        K: TryInto<HeaderName>,
        <K as TryInto<HeaderName>>::Error: std::fmt::Debug,
    {
        let key = key.try_into().expect("Cannot parse field to header name");
        let regex = Regex::new(value).expect("Cannot parse value into Regex");

        Self(key, regex)
    }
}

impl Match for HeaderValueRegexMatcher {
    fn matches(&self, request: &Request) -> bool {
        let mut value_iterator = request
            .headers
            .get_all(&self.0)
            .iter()
            .filter_map(|value| value.to_str().ok())
            .peekable();

        if value_iterator.peek().is_some() {
            value_iterator.all(|value| self.1.is_match(value))
        } else {
            false
        }
    }
}

pub struct HeaderValueContainsMatcher(HeaderName, HashSet<HeaderValue>);

impl HeaderValueContainsMatcher {
    pub fn new<K, V>(key: K, values: Vec<V>) -> Self
    where
        K: TryInto<HeaderName>,
        <K as TryInto<HeaderName>>::Error: std::fmt::Debug,
        V: TryInto<HeaderValue>,
        <V as TryInto<HeaderValue>>::Error: std::fmt::Debug,
    {
        let key = key.try_into().expect("Cannot parse field to header name");
        let values = values
            .into_iter()
            .map(|value| {
                value
                    .try_into()
                    .expect("Cannot parse field to header value")
            })
            .collect::<HashSet<_>>();

        Self(key, values)
    }
}

impl Match for HeaderValueContainsMatcher {
    fn matches(&self, request: &Request) -> bool {
        request
            .headers
            .get_all(&self.0)
            .iter()
            .filter_map(|val| val.to_str().ok())
            .flat_map(|val| {
                val.split(',')
                    .map(str::trim)
                    .filter_map(|val| HeaderValue::from_str(val).ok())
            })
            .all(|item| self.1.contains(&item))
    }
}

pub struct BasicAuthMatcher(HeaderExactMatcher);

impl BasicAuthMatcher {
    pub fn from_credentials<T, U>(username: T, password: U) -> Self
    where
        T: AsRef<str>,
        U: AsRef<str>,
    {
        Self::from_token(BASE64_STANDARD.encode(format!(
            "{}:{}",
            username.as_ref(),
            password.as_ref()
        )))
    }

    pub fn from_token<T>(token: T) -> Self
    where
        T: AsRef<str>,
    {
        Self(HeaderExactMatcher::new(
            "Authorization",
            vec![&*format!("Basic {}", token.as_ref())],
        ))
    }
}

impl Match for BasicAuthMatcher {
    fn matches(&self, request: &Request) -> bool {
        self.0.matches(request)
    }
}

pub struct BearerTokenMatcher(HeaderExactMatcher);

impl BearerTokenMatcher {
    pub fn from_token<T>(token: T) -> Self
    where
        T: AsRef<str>,
    {
        Self(HeaderExactMatcher::new(
            "Authorization",
            vec![&*format!("Bearer {}", token.as_ref())],
        ))
    }
}

impl Match for BearerTokenMatcher {
    fn matches(&self, request: &Request) -> bool {
        self.0.matches(request)
    }
}
