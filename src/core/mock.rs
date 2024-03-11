use crate::net::request::Request;

pub(crate) struct Matcher(Box<dyn Match>);

impl Match for Matcher {
    fn matches(&self, request: &Request) -> bool {
        self.0.matches(request)
    }
}

pub struct StubMappingBuilder {
    pub(crate) matchers: Vec<Matcher>,
}

pub struct Mock {
    pub(crate) matchers: Vec<Matcher>,
}

impl Mock {
    pub fn stub_for<M>(matcher: M) -> StubMappingBuilder
    where
        M: 'static + Match,
    {
        StubMappingBuilder {
            matchers: vec![Matcher(Box::new(matcher))],
        }
    }
}

impl StubMappingBuilder {
    pub fn and<M: Match + 'static>(mut self, matcher: M) -> Self {
        self.matchers.push(Matcher(Box::new(matcher)));
        self
    }
}

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
