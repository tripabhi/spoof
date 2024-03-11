use crate::core::mock::StubMappingBuilder;
use crate::matcher::path::{PathExactMatcher, PathRegexMatcher};
use crate::matcher::{path, path_regex};
use crate::model::request::MockRegistrable;
use url::Url;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpPathStub {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_equals: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_regex: Option<String>,
}

impl MockRegistrable for HttpPathStub {
    fn register(&self, mut builder: StubMappingBuilder) -> StubMappingBuilder {
        if let Ok(path_matcher) = PathExactMatcher::try_from(self) {
            builder = builder.and(path_matcher)
        } else if let Ok(path_regex_matcher) = PathRegexMatcher::try_from(self) {
            builder = builder.and(path_regex_matcher)
        }
        builder
    }
}

impl TryFrom<&HttpPathStub> for PathExactMatcher {
    type Error = ();

    fn try_from(http_path_stub: &HttpPathStub) -> Result<Self, Self::Error> {
        Url::try_from(http_path_stub).map(|url| path(url.path()))
    }
}

impl TryFrom<&HttpPathStub> for Url {
    type Error = ();

    fn try_from(http_path_stub: &HttpPathStub) -> Result<Self, Self::Error> {
        http_path_stub
            .path_equals
            .as_ref()
            .map(|it| format!("http:://localhost{it}"))
            .map(|it| Url::parse(&it))
            .map(|url| match url {
                Ok(url) => Ok(url),
                Err(_) => Err(()),
            })
            .unwrap_or(Err(()))
    }
}

impl TryFrom<&HttpPathStub> for PathRegexMatcher {
    type Error = ();

    fn try_from(http_path_stub: &HttpPathStub) -> Result<Self, Self::Error> {
        http_path_stub
            .path_regex
            .as_deref()
            .map(path_regex)
            .ok_or_else(|| ())
    }
}
