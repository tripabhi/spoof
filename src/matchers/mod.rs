use crate::matchers::body::{BodyContainsMatcher, BodyExactMatcher, BodyPartialJsonMatcher};
use crate::matchers::header::{
    HeaderExactMatcher, HeaderExistsMatcher, HeaderValueContainsMatcher, HeaderValueRegexMatcher,
};
use crate::matchers::method::MethodMatcher;
use crate::matchers::path::{PathExactMatcher, PathRegexMatcher};
use crate::matchers::query::{
    QueryParamContainsMatcher, QueryParamExactMatcher, QueryParamMissingMatcher,
};
use crate::net::request::Request;
use http::{HeaderName, HeaderValue, Method};
use serde::Serialize;

mod body;
mod header;
pub mod method;
mod path;
mod query;

pub trait Match: Send + Sync {
    fn matches(&self, request: &Request) -> bool;
}

impl<F> Match for F
where
    F: Fn(&Request) -> bool,
    F: Send + Sync,
{
    fn matches(&self, request: &Request) -> bool {
        self(request)
    }
}

pub struct AnyMatcher;

impl Match for AnyMatcher {
    fn matches(&self, _: &Request) -> bool {
        true
    }
}

pub fn method<T>(method: T) -> MethodMatcher
where
    T: TryInto<Method>,
    <T as TryInto<Method>>::Error: std::fmt::Debug,
{
    MethodMatcher::new(method)
}

pub fn any() -> AnyMatcher {
    AnyMatcher
}

pub fn path<T>(path: T) -> PathExactMatcher
where
    T: Into<String>,
{
    PathExactMatcher::new(path)
}

pub fn path_regex<T>(path: T) -> PathRegexMatcher
where
    T: Into<String>,
{
    PathRegexMatcher::new(path)
}

pub fn header<K, V>(key: K, value: V) -> HeaderExactMatcher
where
    K: TryInto<HeaderName>,
    <K as TryInto<HeaderName>>::Error: std::fmt::Debug,
    V: TryInto<HeaderValue>,
    <V as TryInto<HeaderValue>>::Error: std::fmt::Debug,
{
    HeaderExactMatcher::new(key, vec![value])
}

pub fn headers<K, V>(key: K, values: Vec<V>) -> HeaderExactMatcher
where
    K: TryInto<HeaderName>,
    <K as TryInto<HeaderName>>::Error: std::fmt::Debug,
    V: TryInto<HeaderValue>,
    <V as TryInto<HeaderValue>>::Error: std::fmt::Debug,
{
    HeaderExactMatcher::new(key, values)
}

pub fn header_exists<K>(key: K) -> HeaderExistsMatcher
where
    K: TryInto<HeaderName>,
    <K as TryInto<HeaderName>>::Error: std::fmt::Debug,
{
    HeaderExistsMatcher::new(key)
}

pub fn header_regex<K>(key: K, value: &str) -> HeaderValueRegexMatcher
where
    K: TryInto<HeaderName>,
    <K as TryInto<HeaderName>>::Error: std::fmt::Debug,
{
    HeaderValueRegexMatcher::new(key, value)
}

pub fn header_contains<K, V>(key: K, values: Vec<V>) -> HeaderValueContainsMatcher
where
    K: TryInto<HeaderName>,
    <K as TryInto<HeaderName>>::Error: std::fmt::Debug,
    V: TryInto<HeaderValue>,
    <V as TryInto<HeaderValue>>::Error: std::fmt::Debug,
{
    HeaderValueContainsMatcher::new(key, values)
}

pub fn body_string<T>(body: T) -> BodyExactMatcher
where
    T: Into<String>,
{
    BodyExactMatcher::string(body)
}

pub fn body_bytes<T>(body: T) -> BodyExactMatcher
where
    T: Into<Vec<u8>>,
{
    BodyExactMatcher::bytes(body)
}

pub fn body_json<T>(body: T) -> BodyExactMatcher
where
    T: Serialize,
{
    BodyExactMatcher::json(body)
}

pub fn body_json_string<T>(body: T) -> BodyExactMatcher
where
    T: AsRef<[u8]>,
{
    BodyExactMatcher::json_string(body)
}

pub fn body_string_contains<T>(body: T) -> BodyContainsMatcher
where
    T: Into<String>,
{
    BodyContainsMatcher::string(body)
}

pub fn body_partial_json<T>(body: T) -> BodyPartialJsonMatcher
where
    T: Serialize,
{
    BodyPartialJsonMatcher::json(body)
}

pub fn body_partial_json_string<T>(body: T) -> BodyPartialJsonMatcher
where
    T: AsRef<str>,
{
    BodyPartialJsonMatcher::json_string(body)
}

pub fn query_param<K, V>(key: K, value: V) -> QueryParamExactMatcher
where
    K: Into<String>,
    V: Into<String>,
{
    QueryParamExactMatcher::new(key, value)
}

pub fn query_param_contains<K, V>(key: K, value: V) -> QueryParamContainsMatcher
where
    K: Into<String>,
    V: Into<String>,
{
    QueryParamContainsMatcher::new(key, value)
}

pub fn query_param_is_missing<K>(key: K) -> QueryParamMissingMatcher
where
    K: Into<String>,
{
    QueryParamMissingMatcher::new(key)
}
