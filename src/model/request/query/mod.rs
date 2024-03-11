mod absent;
mod case;
mod contains;
mod exact;
mod matches;

use crate::core::mock::StubMappingBuilder;
use crate::matcher::query::{
    QueryParamCaseInsensitiveMatcher, QueryParamContainsMatcher, QueryParamExactMatcher,
    QueryParamExistsMatcher, QueryParamRegexMatcher,
};
use crate::model::request::matcher::GenericMatcherStub;
use crate::model::request::MockRegistrable;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Default, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpQueryParamsStub {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_params: Option<Map<String, Value>>,
}

impl HttpQueryParamsStub {
    fn get_query_params_as_iter(&self) -> Option<impl Iterator<Item = GenericMatcherStub> + '_> {
        self.query_params.as_ref().map(|q| {
            q.iter()
                .filter_map(|it| GenericMatcherStub::try_from(it).ok())
        })
    }
}

impl PartialEq for HttpQueryParamsStub {
    fn eq(&self, other: &Self) -> bool {
        self.query_params.as_ref().eq(&other.query_params.as_ref())
    }
}

impl Hash for HttpQueryParamsStub {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(map) = self.query_params.as_ref() {
            map.iter().for_each(|(key, value)| {
                key.hash(state);
                value.to_string().hash(state);
            })
        }
    }
}

impl MockRegistrable for HttpQueryParamsStub {
    fn register(&self, mut builder: StubMappingBuilder) -> StubMappingBuilder {
        if let Ok(equals_matcher) = Vec::<QueryParamExactMatcher>::try_from(self) {
            for matcher in equals_matcher {
                builder = builder.and(matcher)
            }
        } else if let Ok(case_insensitive_matchers) =
            Vec::<QueryParamCaseInsensitiveMatcher>::try_from(self)
        {
            for matcher in case_insensitive_matchers {
                builder = builder.and(matcher)
            }
        } else if let Ok(contains_matchers) = Vec::<QueryParamContainsMatcher>::try_from(self) {
            for matcher in contains_matchers {
                builder = builder.and(matcher)
            }
        } else if let Ok(regex_matchers) = Vec::<QueryParamRegexMatcher>::try_from(self) {
            for matcher in regex_matchers {
                builder = builder.and(matcher)
            }
        } else if let Ok(exists_matchers) = Vec::<QueryParamExistsMatcher>::try_from(self) {
            for matcher in exists_matchers {
                builder = builder.and(matcher)
            }
        }
        builder
    }
}
