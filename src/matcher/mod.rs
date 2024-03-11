use crate::core::mock::Match;
use crate::matcher::body::{BodyContainsMatcher, BodyExactMatcher, BodyPartialJsonMatcher};
use crate::matcher::header::{
    BasicAuthMatcher, BearerTokenMatcher, HeaderExactMatcher, HeaderExistsMatcher,
    HeaderValueContainsMatcher, HeaderValueRegexMatcher,
};
use crate::matcher::method::MethodMatcher;
use crate::matcher::path::{PathExactMatcher, PathRegexMatcher};
use crate::matcher::query::{QueryParamCaseInsensitiveMatcher, QueryParamContainsMatcher, QueryParamExactMatcher, QueryParamExistsMatcher, QueryParamMissingMatcher, QueryParamRegexMatcher};
use crate::net::request::Request;
use http::{HeaderName, HeaderValue, Method};
use serde::Serialize;

pub mod body;
pub mod header;
pub mod method;
pub mod path;
pub mod query;

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

pub fn query_param_case_insensitive<K, V>(key: K, value: V) -> QueryParamCaseInsensitiveMatcher
where
    K: Into<String>,
    V: Into<String>,
{
    QueryParamCaseInsensitiveMatcher::new(key, value)
}

pub fn query_param_regex<K, V>(key: K, value: V) -> QueryParamRegexMatcher
where
    K: Into<String>,
    V: Into<String>,
{
    QueryParamRegexMatcher::new(key, value)
}

pub fn query_param_contains<K, V>(key: K, value: V) -> QueryParamContainsMatcher
where
    K: Into<String>,
    V: Into<String>,
{
    QueryParamContainsMatcher::new(key, value)
}

pub fn query_param_is_missing<K>(key: K) -> QueryParamExistsMatcher
where
    K: Into<String>,
{
    QueryParamExistsMatcher::does_not_exist(key)
}

pub fn query_param_exists<K>(key: K) -> QueryParamExistsMatcher
    where
        K: Into<String>,
{
    QueryParamExistsMatcher::does_exist(key)
}

pub fn query_param_is_missing<K>(key: K) -> QueryParamExistsMatcher
    where
        K: Into<String>,
{
    QueryParamExistsMatcher::does_not_exist(key)
}

pub fn basic_auth<T, U>(username: T, password: U) -> BasicAuthMatcher
where
    T: AsRef<str>,
    U: AsRef<str>,
{
    BasicAuthMatcher::from_credentials(username, password)
}

pub fn bearer_token<T>(token: T) -> BearerTokenMatcher
where
    T: AsRef<str>,
{
    BearerTokenMatcher::from_token(token)
}
