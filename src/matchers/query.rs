use crate::matchers::Match;
use crate::net::request::Request;

pub struct QueryParamExactMatcher(String, String);

impl QueryParamExactMatcher {
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        Self(key.into(), value.into())
    }
}

impl Match for QueryParamExactMatcher {
    fn matches(&self, request: &Request) -> bool {
        request
            .url
            .query_pairs()
            .any(|query| query.0 == self.0.as_str() && query.1 == self.1.as_str())
    }
}

pub struct QueryParamContainsMatcher(String, String);

impl QueryParamContainsMatcher {
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        Self(key.into(), value.into())
    }
}

impl Match for QueryParamContainsMatcher {
    fn matches(&self, request: &Request) -> bool {
        request
            .url
            .query_pairs()
            .any(|query| query.0 == self.0.as_str() && query.1.contains(self.1.as_str()))
    }
}

pub struct QueryParamMissingMatcher(String);

impl QueryParamMissingMatcher {
    pub fn new<K>(key: K) -> Self
    where
        K: Into<String>,
    {
        Self(key.into())
    }
}

impl Match for QueryParamMissingMatcher {
    fn matches(&self, request: &Request) -> bool {
        !request
            .url
            .query_pairs()
            .any(|query| query.0 == self.0.as_str())
    }
}
