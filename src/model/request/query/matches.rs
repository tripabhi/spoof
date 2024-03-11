use crate::matcher::query::QueryParamRegexMatcher;
use crate::matcher::{query_param_contains, query_param_regex};
use crate::model::request::matcher::GenericMatcherStub;
use crate::model::request::query::HttpQueryParamsStub;
use itertools::Itertools;

impl TryFrom<&HttpQueryParamsStub> for Vec<QueryParamRegexMatcher> {
    type Error = ();

    fn try_from(http_query_params: &HttpQueryParamsStub) -> Result<Self, Self::Error> {
        http_query_params
            .get_query_params_as_iter()
            .ok_or_else(|| ())
            .map(|iter| {
                iter.filter(|it| it.is_by_regex())
                    .filter_map(|it| QueryParamRegexMatcher::try_from(&it).ok())
                    .collect_vec()
            })
    }
}

impl TryFrom<&GenericMatcherStub> for QueryParamRegexMatcher {
    type Error = ();

    fn try_from(query: &GenericMatcherStub) -> Result<Self, Self::Error> {
        query
            .get_matches_as_str()
            .filter(|_| query.is_by_regex())
            .map(|it| query_param_regex(query.key.to_string(), it))
            .ok_or_else(|| ())
    }
}
