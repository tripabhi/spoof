use crate::core::mock::Match;
use crate::net::request::Request;
use regex::Regex;

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
            .find(|(key, _)| key == self.0.as_str())
            .map(|(_, value)| value == self.1.as_str())
            .unwrap_or_default()
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
            .find(|(key, _)| key == self.0.as_str())
            .map(|(_, value)| value.contains(self.1.as_str()))
            .unwrap_or_default()
    }
}

pub struct QueryParamExistsMatcher(String, bool);

impl QueryParamExistsMatcher {
    pub fn does_not_exist<K>(key: K) -> Self
    where
        K: Into<String>,
    {
        Self(key.into(), false)
    }

    pub fn does_exist<K>(key: K) -> Self
        where
            K: Into<String>,
    {
        Self(key.into(), true)
    }
}

impl Match for QueryParamExistsMatcher {
    fn matches(&self, request: &Request) -> bool {
        let exists = request
            .url
            .query_pairs()
            .any(|query| query.0 == self.0.as_str());
        exists == self.1
    }
}

pub struct QueryParamMissingMatcher(QueryParamExistsMatcher);

impl QueryParamMissingMatcher {
    pub fn new<K>(key: K) -> Self
    where
        K: Into<String>,
    {
        Self(QueryParamExistsMatcher::new(key))
    }
}

impl Match for QueryParamMissingMatcher {
    fn matches(&self, request: &Request) -> bool {
        !self.0.matches(request)
    }
}

pub struct QueryParamCaseInsensitiveMatcher(String, String);

impl QueryParamCaseInsensitiveMatcher {
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        Self(key.into(), value.into())
    }
}

impl Match for QueryParamCaseInsensitiveMatcher {
    fn matches(&self, request: &Request) -> bool {
        request
            .url
            .query_pairs()
            .find(|(key, _)| key == self.0.as_str())
            .map(|(_, value)| value.eq_ignore_ascii_case(self.1.as_str()))
            .unwrap_or_default()
    }
}

pub struct QueryParamRegexMatcher(String, Regex);

impl QueryParamRegexMatcher {
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        let key = key.into();
        let regex = Regex::new(&value.into()).expect("Cannot convert field to regex");
        Self(key, regex)
    }
}

impl Match for QueryParamRegexMatcher {
    fn matches(&self, request: &Request) -> bool {
        request
            .url
            .query_pairs()
            .find(|(key, _)| key == self.0.as_str())
            .map(|(_, value)| self.1.is_match(value.as_ref()))
            .unwrap_or_default()
    }
}
