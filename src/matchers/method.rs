use crate::matchers::Match;
use crate::net::request::Request;
use http::Method;

pub struct MethodMatcher(Method);

impl MethodMatcher {
    pub fn new<T>(method: T) -> Self
    where
        T: TryInto<Method>,
        <T as TryInto<Method>>::Error: std::fmt::Debug,
    {
        let method = method
            .try_into()
            .expect("Failed to convert to HTTP method.");
        Self(method)
    }
}

impl Match for MethodMatcher {
    fn matches(&self, request: &Request) -> bool {
        request.method == self.0
    }
}
