use crate::matchers::Match;
use crate::net::request::Request;
use regex::Regex;
use url::Url;

pub struct PathExactMatcher(String);

impl PathExactMatcher {
    pub fn new<T>(path: T) -> Self
    where
        T: Into<String>,
    {
        let path = path.into();

        if path.contains('?') {
            panic!("Cannot match path - {} (Path contains '?', use spoof::matchers::query_param to match on query parameters)", path);
        }

        if let Ok(url) = Url::parse(&path) {
            if let Some(host) = url.host_str() {
                panic!("Cannot match path - {} (Path contains host - {}; Spoof is aware of the host, try replacing `path(\"{}\")` with `path(\"{}\")`", path, host, path, url.path());
            }
        }

        if path.starts_with('/') {
            Self(path)
        } else {
            Self(format!("/{}", path))
        }
    }
}

impl Match for PathExactMatcher {
    fn matches(&self, request: &Request) -> bool {
        request.url.path() == self.0
    }
}

pub struct PathRegexMatcher(Regex);

impl PathRegexMatcher {
    pub fn new<T>(path: T) -> Self
    where
        T: Into<String>,
    {
        let path = path.into();
        Self(Regex::new(&path).expect("Failed to create regex from path matcher"))
    }
}

impl Match for PathRegexMatcher {
    fn matches(&self, request: &Request) -> bool {
        self.0.is_match(request.url.path())
    }
}
